use candid::{decode_one, encode_one, CandidType, Deserialize, Principal};
use pocket_ic::PocketIc;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Common methods for interacting with a canister using `PocketIc`.
pub trait PicCanisterTrait {
    /// A shared PocketIc instance.
    fn pic(&self) -> Arc<PocketIc>;

    /// The ID of this canister.
    fn canister_id(&self) -> Principal;

    /// Makes an update call to the canister.
    fn update<T>(&self, caller: Principal, method: &str, arg: impl CandidType) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a> + CandidType,
    {
        self.pic()
            .update_call(self.canister_id(), caller, method, encode_one(arg).unwrap())
            .map_err(|e| {
                format!(
                    "Update call error. RejectionCode: {:?}, Error: {}",
                    e.reject_code, e.reject_message
                )
            })
            .and_then(|bytes| decode_one(&bytes).map_err(|e| format!("Decoding failed: {e}")))
    }

    /// Makes a query call to the canister.
    #[allow(dead_code)]
    fn query<T>(&self, caller: Principal, method: &str, arg: impl CandidType) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a> + CandidType,
    {
        self.pic()
            .query_call(self.canister_id(), caller, method, encode_one(arg).unwrap())
            .map_err(|e| {
                format!(
                    "Query call error. RejectionCode: {:?}, Error: {}",
                    e.reject_code, e.reject_message
                )
            })
            .and_then(|bytes| decode_one(&bytes).map_err(|_| "Decoding failed".to_string()))
    }
    fn workspace_dir() -> PathBuf {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
        cargo_path.parent().unwrap().to_path_buf()
    }
    /// The path to a typical Cargo Wasm build.
    fn cargo_wasm_path(name: &str) -> String {
        let workspace_dir = Self::workspace_dir();
        workspace_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(name)
            .with_extension("wasm")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// A typical canister running on PocketIC.
pub struct PicCanister {
    pub pic: Arc<PocketIc>,
    pub canister_id: Principal,
}

impl PicCanisterTrait for PicCanister {
    /// The shared PocketIc instance.
    fn pic(&self) -> Arc<PocketIc> {
        self.pic.clone()
    }
    /// The ID of this canister.
    fn canister_id(&self) -> Principal {
        self.canister_id
    }
}

impl PicCanister {
    /// Creates a new canister.
    pub fn new(pic: Arc<PocketIc>, wasm_path: &str) -> Self {
        PicCanisterBuilder::default()
            .with_wasm(wasm_path)
            .deploy_to(pic)
    }
}

#[derive(Debug)]
pub struct PicCanisterBuilder {
    canister_id: Option<Principal>,
    cycles: u128,
    wasm_path: String,
    arg: Vec<u8>,
    controllers: Option<Vec<Principal>>,
}
// Defaults
impl PicCanisterBuilder {
    pub const DEFAULT_CYCLES: u128 = 2_000_000_000_000;
    pub fn default_arg() -> Vec<u8> {
        encode_one(()).unwrap()
    }
}
impl Default for PicCanisterBuilder {
    fn default() -> Self {
        Self {
            canister_id: None,
            cycles: Self::DEFAULT_CYCLES,
            wasm_path: "unspecified.wasm".to_string(),
            arg: Self::default_arg(),
            controllers: None,
        }
    }
}
// Customisation
impl PicCanisterBuilder {
    pub fn with_arg(mut self, arg: Vec<u8>) -> Self {
        self.arg = arg;
        self
    }
    pub fn with_canister(mut self, canister_id: Principal) -> Self {
        self.canister_id = Some(canister_id);
        self
    }
    pub fn with_controllers(mut self, controllers: Vec<Principal>) -> Self {
        self.controllers = Some(controllers);
        self
    }
    pub fn with_cycles(mut self, cycles: u128) -> Self {
        self.cycles = cycles;
        self
    }
    pub fn with_wasm(mut self, wasm_path: &str) -> Self {
        self.wasm_path = wasm_path.to_string();
        self
    }
}
// Get parameters
impl PicCanisterBuilder {
    fn wasm_bytes(&self) -> Vec<u8> {
        fs::read(self.wasm_path.clone())
            .unwrap_or_else(|_| panic!("Could not find the backend wasm: {}", self.wasm_path))
    }
}
// Builder
impl PicCanisterBuilder {
    fn get_or_create_canister_id(&mut self, pic: &PocketIc) -> Principal {
        if let Some(canister_id) = self.canister_id {
            canister_id
        } else {
            let canister_id = pic.create_canister();
            self.canister_id = Some(canister_id);
            canister_id
        }
    }
    fn add_cycles(&mut self, pic: &PocketIc) {
        if self.cycles > 0 {
            let canister_id = self.get_or_create_canister_id(pic);
            pic.add_cycles(canister_id, self.cycles);
        }
    }
    fn install(&mut self, pic: &PocketIc) {
        let wasm_bytes = self.wasm_bytes();
        let canister_id = self.get_or_create_canister_id(pic);
        let arg = self.arg.clone();
        pic.install_canister(canister_id, wasm_bytes, arg, None);
    }
    fn set_controllers(&mut self, pic: &PocketIc) {
        if let Some(controllers) = self.controllers.clone() {
            let canister_id = self.get_or_create_canister_id(pic);
            pic.set_controllers(canister_id, None, controllers)
                .expect("Test setup error: Failed to set controllers");
        }
    }
    pub fn deploy_to(&mut self, pic: Arc<PocketIc>) -> PicCanister {
        let canister_id = self.get_or_create_canister_id(&pic);
        self.add_cycles(&pic);
        self.install(&pic);
        self.set_controllers(&pic);
        PicCanister {
            pic: pic.clone(),
            canister_id,
        }
    }
}
