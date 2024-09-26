mod state;

use example_paid_service_api::InitArgs;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::cycles::cycles_ledger_canister_id;
use ic_papi_api::{PaymentError, PaymentType};
use ic_papi_guard::guards::{
    attached_cycles::AttachedCyclesPayment, icrc2_cycles::Icrc2CyclesPaymentGuard,
    icrc2_tokens::CallerPaysIcrc2TokensPaymentGuard,
};
use ic_papi_guard::guards::{PaymentContext, PaymentGuard, PaymentGuard2};
use state::{set_init_args, PAYMENT_GUARD};

#[init]
fn init(init_args: Option<InitArgs>) {
    if let Some(init_args) = init_args {
        set_init_args(init_args);
    }
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
async fn caller_pays_1b_icrc2_cycles() -> Result<String, PaymentError> {
    Icrc2CyclesPaymentGuard::default()
        .deduct(1_000_000_000)
        .await?;
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

/// An API method that requires 1 billion tokens (in this case cycles) using an ICRC-2 approve with default parameters.
///
/// The tokens will be transferred to the vendor's main account on the ledger.
#[update()]
async fn caller_pays_1b_icrc2_tokens() -> Result<String, PaymentError> {
    CallerPaysIcrc2TokensPaymentGuard {
        ledger: cycles_ledger_canister_id(),
    }
    .deduct(1_000_000_000)
    .await?;
    Ok("Yes, you paid 1 billion tokens!".to_string())
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
