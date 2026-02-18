use candid::Encode;
use candid::{CandidType, Principal};
use ic_papi_api::PaymentType;
use ic_papi_guard::guards::any::VendorPaymentConfig;
use serde::Deserialize;
use serde_bytes::ByteBuf;

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

/// Arguments for the `call0` function.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct Call0Args {
    /// The principal of the canister to call.
    pub target: Principal,
    /// The name of the method to call.
    pub method: String,
    /// The amount of fee to charge.
    pub fee_amount: u128,
    /// Optional payment configuration (defaults to `AttachedCycles`).
    pub payment: Option<PaymentType>,
    /// Optional cycles to forward to the target canister.
    pub cycles_to_forward: Option<u128>,
}

/// Arguments for the `call_blob` function.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct CallBlobArgs {
    /// The principal of the canister to call.
    pub target: Principal,
    /// The name of the method to call.
    pub method: String,
    /// The Candid-encoded arguments as a byte buffer.
    pub args_blob: ByteBuf,
    /// The amount of fee to charge.
    pub fee_amount: u128,
    /// Optional payment configuration (defaults to `AttachedCycles`).
    pub payment: Option<PaymentType>,
    /// Optional cycles to forward to the target canister.
    pub cycles_to_forward: Option<u128>,
}

/// Arguments for the `call_text` function.
#[derive(Debug, CandidType, Deserialize, Clone, Eq, PartialEq)]
pub struct CallTextArgs {
    /// The principal of the canister to call.
    pub target: Principal,
    /// The name of the method to call.
    pub method: String,
    /// The Candid text representation of the arguments.
    pub args_text: String,
    /// The amount of fee to charge.
    pub fee_amount: u128,
    /// Optional payment configuration (defaults to `AttachedCycles`).
    pub payment: Option<PaymentType>,
    /// Optional cycles to forward to the target canister.
    pub cycles_to_forward: Option<u128>,
}

/// Internal arguments for the bridge call logic.
#[derive(Debug, Clone)]
pub struct BridgeCallArgs {
    pub target: Principal,
    pub method: String,
    pub args: Vec<u8>,
    pub fee_amount: u128,
    pub payment: Option<PaymentType>,
    pub cycles_to_forward: Option<u128>,
}

impl From<Call0Args> for BridgeCallArgs {
    fn from(args: Call0Args) -> Self {
        Self {
            target: args.target,
            method: args.method,
            args: Encode!(&()).expect("Encoding of () should never fail"),
            // .map_err(|e| domain::errors::BridgeError::from(e).to_string())?;
            fee_amount: args.fee_amount,
            payment: args.payment,
            cycles_to_forward: args.cycles_to_forward,
        }
    }
}

impl From<CallBlobArgs> for BridgeCallArgs {
    fn from(args: CallBlobArgs) -> Self {
        Self {
            target: args.target,
            method: args.method,
            args: args.args_blob.into_vec(),
            fee_amount: args.fee_amount,
            payment: args.payment,
            cycles_to_forward: args.cycles_to_forward,
        }
    }
}

impl From<CallTextArgs> for BridgeCallArgs {
    fn from(args: CallTextArgs) -> Self {
        Self {
            target: args.target,
            method: args.method,
            // Note: args_text is not used yet as call_text is disabled
            args: vec![],
            fee_amount: args.fee_amount,
            payment: args.payment,
            cycles_to_forward: args.cycles_to_forward,
        }
    }
}
