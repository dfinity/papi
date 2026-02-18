//! Public, stateless API for payment-wrapped proxy calls.
//!
//! You choose *how* to pass arguments:
//!  - `call0`   : target takes no args → we send `()`
//!  - `call_blob`: you provide a Candid-encoded arg blob
//!  - `call_text`: you provide Candid text like `("(42, \"hi\")")`
//!
//! In all cases you also pass:
//!  - `fee_amount` (u128, in the unit implied by your `PaymentType` variant)
//!  - `payment`   (which variant determines cycles vs token, patron vs caller)
//!  - optional `cycles_to_forward` (for targets that expect cycles)

use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::domain::types::BridgeCallArgs;
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Display>(e: E) -> String {
    BridgeError::GuardError(e.to_string()).to_string()
}

/// Internal helper to unify the bridge logic: charge fee -> forward call.
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
