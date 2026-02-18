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
            args: Encode!(&()).unwrap(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Encode, Principal};

    #[test]
    fn test_call0_to_bridge_args() {
        let args = Call0Args {
            target: Principal::anonymous(),
            method: "test".to_string(),
            fee_amount: 100,
            payment: Some(PaymentType::AttachedCycles),
            cycles_to_forward: Some(50),
        };
        let bridge_args: BridgeCallArgs = args.clone().into();
        assert_eq!(bridge_args.target, args.target);
        assert_eq!(bridge_args.method, args.method);
        assert_eq!(bridge_args.fee_amount, args.fee_amount);
        assert_eq!(bridge_args.payment, args.payment);
        assert_eq!(bridge_args.cycles_to_forward, args.cycles_to_forward);
        // call0 should encode unit ()
        assert_eq!(bridge_args.args, Encode!(&()).unwrap());
    }

    #[test]
    fn test_call_blob_to_bridge_args() {
        let blob = vec![1, 2, 3];
        let args = CallBlobArgs {
            target: Principal::anonymous(),
            method: "test_blob".to_string(),
            args_blob: ByteBuf::from(blob.clone()),
            fee_amount: 200,
            payment: None,
            cycles_to_forward: None,
        };
        let bridge_args: BridgeCallArgs = args.clone().into();
        assert_eq!(bridge_args.target, args.target);
        assert_eq!(bridge_args.method, args.method);
        assert_eq!(bridge_args.fee_amount, 200);
        assert_eq!(bridge_args.payment, None);
        assert_eq!(bridge_args.cycles_to_forward, None);
        assert_eq!(bridge_args.args, blob);
    }

    #[test]
    fn test_call_text_to_bridge_args() {
        let args = CallTextArgs {
            target: Principal::anonymous(),
            method: "test_text".to_string(),
            args_text: "(record { x = 42 })".to_string(),
            fee_amount: 300,
            payment: None,
            cycles_to_forward: None,
        };
        let bridge_args: BridgeCallArgs = args.clone().into();
        assert_eq!(bridge_args.target, args.target);
        assert_eq!(bridge_args.method, args.method);
        assert_eq!(bridge_args.fee_amount, args.fee_amount);
        assert_eq!(bridge_args.payment, args.payment);
        assert_eq!(bridge_args.cycles_to_forward, args.cycles_to_forward);
        // call_text currently does not use args_text, so args should be empty
        assert_eq!(bridge_args.args, Vec::<u8>::new());
    }
}
