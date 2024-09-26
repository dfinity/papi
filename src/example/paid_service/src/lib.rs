mod state;

use example_paid_service_api::InitArgs;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update, query};
use ic_papi_api::{PaymentError, PaymentType};
use ic_papi_guard::guards::any::PaymentWithConfig;
use ic_papi_guard::guards::{
    attached_cycles::AttachedCyclesPayment, icrc2_cycles::Icrc2CyclesPaymentGuard,
};
use ic_papi_guard::guards::{PaymentContext, PaymentGuard, PaymentGuard2};
use state::{set_init_args, PAYMENT_GUARD};

#[init]
fn init(init_args: Option<InitArgs>) {
    if let Some(init_args) = init_args {
        set_init_args(init_args);
    }
}

/// An API method that returns the current payment configuration for a given payment type.
/// 
/// This is used in tests to verify that the payment configuration is set correctly.
#[query()]
fn payment_config() -> Option<PaymentWithConfig> {
    PAYMENT_GUARD.config(PaymentType::CallerPaysIcrc2Cycles)
}

#[update()]
fn free() -> String {
    "Yes, I am free!".to_string()
}

/// An API method that requires cycles to be attached directly to the call.
#[update()]
async fn cost_1000_attached_cycles() -> Result<String, PaymentError> {
    AttachedCyclesPayment::default().deduct(1000).await?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}

/// An API method that requires 1 billion cycles using an ICRC-2 approve with default parameters.
#[update()]
async fn cost_1b_icrc2_from_caller() -> Result<String, PaymentError> {
    Icrc2CyclesPaymentGuard::default().deduct(1_000_000_000).await?;
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

/// An API method that requires 1 billion cycles, paid in whatever way the client chooses.
#[update()]
async fn cost_1b(payment: PaymentType) -> Result<String, PaymentError> {
    let fee = 1_000_000_000;
    PAYMENT_GUARD
        .deduct(PaymentContext::default(), payment, fee)
        .await?;
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

export_candid!();
