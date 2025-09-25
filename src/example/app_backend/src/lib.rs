use candid::Principal;
use ic_cdk::api::{is_controller, msg_caller};
use ic_cdk::call::Call;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::PaymentError;

/// Calls an arbitrary method on an arbitrary canister with an arbitrary amount of cycles attached.
///
/// Note: This is for demonstration purposes only.  To avoid cycle theft, the API may be aclled by a controller only.
#[update()]
async fn call_with_attached_cycles(
    call_params: (Principal, String, u64),
) -> Result<String, PaymentError> {
    assert!(
        is_controller(&msg_caller()),
        "The caller must be a controller."
    );
    let (canister_id, method, cycles) = call_params;
    let arg = ();
    let response = Call::unbounded_wait(canister_id, &method)
        .with_cycles(u128::from(cycles))
        .with_arg(arg)
        .await
        .unwrap();
    let (ans,): (Result<String, PaymentError>,) = response.candid().unwrap();
    ans
}

export_candid!();
