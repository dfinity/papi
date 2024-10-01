use candid::Principal;
use ic_cdk::api::{call::call_with_payment128, is_controller};
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
        is_controller(&ic_cdk::caller()),
        "The caller must be a controller."
    );
    let (canister_id, method, cycles) = call_params;
    let arg = ();
    let (ans,): (Result<String, PaymentError>,) =
        call_with_payment128(canister_id, &method, arg, u128::from(cycles))
            .await
            .unwrap();
    ans
}

export_candid!();
