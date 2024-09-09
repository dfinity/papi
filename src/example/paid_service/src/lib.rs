mod state;

use candid::Principal;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::{Account, PaymentError};
use ic_papi_guard::guards::attached_cycles::AttachedCyclesPayment;
use ic_papi_guard::guards::icrc2_from_caller::Icrc2FromCaller;
use ic_papi_guard::guards::PaymentGuard;
pub use state::InitArgs;
use state::{payment_ledger, set_init_args};

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

/// An API method that requires cycles to be provided by the user using an ICRC-2 approve.
#[update()]
async fn cost_1000_icrc2_from_caller() -> Result<String, PaymentError> {
    let guard = Icrc2FromCaller {
        payer: Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        },
        ledger_canister_id: payment_ledger(),
    };
    guard.deduct(1000).await?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}

export_candid!();
