mod state;

use example_paid_service_api::InitArgs;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::{Account, PaymentError};
use ic_papi_guard::guards::attached_cycles::AttachedCyclesPayment;
use ic_papi_guard::guards::icrc2_from_caller::Icrc2FromCaller;
use ic_papi_guard::guards::PaymentGuard;
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

/// An API method that requires 1 billion cycles to be provided by the user using an ICRC-2 approve.
#[update()]
async fn cost_1b_icrc2_from_caller() -> Result<String, PaymentError> {
    let guard = Icrc2FromCaller {
        own_canister_id: own_canister_id(),
        payer: Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        },
        ledger_canister_id: payment_ledger(),
    };
    guard.deduct(1_000_000_000).await?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}

export_candid!();
