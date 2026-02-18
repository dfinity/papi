use candid::{Encode, Principal};
use ic_cdk::export_candid;
use ic_cdk::update;
use ic_papi_api::PaymentType;

pub mod api;
pub mod domain;
pub mod payments;
pub mod util;

/// Proxies a call to a target method that takes **no arguments** (`()`), after charging a fee.
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
    let args = Encode!().map_err(|e| domain::errors::BridgeError::from(e).to_string())?;

    api::call::bridge_call(target, method, args, fee_amount, payment, cycles_to_forward).await
}

/// Proxies a call using a **Candid-encoded argument blob**, after charging a fee.
///
/// Use this when your client already encoded args with `IDL.encode` (agent-js)
/// or `candid::Encode!` (Rust).
#[update]
pub async fn call_blob(
    target: Principal,
    method: String,
    args_blob: Vec<u8>,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    api::call::bridge_call(
        target,
        method,
        args_blob,
        fee_amount,
        payment,
        cycles_to_forward,
    )
    .await
}

/// Proxies a call using **Candid text** (e.g., `"(\"hello\", 42, opt null)"`), after charging a fee.
///
/// This is convenient when you want to pass args “like in a .did example” without
/// writing encoding code on the client.
#[update]
pub fn call_text(
    target: Principal,
    method: String,
    args_text: String,
    fee_amount: u128,
    payment: Option<PaymentType>,
    cycles_to_forward: Option<u128>,
) -> Result<Vec<u8>, String> {
    let _ = (
        target,
        method,
        args_text,
        fee_amount,
        payment,
        cycles_to_forward,
    );

    Err("call_text is currently disabled because of a workspace dependency conflict with the Candid parser. Please use call_blob instead.".to_string())
}

export_candid!();
