use crate::util::cycles_depositor::{self, CyclesDepositorPic};
use crate::util::cycles_ledger::{Account, ApproveArgs, CyclesLedgerPic, InitArgs, LedgerArgs};
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{de, encode_one, Nat, Principal};
use ic_papi_api::PaymentError;
use pocket_ic::PocketIc;
use std::sync::Arc;
use std::u128::MAX;

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
impl CallerPaysWithIcRc2TestSetup {
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees

    /// Deposit 100 * the ledger fee in the user's ledger wallet. That should be enough to be getting on with.
    fn fund_user(&self, megasquigs: u128) {
        let initial_balance = self.user_balance();
        // .. Magic cycles into existence (test only - not IRL).
        let deposit = megasquigs + Self::LEDGER_FEE;
        self.pic.add_cycles(self.wallet.canister_id, deposit);
        // .. Send cycles to the cycles ledger.
        self.wallet
            .deposit(
                self.user,
                &cycles_depositor::DepositArg {
                    to: cycles_depositor::Account {
                        owner: self.user,
                        subaccount: None,
                    },
                    memo: None,
                    cycles: candid::Nat::from(deposit),
                },
            )
            .expect("Failed to deposit funds in the ledger");
        // .. That should have cost one fee.
        let expected_balance = initial_balance.clone() + megasquigs;
        self.assert_user_balance_eq(expected_balance.clone(), format!("Expected user balance to be the initial balance ({initial_balance}) plus the requested sum ({megasquigs}) = {expected_balance}"));
    }
    /// Gets the user balance
    fn user_balance(&self) -> Nat {
        self.ledger
            .icrc_1_balance_of(
                self.user,
                &Account {
                    owner: self.user,
                    subaccount: None,
                },
            )
            .expect("Could not get user balance")
    }
    /// Asserts that the user's ledger balance is a certain value.
    fn assert_user_balance_eq<T>(&self, expected_balance: T, message: String)
    where
        T: Into<Nat>,
    {
        assert_eq!(self.user_balance(), expected_balance.into(), "{}", message);
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
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees
    let mut remainder = 100u128; // Multiple of fees we have left to play with.
    setup.fund_user(LEDGER_FEE * remainder);
    setup.assert_user_balance_eq(
        LEDGER_FEE * remainder,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Exercise the protocol...
    let api_method = "cost_1000_icrc2_from_caller";
    let api_fee = 1_000u128;
    setup
        .ledger
        .icrc_2_approve(
            setup.user,
            &ApproveArgs {
                spender: Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: None,
                },
                amount: Nat::from(api_fee),
                ..ApproveArgs::default()
            },
        )
        .expect("Failed to call the ledger to approve")
        .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
    let response: Result<String, PaymentError> = setup
        .paid_service
        .update(setup.user, api_method, ())
        .expect("Failed to call the paid service");
}
