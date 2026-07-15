use ic_cdk::api::{is_controller, msg_caller};
use ic_cdk::export_candid;
use ic_cdk::{post_upgrade, pre_upgrade, query, update};

pub mod api;
pub mod domain;
pub mod payments;
pub mod state;
pub mod util;

use crate::api::call::bridge_call;
use crate::domain::types::{
    BridgeCallArgs, Call0Args, CallBlobArgs, CallTextArgs, FeeDenom, MethodConfig, MethodKey,
};

/// Proxies a call to a target method that takes **no arguments**.
#[update]
pub async fn call0(args: Call0Args) -> Result<Vec<u8>, String> {
    bridge_call(args.into()).await
}

/// Proxies a call using a **Candid-encoded argument blob**.
#[update]
pub async fn call_blob(args: CallBlobArgs) -> Result<Vec<u8>, String> {
    bridge_call(args.into()).await
}

/// Proxies a call using **Candid text** (currently disabled).
#[update]
#[allow(clippy::needless_pass_by_value)]
pub fn call_text(args: CallTextArgs) -> Result<Vec<u8>, String> {
    let _args: BridgeCallArgs = args.into();

    Err("call_text is currently disabled due to a workspace dependency conflict with the Candid parser. Please use call_blob instead.".to_string())
}

// --------------------------------------------------------------------------
// Operator configuration (controller-only)
//
// Pricing is server-side: the operator registers, per `(target, method)`, the
// fee to charge and the cycles to forward. Callers can never set these, so they
// cannot make the wrapper forward more cycles than it is paid.
// --------------------------------------------------------------------------

fn ensure_controller() -> Result<(), String> {
    if is_controller(&msg_caller()) {
        Ok(())
    } else {
        Err("Only a canister controller may change the wrapper configuration.".to_string())
    }
}

/// Reject configurations that would let the wrapper forward more cycles than the
/// fee funds. When a method forwards cycles, the fee must be denominated in
/// cycles and be at least the forwarded amount.
fn validate_config(config: &MethodConfig) -> Result<(), String> {
    if let Some(forward) = config.forward_cycles {
        if forward > 0 {
            if config.fee.denom != FeeDenom::Cycles {
                return Err(
                    "A method that forwards cycles must charge its fee in cycles (fee.denom = Cycles)."
                        .to_string(),
                );
            }
            if config.fee.amount < forward {
                return Err(format!(
                    "The fee ({}) must cover the cycles to forward ({forward}).",
                    config.fee.amount
                ));
            }
        }
    }
    Ok(())
}

/// Register or replace the price for a `(target, method)` pair.
#[update]
pub fn set_method_config(key: MethodKey, config: MethodConfig) -> Result<(), String> {
    ensure_controller()?;
    validate_config(&config)?;
    state::set_config(key, config);
    Ok(())
}

/// Remove the price for a `(target, method)` pair, returning any prior value.
#[update]
#[allow(clippy::needless_pass_by_value)]
pub fn remove_method_config(key: MethodKey) -> Result<Option<MethodConfig>, String> {
    ensure_controller()?;
    Ok(state::remove_config(&key))
}

/// Read the price configured for a `(target, method)` pair.
#[query]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn get_method_config(key: MethodKey) -> Option<MethodConfig> {
    state::get_config(&key)
}

/// List every configured `(target, method)` price.
#[query]
#[must_use]
pub fn list_method_configs() -> Vec<(MethodKey, MethodConfig)> {
    state::list_configs()
}

// --------------------------------------------------------------------------
// Upgrade persistence
// --------------------------------------------------------------------------

#[pre_upgrade]
fn pre_upgrade() {
    let configs = state::list_configs();
    ic_cdk::storage::stable_save((configs,)).expect("Failed to persist method configs on upgrade");
}

#[post_upgrade]
fn post_upgrade() {
    match ic_cdk::storage::stable_restore::<(Vec<(MethodKey, MethodConfig)>,)>() {
        Ok((configs,)) => state::replace_all(configs),
        // Do not trap: trapping in `post_upgrade` would make the canister
        // permanently un-upgradable. But a silent failure would bring the
        // wrapper up with an empty registry, causing every call to fail with
        // "No price is configured" and no explanation. Log loudly so operators
        // can diagnose the lost configuration.
        Err(err) => {
            ic_cdk::println!(
                "post_upgrade: failed to restore method configs from stable memory, \
                 starting with an EMPTY registry. All calls will fail until reconfigured. Error: {err}"
            );
        }
    }
}

export_candid!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::{FeeSpec, MethodConfig};

    fn config(fee_amount: u128, denom: FeeDenom, forward: Option<u128>) -> MethodConfig {
        MethodConfig {
            fee: FeeSpec {
                amount: fee_amount,
                denom,
            },
            supported: vec![],
            forward_cycles: forward,
        }
    }

    #[test]
    fn accepts_config_without_forwarding() {
        // Any fee denomination is fine when no cycles are forwarded.
        assert!(validate_config(&config(0, FeeDenom::Cycles, None)).is_ok());
        assert!(validate_config(&config(5, FeeDenom::Cycles, Some(0))).is_ok());
        let ledger = candid::Principal::anonymous();
        assert!(validate_config(&config(5, FeeDenom::Icrc2 { ledger }, None)).is_ok());
    }

    #[test]
    fn accepts_forwarding_covered_by_cycle_fee() {
        assert!(validate_config(&config(1000, FeeDenom::Cycles, Some(1000))).is_ok());
        assert!(validate_config(&config(2000, FeeDenom::Cycles, Some(1000))).is_ok());
    }

    #[test]
    fn rejects_forwarding_with_token_fee() {
        let ledger = candid::Principal::anonymous();
        let err = validate_config(&config(1000, FeeDenom::Icrc2 { ledger }, Some(1000)))
            .expect_err("token-denominated fee cannot fund forwarded cycles");
        assert!(err.contains("cycles"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_forwarding_exceeding_fee() {
        let err = validate_config(&config(999, FeeDenom::Cycles, Some(1000)))
            .expect_err("fee below forwarded amount must be rejected");
        assert!(err.contains("cover"), "unexpected error: {err}");
    }
}
