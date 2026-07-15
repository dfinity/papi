use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::domain::types::{BridgeCallArgs, MethodKey};
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::state;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Internal helper to unify the bridge logic: look up price -> charge fee -> forward call.
///
/// The fee and the number of cycles to forward are taken from the
/// operator-configured [`crate::domain::types::MethodConfig`] for the
/// `(target, method)` pair — never from the caller. This prevents a caller from
/// naming a trivial fee while forwarding a large amount, which would drain the
/// wrapper's own cycle balance.
// TODO: The caller may have to provide more type information than they are used to. Normally dfx will use the target canister's candid file to convert to the correct types; without that information it will guess more simply and won't always get this conversion right.
pub async fn bridge_call(args: BridgeCallArgs) -> Result<Vec<u8>, String> {
    if args.target == ic_cdk::api::canister_self() {
        return Err("Self-calls are not allowed through the bridge.".to_string());
    }

    // Look up the operator-configured price for this `(target, method)`.
    let key = MethodKey {
        target: args.target,
        method: args.method.clone(),
    };
    let config = state::get_config(&key).ok_or_else(|| {
        BridgeError::MethodNotConfigured {
            target: args.target.to_string(),
            method: args.method.clone(),
        }
        .to_string()
    })?;

    let p = args.payment.unwrap_or(PaymentType::AttachedCycles);
    let cycles = config.forward_cycles.unwrap_or(0);

    // If this method forwards cycles, the caller must pay in cycles so that the
    // fee actually credits the wrapper's cycle balance. `set_method_config`
    // already guarantees `fee.denom == Cycles` and `fee.amount >= forward_cycles`
    // for such methods; here we additionally ensure the *caller's chosen* payment
    // type is cycle-denominated (token payments credit a token account, not cycles).
    if cycles > 0 && !is_cycle_payment(&p) {
        return Err(BridgeError::ForwardRequiresCyclePayment.to_string());
    }

    // 1) Charge the operator-set fee.
    PAYMENT_GUARD
        .deduct(p, config.fee.amount)
        .await
        .map_err(map_guard_err)?;

    // 2) Forward the call with the operator-set cycles.
    forward_raw(args.target, &args.method, args.args, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}

/// Whether a payment type credits this canister's *cycle* balance (as opposed to
/// a token ledger account), and can therefore fund forwarded cycles.
fn is_cycle_payment(payment: &PaymentType) -> bool {
    matches!(
        payment,
        PaymentType::AttachedCycles
            | PaymentType::CallerPaysIcrc2Cycles
            | PaymentType::PatronPaysIcrc2Cycles(_)
    )
}
