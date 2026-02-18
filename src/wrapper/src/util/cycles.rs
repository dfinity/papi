use candid::Principal;
use ic_cdk::call::Call;
use ic_cdk::call::Response;

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
    Call::bounded_wait(target, method)
        .with_cycles(cycles)
        .with_raw_args(&args)
        .await
        .map(Response::into_bytes)
        .map_err(|e| format!("{e:?}"))
}
