use candid::{CandidType, Deserialize, Principal};

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub own_canister_id: Principal,
    pub ledger: Option<Principal>,
}
