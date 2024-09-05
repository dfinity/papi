use candid::Principal;
use ic_cdk::api::call::call_with_payment128;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::PaymentError;

/// Calls an arbitrary method on an arbitrary canister with an arbitrary amount of cycles attached.
#[update()]
async fn call_with_attached_cycles(
    call_params: (Principal, String, u64),
) -> Result<String, PaymentError> {
    let (canister_id, method, cycles) = call_params;
    let arg = ();
    let (ans,): (Result<String, PaymentError>,) =
        call_with_payment128(canister_id, &method, arg, u128::from(cycles))
            .await
            .unwrap();
    ans
}

export_candid!();
