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

use candid::Principal;
use ic_papi_api::PaymentType;

use crate::domain::errors::BridgeError;
use crate::payments::guard_config::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Internal helper to unify the bridge logic: charge fee -> forward call.
pub async fn bridge_call(
    target: Principal,
    method: String,
    args: Vec<u8>,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    // 1) Charge fee using the payment guard
    let p = payment.unwrap_or(PaymentType::AttachedCycles);
    PAYMENT_GUARD
        .deduct(p, fee_amount)
        .await
        .map_err(map_guard_err)?;

    // 2) Forward the call to target
    let cycles = cycles_to_forward.unwrap_or(0);
    forward_raw(target, &method, args, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}
