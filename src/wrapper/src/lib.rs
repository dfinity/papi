use candid::Encode;
use domain::types::*;
use ic_cdk::export_candid;
use ic_cdk::update;

pub mod api;
pub mod domain;
pub mod payments;
pub mod util;

use crate::api::call::bridge_call;

/// Proxies a call to a target method that takes **no arguments** (`()`), after charging a fee.
///
/// # Arguments
/// * `args` - A [`Call0Args`] struct containing:
///     * `target`: The principal of the canister to call.
///     * `method`: The name of the method to call.
///     * `fee_amount`: The amount of fee to charge.
///     * `payment`: Optional payment configuration (defaults to `AttachedCycles`).
///     * `cycles_to_forward`: Optional cycles to forward to the target canister.
///
/// # Returns
/// Raw Candid reply blob from the target (decode on the client).
#[update]
pub async fn call0(args: Call0Args) -> Result<Vec<u8>, String> {
    let call_args = Encode!().map_err(|e| domain::errors::BridgeError::from(e).to_string())?;

    bridge_call(
        args.target,
        args.method,
        call_args,
        args.fee_amount,
        args.payment,
        args.cycles_to_forward,
    )
    .await
}

/// Proxies a call using a **Candid-encoded argument blob**, after charging a fee.
///
/// Use this when your client already encoded args with `IDL.encode` (agent-js)
/// or `candid::Encode!` (Rust).
///
/// # Arguments
/// * `args` - A [`CallBlobArgs`] struct containing:
///     * `target`: The principal of the canister to call.
///     * `method`: The name of the method to call.
///     * `args_blob`: The Candid-encoded arguments as a byte buffer.
///     * `fee_amount`: The amount of fee to charge.
///     * `payment`: Optional payment configuration (defaults to `AttachedCycles`).
///     * `cycles_to_forward`: Optional cycles to forward to the target canister.
#[update]
pub async fn call_blob(args: CallBlobArgs) -> Result<Vec<u8>, String> {
    bridge_call(
        args.target,
        args.method,
        args.args_blob.into_vec(),
        args.fee_amount,
        args.payment,
        args.cycles_to_forward,
    )
    .await
}

/// Proxies a call using **Candid text** (e.g., `"(\"hello\", 42, opt null)"`), after charging a fee.
///
/// This is convenient when you want to pass args “like in a .did example” without
/// writing encoding code on the client.
///
/// # Arguments
/// * `args` - A [`CallTextArgs`] struct containing:
///     * `target`: The principal of the canister to call.
///     * `method`: The name of the method to call.
///     * `args_text`: The Candid text representation of the arguments.
///     * `fee_amount`: The amount of fee to charge.
///     * `payment`: Optional payment configuration (defaults to `AttachedCycles`).
///     * `cycles_to_forward`: Optional cycles to forward to the target canister.
///
/// # Errors
/// This method is currently disabled due to a workspace dependency conflict with the Candid parser. Please use `call_blob` with pre-encoded arguments instead.
#[update]
pub fn call_text(args: CallTextArgs) -> Result<Vec<u8>, String> {
    let _ = args;

    Err("call_text is currently disabled because of a workspace dependency conflict with the Candid parser. Please use call_blob instead.".to_string())
}

export_candid!();
