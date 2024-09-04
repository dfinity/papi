use ic_cdk_macros::update;
use ic_cdk::api::call::call_with_payment128;
use ic_papi_api::PaymentError;
use candid::Principal;

/// Calls an arbitrary method on an arbitrary canister with an arbitrary amount of cycles attached.
#[update()]
async fn call_with_attached_cycles(cycles: u128, canister_id: Principal, method: String) -> Result<(), PaymentError> {
    let arg = ();
    let (ans, ): (Result<(), PaymentError>,) = call_with_payment128(canister_id, &method, arg, cycles).await.unwrap();
    ans
}
