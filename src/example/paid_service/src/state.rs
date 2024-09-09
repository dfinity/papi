use std::cell::RefCell;

use candid::{CandidType, Deserialize, Principal};

thread_local! {
    pub static INIT_ARGS: RefCell<Option<InitArgs>> = RefCell::new(None);
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub own_canister_id: Principal,
    pub ledger: Option<Principal>,
}

pub fn init_element<F, T>(f: F) -> T
where
    F: FnOnce(&InitArgs) -> T,
{
    INIT_ARGS.with(|init_args| f(init_args.borrow().as_ref().expect("foomsg")))
}

/// Provides the canister_id of the ledger used for payments.
pub fn payment_ledger() -> Principal {
    init_element(|init_args| {
        init_args
            .ledger
            .clone()
            .expect("Init args specify no ledger")
    })
}

/// Sets the payment ledger canister ID.
pub fn set_init_args(init_args: InitArgs) {
    INIT_ARGS.set(Some(init_args));
}
