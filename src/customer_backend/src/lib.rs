use ic_cdk_macros::update;
use ic_cdk::api::call::call_with_payment128;
use ic_papi_api::PaymentError;
use candid::Principal;

/// Calls an arbitrary method on an arbitrary canister with an arbitrary amount of cycles attached.
#[update()]
async fn call_with_attached_cycles(canister_id: Principal) -> Result<String, PaymentError> {
    let cycles = 20000;
    let  method = "cost_1000_cycles".to_string();
    let arg = ();
    let (ans, ): (Result<String, PaymentError>,) = call_with_payment128(canister_id, &method, arg, cycles).await.unwrap();
    ans
}
/*
async fn call_with_attached_cycles(cycles: u128, canister_id: Principal, method: String) -> Result<(), PaymentError> {
    let arg = ();
    let (ans, ): (Result<(), PaymentError>,) = call_with_payment128(canister_id, &method, arg, cycles).await.unwrap();
    ans
}
     */