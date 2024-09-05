use crate::util::cycles_depositor::{self, CyclesDepositorPic};
use crate::util::cycles_ledger::{Account, CyclesLedgerPic, InitArgs, LedgerArgs};
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{encode_one, Nat, Principal};
use ic_papi_api::PaymentError;
use pocket_ic::PocketIc;
use std::sync::Arc;

pub struct CallerPaysWithIcRc2TestSetup {
    /// The PocketIC instance.
    #[allow(dead_code)]
    // The Arc is used; this makes it accessible without having to refer to a specific canister.
    pic: Arc<PocketIc>,
    /// The canister providing the API.
    paid_service: PicCanister,
    /// ICRC2 ledger
    ledger: CyclesLedgerPic,
    /// User
    user: Principal,
    /// User's wallet.  We use the cycles wallet so that we can top it up easily, but any source of funds will do, with any ICRC-2 token.
    wallet: CyclesDepositorPic,
}
impl Default for CallerPaysWithIcRc2TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let paid_service = PicCanister::new(
            pic.clone(),
            &PicCanister::cargo_wasm_path("example_paid_service"),
        );
        let ledger = CyclesLedgerPic::from(
            PicCanisterBuilder::default()
                .with_wasm(&PicCanister::dfx_wasm_path("cycles_ledger"))
                .with_arg(
                    encode_one(LedgerArgs::Init(InitArgs {
                        index_id: None,
                        max_blocks_per_request: 999,
                    }))
                    .expect("Failed to encode ledger init arg"),
                )
                .deploy_to(pic.clone()),
        );
        let user =
            Principal::from_text("xzg7k-thc6c-idntg-knmtz-2fbhh-utt3e-snqw6-5xph3-54pbp-7axl5-tae")
                .unwrap();
        let wallet = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::dfx_wasm_path("cycles_wallet"))
            .with_controllers(vec![user])
            .with_arg(
                encode_one(cycles_depositor::InitArg {
                    ledger_id: ledger.canister_id,
                })
                .unwrap(),
            )
            .deploy_to(pic.clone())
            .into();
        Self {
            pic,
            paid_service,
            ledger,
            user,
            wallet,
        }
    }
}

#[test]
fn icrc2_test_setup_works() {
    let _setup = CallerPaysWithIcRc2TestSetup::default();
}

#[test]
fn icrc2_payment_works() {
    let setup = CallerPaysWithIcRc2TestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    let balance = setup.ledger.icrc_1_balance_of(setup.user, &Account { owner: setup.user, subaccount: None }).expect("Could not get user balance");
    assert_eq!(balance, Nat::from(0u32), "User should have zero balance in the ledger");
    // .. Magic cycles into existence (test only - not IRL).
    setup.pic.add_cycles(setup.wallet.canister_id, 100_000_000);
    // .. Send cycles to the cycles ledger.
    setup.wallet.deposit(
        setup.user,
        &cycles_depositor::DepositArg {
            to: cycles_depositor::Account {
                owner: setup.user,
                subaccount: None,
            },
            memo: None,
            cycles: candid::Nat::from(100_000u32),
        },
    ).expect("Failed to deposit funds in the ledger");
    let balance = setup.ledger.icrc_1_balance_of(setup.user, &Account { owner: setup.user, subaccount: None }).expect("Could not get user balance");
    const MIN_BALANCE: u64 = 1_000_000;
    assert!(balance >= Nat::from(MIN_BALANCE), "User needs a balance of at least {MIN_BALANCE} in the ledger but has only {balance}");
}
