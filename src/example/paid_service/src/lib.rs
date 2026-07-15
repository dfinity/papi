mod state;

use example_paid_service_api::InitArgs;
use ic_cdk::{export_candid, init, post_upgrade, pre_upgrade, update};
use ic_papi_api::cycles::cycles_ledger_canister_id;
use ic_papi_api::{PaymentError, PaymentType};
use ic_papi_guard::guards::PaymentGuardTrait;
use ic_papi_guard::guards::{
    attached_cycles::AttachedCyclesPayment,
    caller_pays_icrc2_cycles::CallerPaysIcrc2CyclesPaymentGuard,
    caller_pays_icrc2_tokens::CallerPaysIcrc2TokensPaymentGuard,
};
use state::{get_init_args, set_init_args, PAYMENT_GUARD};

#[init]
fn init(init_args: Option<InitArgs>) {
    if let Some(init_args) = init_args {
        set_init_args(init_args);
    }
}

/// Persists the init args to stable memory before a canister upgrade.
///
/// The init args are held in a non-stable `thread_local` (see `state::INIT_ARGS`), which the IC
/// wipes on upgrade. Without this hook there is no `post_upgrade` counterpart to restore them, so
/// after any upgrade `payment_ledger()` — and therefore `cost_1b` — would trap with
/// "No init args provided".
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::storage::stable_save((get_init_args(),))
        .expect("Failed to save init args to stable memory");
}

/// Restores the init args from stable memory after a canister upgrade.
#[post_upgrade]
fn post_upgrade() {
    let (init_args,): (Option<InitArgs>,) =
        ic_cdk::storage::stable_restore().expect("Failed to restore init args from stable memory");
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
    CallerPaysIcrc2CyclesPaymentGuard::default()
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
    PAYMENT_GUARD.deduct(payment, fee).await?;
    Ok("Yes, you paid 1 billion cycles!".to_string())
}

export_candid!();
