use ic_cdk_macros::update;
use ic_papi_api::PaymentError;
use ic_papi_guard::guards::attached_cycles::AttachedCyclesPayment;
use ic_papi_guard::guards::PaymentGuard;

#[update()]
async fn free() -> String {
    "Yes, I am free!".to_string()
}

/// An API method that requires cycles to be attached directly to the call.
#[update()]
async fn cost_1000_cycles() -> Result<String, PaymentError> {
    AttachedCyclesPayment::deduct(1000)?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}
