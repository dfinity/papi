use candid::Principal;
use ic_cdk::{api::is_controller, call::Call};
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::PaymentError;

/// Calls an arbitrary method on an arbitrary canister with an arbitrary amount of cycles attached.
///
/// Note: This is for demonstration purposes only.  To avoid cycle theft, the API may be called by a controller only.
#[update()]
async fn call_with_attached_cycles(
    call_params: (Principal, String, u128),
) -> Result<String, PaymentError> {
    assert!(
        is_controller(&ic_cdk::api::msg_caller()),
        "The caller must be a controller."
    );
    let (canister_id, method, cycles) = call_params;
    Call::unbounded_wait(canister_id, &method)
        .with_cycles(cycles)
        .await
        .unwrap()
        .candid::<Result<String, PaymentError>>()
        .unwrap()
}

export_candid!();
