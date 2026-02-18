use candid::Principal;
use ic_cdk::api::call::{call_raw, call_raw128};
use ic_cdk::call::Call;

/// Forwards a raw candid call to the target canister, optionally attaching cycles.
///
/// # Errors
/// Returns a string with reject code/message if the target rejects.
pub async fn forward_raw(
    target: Principal,
    method: &str,
    args: Vec<u8>,
    cycles: u128,
) -> Result<Vec<u8>, String> {
   Call::bounded_wait(target, method, args, cycles)
        .await
        .map_err(|(code, msg)| format!("{code:?} {msg}"))
}
