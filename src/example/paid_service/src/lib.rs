mod state;

use example_paid_service_api::InitArgs;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::{PaymentError, PaymentType};
use ic_papi_guard::guards::PaymentGuard;
use ic_papi_guard::guards::{
    attached_cycles::AttachedCyclesPayment, icrc2_cycles::Icrc2CyclesPaymentGuard,
};
use state::{own_canister_id, payment_ledger, set_init_args};

#[init]
fn init(init_args: Option<InitArgs>) {
    if let Some(init_args) = init_args {
        set_init_args(init_args);
    }
}

#[update()]
async fn free() -> String {
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
    let mut guard = Icrc2CyclesPaymentGuard::new(own_canister_id());
    guard.ledger_canister_id = payment_ledger();
    guard.deduct(1_000_000_000).await?;
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

/// An API method that requires 1 billion cycles using an ICRC-2 approve with default parameters.
#[update()]
async fn cost_1b(payment: PaymentType) -> Result<String, PaymentError> {
    let fee = 1_000_000_000;
    match payment {
        PaymentType::AttachedCycles => {
            AttachedCyclesPayment::default().deduct(fee).await?;
        }
        PaymentType::Icrc2 => {
            let mut guard = Icrc2CyclesPaymentGuard::new(own_canister_id());
            guard.ledger_canister_id = payment_ledger();
            guard.deduct(fee).await?;
        }
    };
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

export_candid!();
