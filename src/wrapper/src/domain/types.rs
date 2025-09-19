use candid::Principal;
use ic_papi_guard::guards::any::VendorPaymentConfig;

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub enum FeeDenom {
    Cycles,
    Icrc2 { ledger: Principal },
}

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct FeeSpec {
    pub amount: u128,
    pub denom: FeeDenom,
}

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct MethodConfig {
    pub fee: FeeSpec,
    pub supported: Vec<VendorPaymentConfig>,
    pub forward_cycles: Option<u128>,
}

#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct MethodKey {
    pub target: Principal,
    pub method: String,
}
