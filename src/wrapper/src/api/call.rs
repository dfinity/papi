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

use candid::{Encode, Principal};
use ic_cdk::update;
use ic_papi_api::PaymentType;

use crate::domain::BridgeError;
use crate::payments::PAYMENT_GUARD;
use crate::util::cycles::forward_raw;

fn map_guard_err<E: core::fmt::Debug>(e: E) -> String {
    BridgeError::GuardError(format!("{e:?}")).to_string()
}

/// Proxies a call to a target method that takes **no arguments** (`()`), after charging a fee.
///
/// # Arguments
/// - `target`: Target canister principal.
/// - `method`: Exact method name on the target.
/// - `fee_amount`: Amount to charge via `PAYMENT_GUARD`.
/// - `payment`: Payment type to use. If `None`, defaults to `AttachedCycles`.
/// - `cycles_to_forward`: Optional cycles to attach to the target call.
///
/// # Returns
/// Raw Candid reply blob from the target (decode on the client).
#[update]
pub async fn call0(
    target: Principal,
    method: String,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    let p = payment.unwrap_or(PaymentType::AttachedCycles);
    PAYMENT_GUARD.deduct(p, fee_amount).await.map_err(map_guard_err)?;

    let args = Encode!().map_err(|e| BridgeError::Candid(e.to_string()).to_string())?;

    let cycles = cycles_to_forward.unwrap_or(0);

    forward_raw(target, &method, args, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}

/// Proxies a call using a **Candid-encoded argument blob**, after charging a fee.
///
/// Use this when your client already encoded args with `IDL.encode` (agent-js)
/// or `candid::Encode!` (Rust).
///
/// # Arguments
/// - `args_blob`: The exact Candid byte payload to forward to the target.
#[update]
pub async fn call_blob(
    target: Principal,
    method: String,
    args_blob: Vec<u8>,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    // 1) charge fee
    let p = payment.unwrap_or(PaymentType::AttachedCycles);
    PAYMENT_GUARD.deduct(p, fee_amount).await.map_err(map_guard_err)?;

    // 2) forward as-is
    let cycles = cycles_to_forward.unwrap_or(0);
    forward_raw(target, &method, args_blob, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}

/// Proxies a call using **Candid text** (e.g., `"(\"hello\", 42, opt null)"`), after charging a fee.
///
/// This is convenient when you want to pass args “like in a .did example” without
/// writing encoding code on the client.
///
/// # Arguments
/// - `args_text`: A string containing Candid values for the target method’s parameter list.
#[update]
pub async fn call_text(
    target: Principal,
    method: String,
    args_text: String,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    use candid::parser::value::IDLArgs;

    // 1) charge fee
    let p = payment.unwrap_or(PaymentType::AttachedCycles);
    PAYMENT_GUARD.deduct(p, fee_amount).await.map_err(map_guard_err)?;

    // 2) parse candid text -> blob
    let idl_args = IDLArgs::from_str(&args_text)
        .map_err(|e| BridgeError::Candid(e.to_string()).to_string())?;
    let args_blob = idl_args
        .to_bytes()
        .map_err(|e| BridgeError::Candid(e.to_string()).to_string())?;

    // 3) forward
    let cycles = cycles_to_forward.unwrap_or(0);
    forward_raw(target, &method, args_blob, cycles)
        .await
        .map_err(|e| BridgeError::TargetRejected(e).to_string())
}
