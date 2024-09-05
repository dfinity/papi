// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use std::sync::Arc;

use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;
use pocket_ic::PocketIc;

use super::pic_canister::PicCanisterTrait;

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum Kind { User, Canister, Unknown }
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum Role { Custodian, Contact, Controller }
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct AddressEntry {
  pub(crate) id: Principal,
  pub(crate) kind: Kind,
  pub(crate) name: Option<String>,
  pub(crate) role: Role,
}
pub(crate) type WalletResult = std::result::Result<(), String>;
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct GetChartArgInner {
  pub(crate) count: Option<u32>,
  pub(crate) precision: Option<u64>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct GetEventsArgInner {
  pub(crate) to: Option<u32>,
  pub(crate) from: Option<u32>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum EventKind {
  CyclesReceived{ from: Principal, memo: Option<String>, amount: u64 },
  CanisterCreated{ cycles: u64, canister: Principal },
  CanisterCalled{ cycles: u64, method_name: String, canister: Principal },
  CyclesSent{ to: Principal, amount: u64, refund: u64 },
  AddressRemoved{ id: Principal },
  WalletDeployed{ canister: Principal },
  AddressAdded{ id: Principal, name: Option<String>, role: Role },
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct Event {
  pub(crate) id: u32,
  pub(crate) kind: EventKind,
  pub(crate) timestamp: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct GetEvents128ArgInner {
  pub(crate) to: Option<u32>,
  pub(crate) from: Option<u32>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum EventKind128 {
  CyclesReceived{ from: Principal, memo: Option<String>, amount: candid::Nat },
  CanisterCreated{ cycles: candid::Nat, canister: Principal },
  CanisterCalled{
    cycles: candid::Nat,
    method_name: String,
    canister: Principal,
  },
  CyclesSent{ to: Principal, amount: candid::Nat, refund: candid::Nat },
  AddressRemoved{ id: Principal },
  WalletDeployed{ canister: Principal },
  AddressAdded{ id: Principal, name: Option<String>, role: Role },
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct Event128 {
  pub(crate) id: u32,
  pub(crate) kind: EventKind128,
  pub(crate) timestamp: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct GetManagedCanisterEventsArg {
  pub(crate) to: Option<u32>,
  pub(crate) from: Option<u32>,
  pub(crate) canister: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum ManagedCanisterEventKind {
  CyclesSent{ amount: u64, refund: u64 },
  Created{ cycles: u64 },
  Called{ cycles: u64, method_name: String },
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct ManagedCanisterEvent {
  pub(crate) id: u32,
  pub(crate) kind: ManagedCanisterEventKind,
  pub(crate) timestamp: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct GetManagedCanisterEvents128Arg {
  pub(crate) to: Option<u32>,
  pub(crate) from: Option<u32>,
  pub(crate) canister: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum ManagedCanisterEventKind128 {
  CyclesSent{ amount: candid::Nat, refund: candid::Nat },
  Created{ cycles: candid::Nat },
  Called{ cycles: candid::Nat, method_name: String },
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct ManagedCanisterEvent128 {
  pub(crate) id: u32,
  pub(crate) kind: ManagedCanisterEventKind128,
  pub(crate) timestamp: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct HeaderField (pub(crate) String,pub(crate) String,);
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct HttpRequest {
  pub(crate) url: String,
  pub(crate) method: String,
  pub(crate) body: serde_bytes::ByteBuf,
  pub(crate) headers: Vec<HeaderField>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct Token {}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct StreamingCallbackHttpResponse {
  pub(crate) token: Option<Token>,
  pub(crate) body: serde_bytes::ByteBuf,
}
candid::define_function!(pub(crate) StreamingStrategyCallbackCallback : (
    Token,
  ) -> (StreamingCallbackHttpResponse) query);
#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum StreamingStrategy {
  Callback{ token: Token, callback: StreamingStrategyCallbackCallback },
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct HttpResponse {
  pub(crate) body: serde_bytes::ByteBuf,
  pub(crate) headers: Vec<HeaderField>,
  pub(crate) streaming_strategy: Option<StreamingStrategy>,
  pub(crate) status_code: u16,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct ListManagedCanistersArg {
  pub(crate) to: Option<u32>,
  pub(crate) from: Option<u32>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct ManagedCanisterInfo {
  pub(crate) id: Principal,
  pub(crate) name: Option<String>,
  pub(crate) created_at: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletBalanceRet { pub(crate) amount: u64 }
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletBalance128Ret { pub(crate) amount: candid::Nat }
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletCallArg {
  pub(crate) args: serde_bytes::ByteBuf,
  pub(crate) cycles: u64,
  pub(crate) method_name: String,
  pub(crate) canister: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletResultCallOk {
  pub(crate) r#return: serde_bytes::ByteBuf,
}
pub(crate) type WalletResultCall = std::result::Result<
  WalletResultCallOk, String
>;
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletCall128Arg {
  pub(crate) args: serde_bytes::ByteBuf,
  pub(crate) cycles: candid::Nat,
  pub(crate) method_name: String,
  pub(crate) canister: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletCallWithMaxCyclesArg {
  pub(crate) args: serde_bytes::ByteBuf,
  pub(crate) method_name: String,
  pub(crate) canister: Principal,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletResultCallWithMaxCyclesOk {
  pub(crate) r#return: serde_bytes::ByteBuf,
  pub(crate) attached_cycles: candid::Nat,
}
pub(crate) type WalletResultCallWithMaxCycles = std::result::Result<
  WalletResultCallWithMaxCyclesOk, String
>;
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct CanisterSettings {
  pub(crate) controller: Option<Principal>,
  pub(crate) freezing_threshold: Option<candid::Nat>,
  pub(crate) controllers: Option<Vec<Principal>>,
  pub(crate) memory_allocation: Option<candid::Nat>,
  pub(crate) compute_allocation: Option<candid::Nat>,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct CreateCanisterArgs {
  pub(crate) cycles: u64,
  pub(crate) settings: CanisterSettings,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletResultCreateOk { pub(crate) canister_id: Principal }
pub(crate) type WalletResultCreate = std::result::Result<
  WalletResultCreateOk, String
>;
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct CreateCanisterArgs128 {
  pub(crate) cycles: candid::Nat,
  pub(crate) settings: CanisterSettings,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct ReceiveOptions { pub(crate) memo: Option<String> }
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletSendArg {
  pub(crate) canister: Principal,
  pub(crate) amount: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletSend128Arg {
  pub(crate) canister: Principal,
  pub(crate) amount: candid::Nat,
}
#[derive(CandidType, Deserialize, Debug)]
pub(crate) struct WalletStoreWalletWasmArg {
  pub(crate) wasm_module: serde_bytes::ByteBuf,
}

pub struct Service(pub Principal);
impl Service {
  pub async fn add_address(&self, arg0: &AddressEntry) -> Result<()> {
    ic_cdk::call(self.0, "add_address", (arg0,)).await
  }
  pub async fn add_controller(&self, arg0: &Principal) -> Result<()> {
    ic_cdk::call(self.0, "add_controller", (arg0,)).await
  }
  pub async fn authorize(&self, arg0: &Principal) -> Result<()> {
    ic_cdk::call(self.0, "authorize", (arg0,)).await
  }
  pub async fn deauthorize(&self, arg0: &Principal) -> Result<(WalletResult,)> {
    ic_cdk::call(self.0, "deauthorize", (arg0,)).await
  }
  pub async fn get_chart(&self, arg0: &Option<GetChartArgInner>) -> Result<(Vec<(u64,u64,)>,)> {
    ic_cdk::call(self.0, "get_chart", (arg0,)).await
  }
  pub async fn get_controllers(&self) -> Result<(Vec<Principal>,)> {
    ic_cdk::call(self.0, "get_controllers", ()).await
  }
  pub async fn get_custodians(&self) -> Result<(Vec<Principal>,)> {
    ic_cdk::call(self.0, "get_custodians", ()).await
  }
  pub async fn get_events(&self, arg0: &Option<GetEventsArgInner>) -> Result<(Vec<Event>,)> {
    ic_cdk::call(self.0, "get_events", (arg0,)).await
  }
  pub async fn get_events_128(&self, arg0: &Option<GetEvents128ArgInner>) -> Result<(Vec<Event128>,)> {
    ic_cdk::call(self.0, "get_events128", (arg0,)).await
  }
  pub async fn get_managed_canister_events(&self, arg0: &GetManagedCanisterEventsArg) -> Result<(Option<Vec<ManagedCanisterEvent>>,)> {
    ic_cdk::call(self.0, "get_managed_canister_events", (arg0,)).await
  }
  pub async fn get_managed_canister_events_128(&self, arg0: &GetManagedCanisterEvents128Arg) -> Result<(Option<Vec<ManagedCanisterEvent128>>,)> {
    ic_cdk::call(self.0, "get_managed_canister_events128", (arg0,)).await
  }
  pub async fn http_request(&self, arg0: &HttpRequest) -> Result<(HttpResponse,)> {
    ic_cdk::call(self.0, "http_request", (arg0,)).await
  }
  pub async fn list_addresses(&self) -> Result<(Vec<AddressEntry>,)> {
    ic_cdk::call(self.0, "list_addresses", ()).await
  }
  pub async fn list_managed_canisters(&self, arg0: &ListManagedCanistersArg) -> Result<(Vec<ManagedCanisterInfo>,u32,)> {
    ic_cdk::call(self.0, "list_managed_canisters", (arg0,)).await
  }
  pub async fn name(&self) -> Result<(Option<String>,)> {
    ic_cdk::call(self.0, "name", ()).await
  }
  pub async fn remove_address(&self, arg0: &Principal) -> Result<(WalletResult,)> {
    ic_cdk::call(self.0, "remove_address", (arg0,)).await
  }
  pub async fn remove_controller(&self, arg0: &Principal) -> Result<(WalletResult,)> {
    ic_cdk::call(self.0, "remove_controller", (arg0,)).await
  }
  pub async fn set_name(&self, arg0: &String) -> Result<()> {
    ic_cdk::call(self.0, "set_name", (arg0,)).await
  }
  pub async fn set_short_name(&self, arg0: &Principal, arg1: &Option<String>) -> Result<(Option<ManagedCanisterInfo>,)> {
    ic_cdk::call(self.0, "set_short_name", (arg0,arg1,)).await
  }
  pub async fn wallet_api_version(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "wallet_api_version", ()).await
  }
  pub async fn wallet_balance(&self) -> Result<(WalletBalanceRet,)> {
    ic_cdk::call(self.0, "wallet_balance", ()).await
  }
  pub async fn wallet_balance_128(&self) -> Result<(WalletBalance128Ret,)> {
    ic_cdk::call(self.0, "wallet_balance128", ()).await
  }
  pub async fn wallet_call(&self, arg0: &WalletCallArg) -> Result<(WalletResultCall,)> {
    ic_cdk::call(self.0, "wallet_call", (arg0,)).await
  }
  pub async fn wallet_call_128(&self, arg0: &WalletCall128Arg) -> Result<(WalletResultCall,)> {
    ic_cdk::call(self.0, "wallet_call128", (arg0,)).await
  }
  pub async fn wallet_call_with_max_cycles(&self, arg0: &WalletCallWithMaxCyclesArg) -> Result<(WalletResultCallWithMaxCycles,)> {
    ic_cdk::call(self.0, "wallet_call_with_max_cycles", (arg0,)).await
  }
  pub async fn wallet_create_canister(&self, arg0: &CreateCanisterArgs) -> Result<(WalletResultCreate,)> {
    ic_cdk::call(self.0, "wallet_create_canister", (arg0,)).await
  }
  pub async fn wallet_create_canister_128(&self, arg0: &CreateCanisterArgs128) -> Result<(WalletResultCreate,)> {
    ic_cdk::call(self.0, "wallet_create_canister128", (arg0,)).await
  }
  pub async fn wallet_create_wallet(&self, arg0: &CreateCanisterArgs) -> Result<(WalletResultCreate,)> {
    ic_cdk::call(self.0, "wallet_create_wallet", (arg0,)).await
  }
  pub async fn wallet_create_wallet_128(&self, arg0: &CreateCanisterArgs128) -> Result<(WalletResultCreate,)> {
    ic_cdk::call(self.0, "wallet_create_wallet128", (arg0,)).await
  }
  pub async fn wallet_receive(&self, arg0: &Option<ReceiveOptions>) -> Result<()> {
    ic_cdk::call(self.0, "wallet_receive", (arg0,)).await
  }
  pub async fn wallet_send(&self, arg0: &WalletSendArg) -> Result<(WalletResult,)> {
    ic_cdk::call(self.0, "wallet_send", (arg0,)).await
  }
  pub async fn wallet_send_128(&self, arg0: &WalletSend128Arg) -> Result<(WalletResult,)> {
    ic_cdk::call(self.0, "wallet_send128", (arg0,)).await
  }
  pub async fn wallet_store_wallet_wasm(&self, arg0: &WalletStoreWalletWasmArg) -> Result<()> {
    ic_cdk::call(self.0, "wallet_store_wallet_wasm", (arg0,)).await
  }
}

pub struct CyclesWalletPic {
  pub pic: Arc<PocketIc>,
  pub canister_id: Principal,
}

impl PicCanisterTrait for CyclesWalletPic {
  /// The shared PocketIc instance.
  fn pic(&self) -> Arc<PocketIc> {
      self.pic.clone()
  }
  /// The ID of this canister.
  fn canister_id(&self) -> Principal {
      self.canister_id.clone()
  }
}
