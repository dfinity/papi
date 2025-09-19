use candid::Principal;
use ic_cdk::api::call::{call_raw, call_raw128};

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
    if cycles > 0 {
        call_raw128(target, method, args, cycles)
            .await
            .map_err(|(code, msg)| format!("{code:?} {msg}"))
    } else {
        call_raw(target, method, args, 0)
            .await
            .map_err(|(code, msg)| format!("{code:?} {msg}"))
    }
}
