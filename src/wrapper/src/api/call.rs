use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::domain::types::BridgeCallArgs;
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Internal helper to unify the bridge logic: charge fee -> forward call.
// TODO: The caller may have to provide more type information than they are used to. Normally dfx will use the target canister's candid file to convert to the correct types; without that information it will guess more simply and won't always get this conversion right.
pub async fn bridge_call(args: BridgeCallArgs) -> Result<Vec<u8>, String> {
    if args.target == ic_cdk::api::canister_self() {
        return Err("Self-calls are not allowed through the bridge.".to_string());
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
