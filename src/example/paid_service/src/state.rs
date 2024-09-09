use std::cell::RefCell;

use candid::Principal;
use example_paid_service_api::InitArgs;

thread_local! {
    pub static INIT_ARGS: RefCell<Option<InitArgs>> = RefCell::new(None);
}

pub fn init_element<F, T>(f: F) -> T
where
    F: FnOnce(&InitArgs) -> T,
{
    INIT_ARGS.with(|init_args| f(init_args.borrow().as_ref().expect("No init args provided")))
}

/// Provides own canister ID
pub fn own_canister_id() -> Principal {
    init_element(|init_args| init_args.own_canister_id.clone())
}

/// Provides the canister_id of the ledger used for payments.
pub fn payment_ledger() -> Principal {
    init_element(|init_args| {
        init_args
            .ledger
            .expect("Init args specify no ledger")
            .clone()
    })
}

/// Sets the payment ledger canister ID.
pub fn set_init_args(init_args: InitArgs) {
    INIT_ARGS.set(Some(init_args));
}
