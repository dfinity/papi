use candid::Principal;
use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::domain::types::BridgeCallArgs;
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

/// The IC management canister principal (`aaaaa-aa`).
///
/// Forwarding to it would let any caller make inter-canister calls that are
/// authorized as *this* canister, e.g. lifecycle operations against any
/// canister the bridge controls.
///
/// Note that not every management-canister method is controller-gated: some
/// (e.g. `raw_rand`, threshold `sign_with_ecdsa`/`sign_with_schnorr`, the
/// Bitcoin API) could be legitimate paid targets. Even so, the bridge is
/// currently a target *denylist* (it forbids only itself and `aaaaa-aa`), so
/// exposing individual safe methods must not be done by unblocking the whole
/// principal — the growing management API makes that too easy to get wrong.
/// Instead, opt specific methods in via the `MethodConfig`/`MethodKey`
/// allowlist in `domain::types`. Until that allowlist is wired up, block
/// `aaaaa-aa` outright.
const MANAGEMENT_CANISTER_ID: Principal = Principal::management_canister();

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Internal helper to unify the bridge logic: charge fee -> forward call.
// TODO: The caller may have to provide more type information than they are used to. Normally dfx will use the target canister's candid file to convert to the correct types; without that information it will guess more simply and won't always get this conversion right.
pub async fn bridge_call(args: BridgeCallArgs) -> Result<Vec<u8>, String> {
    if args.target == ic_cdk::api::canister_self() {
        return Err("Self-calls are not allowed through the bridge.".to_string());
    }

    // The bridge must never be usable as a proxy to the management canister:
    // such calls would execute with the bridge's own principal as the caller.
    if args.target == MANAGEMENT_CANISTER_ID {
        return Err(BridgeError::ForbiddenTarget(
            "the management canister may not be reached through the bridge.".to_string(),
        )
        .to_string());
    }

    // 1) Charge fee using the payment guard
    let p = args.payment.unwrap_or(PaymentType::AttachedCycles);
    PAYMENT_GUARD
        .deduct(p, args.fee_amount)
        .await
        .map_err(map_guard_err)?;

    // 2) Forward the call to target
    let cycles = args.cycles_to_forward.unwrap_or(0);
    forward_raw(args.target, &args.method, args.args, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}
