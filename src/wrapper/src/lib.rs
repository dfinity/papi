use ic_cdk::export_candid;
use ic_cdk::update;

pub mod api;
pub mod domain;
pub mod payments;
pub mod util;

use crate::api::call::bridge_call;
use crate::domain::types::{BridgeCallArgs, Call0Args, CallBlobArgs, CallTextArgs};

/// Proxies a call to a target method that takes **no arguments**.
#[update]
pub async fn call0(args: Call0Args) -> Result<Vec<u8>, String> {
    bridge_call(args.into()).await
}

/// Proxies a call using a **Candid-encoded argument blob**.
#[update]
pub async fn call_blob(args: CallBlobArgs) -> Result<Vec<u8>, String> {
    bridge_call(args.into()).await
}

/// Proxies a call using **Candid text** (currently disabled).
#[update]
#[allow(clippy::needless_pass_by_value)]
pub fn call_text(args: CallTextArgs) -> Result<Vec<u8>, String> {
    let _args: BridgeCallArgs = args.into();

    Err("call_text is currently disabled due to a workspace dependency conflict with the Candid parser. Please use call_blob instead.".to_string())
}

export_candid!();
