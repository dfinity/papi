mod state;

use std::any::Any;

use example_paid_service_api::InitArgs;
use ic_cdk::init;
use ic_cdk_macros::{export_candid, update};
use ic_papi_api::vendor::PaymentOption;
use ic_papi_api::{principal2account, PaymentError, PaymentType};
use ic_papi_guard::guards::any::AnyPaymentGuard;
use ic_papi_guard::guards::PaymentGuard;
use ic_papi_guard::guards::{
    attached_cycles::AttachedCyclesPayment, icrc2_cycles::Icrc2CyclesPaymentGuard,
};
use state::{payment_ledger, set_init_args};

const SUPPORTED_PAYMENT_OPTIONS: [PaymentOption; 3] = [
    PaymentOption::AttachedCycles { fee: None },
    PaymentOption::CallerPaysIcrc2Cycles { fee: None },
    PaymentOption::PatronPaysIcrc2Cycles { fee: None },
];

const PAYMENT_GUARD: AnyPaymentGuard<3> = AnyPaymentGuard {
    supported: SUPPORTED_PAYMENT_OPTIONS,
};

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
async fn cost_1b_icrc2_from_caller() -> Result<String, PaymentError> {
    let guard = Icrc2CyclesPaymentGuard {
        ledger_canister_id: payment_ledger(),
        ..Icrc2CyclesPaymentGuard::default()
    };
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
        PaymentType::CallerPaysIcrc2Cycles => {
            let guard = Icrc2CyclesPaymentGuard {
                ledger_canister_id: payment_ledger(),
                ..Icrc2CyclesPaymentGuard::default()
            };
            guard.deduct(fee).await?;
        }
        PaymentType::PatronPaysIcrc2Cycles(patron) => {
            let guard = Icrc2CyclesPaymentGuard {
                ledger_canister_id: payment_ledger(),
                payer_account: ic_papi_api::Account {
                    owner: patron,
                    subaccount: None,
                },
                spender_subaccount: Some(principal2account(&ic_cdk::caller())),
                ..Icrc2CyclesPaymentGuard::default()
            };
            guard.deduct(fee).await?;
        }
        _ => {
            return Err(PaymentError::UnsupportedPaymentType {
                supported: SUPPORTED_PAYMENT_OPTIONS.to_vec(),
            })
        }
    };
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

export_candid!();
