use candid::{CandidType, Deserialize, Principal};

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct InitArgs {
    pub ledger: Principal,
}
