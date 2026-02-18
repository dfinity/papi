use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{encode_one, Principal};
use pocket_ic::PocketIc;
use std::sync::Arc;

pub struct TestSetup {
    pub pic: Arc<PocketIc>,
    pub wrapper: PicCanister,
    pub target: PicCanister,
    pub user: Principal,
}

impl Default for TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());

        // Deploy a mock target canister (could be the same example paid service or a simple one)
        let target = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("example_paid_service"))
            .with_arg(encode_one(Some(candid::Principal::anonymous())).unwrap()) // Simple init
            .deploy_to(pic.clone());

        let wrapper = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("ic_papi_wrapper"))
            .deploy_to(pic.clone());

        let user =
            Principal::from_text("xzg7k-thc6c-idntg-knmtz-2fbhh-utt3e-snqw6-5xph3-54pbp-7axl5-tae")
                .unwrap();

        Self {
            pic,
            wrapper,
            target,
            user,
        }
    }
}
