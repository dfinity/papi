use candid::Principal;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::PaymentError;
use ic_papi_guard::guards::attached_cycles::AttachedCyclesPayment;
use ic_papi_guard::guards::icrc2_from_caller::Icrc2FromCaller;
use ic_papi_guard::guards::PaymentGuard;

#[update()]
async fn free() -> String {
    "Yes, I am free!".to_string()
}

/// An API method that requires cycles to be attached directly to the call.
#[update()]
async fn cost_1000_attached_cycles() -> Result<String, PaymentError> {
    AttachedCyclesPayment::default().deduct(1000)?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}

/// An API method that requires cycles to be provided by the user using an ICRC-2 approve.
#[update()]
async fn cost_1000_icrc2_from_caller() -> Result<String, PaymentError> {
    let guard = Icrc2FromCaller {
        payer: ic_papi_api::Account {
            owner: ic_cdk::caller(),
            subaccount: None,
        },
        ledger_canister_id: Principal::from_text(
            option_env!("CANISTER_ID_CYCLES_LEDGER")
                .expect("CANISTER_ID_CYCLES_LEDGER was not set at compile time"),
        )
        .unwrap(),
    };
    guard.deduct(1000)?;
    Ok("Yes, you paid 1000 cycles!".to_string())
}

export_candid!();
