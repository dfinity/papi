use crate::util::cycles_depositor::{self, CyclesDepositorPic};
use crate::util::cycles_ledger::{
    Account, ApproveArgs, CyclesLedgerPic, InitArgs as LedgerInitArgs, LedgerArgs,
};
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{encode_one, Nat, Principal};
use example_paid_service_api::InitArgs;
use ic_papi_api::cycles::CYCLES_LEDGER_CANISTER_ID;
use ic_papi_api::{PaymentError, PaymentType};
use pocket_ic::{PocketIc, PocketIcBuilder};
use std::sync::Arc;

const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees

pub struct CallerPaysWithIcrc2CyclesTestSetup {
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
    /// Another user
    user2: Principal,
    /// We should really put these in an array
    users: [Principal; 5],
    /// Unauthorized user
    unauthorized_user: Principal,
    /// User's wallet.  We use the cycles wallet so that we can top it up easily, but any source of funds will do, with any ICRC-2 token.
    wallet: CyclesDepositorPic,
}
impl Default for CallerPaysWithIcrc2CyclesTestSetup {
    fn default() -> Self {
        let pic = Arc::new(
            PocketIcBuilder::new()
                .with_fiduciary_subnet()
                .with_system_subnet()
                .with_application_subnet()
                .with_ii_subnet()
                .with_nns_subnet()
                .build(),
        );
        let cycles_ledger_canister_id = pic
            .create_canister_with_id(
                None,
                None,
                Principal::from_text(CYCLES_LEDGER_CANISTER_ID).unwrap(),
            )
            .unwrap();

        // Would like to create this with the cycles ledger canister ID but currently this yields an error.
        let ledger = CyclesLedgerPic::from(
            PicCanisterBuilder::default()
                .with_canister(cycles_ledger_canister_id)
                .with_wasm(&PicCanister::dfx_wasm_path("cycles_ledger"))
                .with_arg(
                    encode_one(LedgerArgs::Init(LedgerInitArgs {
                        index_id: None,
                        max_blocks_per_request: 999,
                    }))
                    .expect("Failed to encode ledger init arg"),
                )
                .deploy_to(pic.clone()),
        );
        let paid_service = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("example_paid_service"))
            .with_arg(
                encode_one(Some(InitArgs {
                    ledger: ledger.canister_id(),
                }))
                .unwrap(),
            )
            .deploy_to(pic.clone());
        let user =
            Principal::from_text("xzg7k-thc6c-idntg-knmtz-2fbhh-utt3e-snqw6-5xph3-54pbp-7axl5-tae")
                .unwrap();
        let user2 =
            Principal::from_text("jwhyn-xieqy-drmun-h7uci-jzycw-vnqhj-s62vl-4upsg-cmub3-vakaq-rqe")
                .unwrap();
        let users = [
            Principal::from_text("s2xin-cwqnw-sjvht-gp553-an54g-2rhlc-z4c5d-xz5iq-irnbi-sadik-qae")
                .unwrap(),
            Principal::from_text("dmvof-2tilt-3xmvh-c7tbj-n3whk-k2i6b-2s2ge-xoo3d-wjuw3-ijpuw-eae")
                .unwrap(),
            Principal::from_text("kjerd-nj73t-u3hhp-jcj4d-g7w56-qlrvb-gguta-45yve-336zs-sunxa-zqe")
                .unwrap(),
            Principal::from_text("zxhav-yshtx-vhzs2-nvuu3-jrq66-bidn2-put3y-ulwcf-2gb2o-ykfco-sae")
                .unwrap(),
            Principal::from_text("nggqm-p5ozz-i5hfv-bejmq-2gtow-4dtqw-vjatn-4b4yw-s5mzs-i46su-6ae")
                .unwrap(),
        ];
        let unauthorized_user =
            Principal::from_text("rg3gz-22tjp-jh7hl-migkq-vb7in-i2ylc-6umlc-dtbug-v6jgc-uo24d-nqe")
                .unwrap();
        let wallet = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::dfx_wasm_path("cycles_depositor"))
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
            user2,
            users,
            unauthorized_user,
            wallet,
        }
    }
}
impl CallerPaysWithIcrc2CyclesTestSetup {
    /// Deposit 100 * the ledger fee in the user's ledger wallet. That should be enough to be getting on with.
    fn fund_user(&self, megasquigs: u128) {
        let initial_balance = self.user_balance();
        // .. Magic cycles into existence (test only - not IRL).
        let deposit = megasquigs + LEDGER_FEE;
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
    /// User sends an ICRC2 approval with teh paid service as spender.
    fn user_approves_payment_for_paid_service<T>(&self, amount: T)
    where
        T: Into<Nat>,
    {
        self.ledger
            .icrc_2_approve(
                self.user,
                &ApproveArgs {
                    spender: Account {
                        owner: self.paid_service.canister_id(),
                        subaccount: None,
                    },
                    amount: amount.into(),
                    ..ApproveArgs::default()
                },
            )
            .expect("Failed to call the ledger to approve")
            .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
    }
}

#[test]
fn icrc2_test_setup_works() {
    let _setup = CallerPaysWithIcrc2CyclesTestSetup::default();
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected
/// on an API method that uses the corresponding guard implicitly, with no payment aguments.
///
/// Note: The method used is: cost_1b_icrc2_from_caller
#[test]
fn caller_pays_icrc2_cycles() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Exercise the protocol...
    let api_method = "cost_1b_icrc2_from_caller";
    let api_fee = 1_000_000_000u128;
    for payment in (api_fee - 5)..(api_fee + 5) {
        for _repetition in 0..2 {
            // Pre-approve payment
            setup.user_approves_payment_for_paid_service(payment + LEDGER_FEE);
            // Check that the user has been charged for the approve.
            expected_user_balance -= LEDGER_FEE;
            setup.assert_user_balance_eq(
                expected_user_balance,
                "Expected the user balance to be charged for the ICRC2 approve".to_string(),
            );

            // Check the balance beforehand
            let service_canister_cycles_before =
                setup.pic.cycle_balance(setup.paid_service.canister_id);
            // Call the API
            let response: Result<String, PaymentError> = setup
                .paid_service
                .update(setup.user, api_method, ())
                .expect("Failed to call the paid service");
            if payment < api_fee {
                assert_eq!(
                    response,
                    Err(PaymentError::LedgerError {
                        ledger: setup.ledger.canister_id(),
                        error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                            allowance: Nat::from(payment + LEDGER_FEE), // TODO: Change up to 128
                        }
                    }),
                    "Should have failed with only {} cycles attached",
                    payment
                );
                setup.assert_user_balance_eq(
                    expected_user_balance,
                    "Expected the user balance to be unchanged by a failed ICRC2".to_string(),
                );
            } else {
                assert_eq!(
                    response,
                    Ok("Yes, you paid 1 billion cycles!".to_string()),
                    "Should have succeeded with {} cycles attached",
                    payment
                );
                let service_canister_cycles_after =
                    setup.pic.cycle_balance(setup.paid_service.canister_id);
                assert!(
                    service_canister_cycles_after > service_canister_cycles_before,
                    "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
                    service_canister_cycles_before - service_canister_cycles_after
                );
                expected_user_balance -= api_fee + LEDGER_FEE;
                setup.assert_user_balance_eq(
                expected_user_balance,
                "Expected the user balance to be the initial balance minus the ledger and API fees"
                    .to_string(),
            );
            }
        }
    }
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected:
/// 
/// - The user's main cycles account has cycles deducted.
/// - The cycle balance of the canister providing the paid service increases.
///   - Note: Given that the canister consumes cycles as part of the operation, we check that the balance increases but do not check an exact amount. 
///
/// Note: The method used is: cost_1b_icrc2_from_caller

#[test]
fn caller_pays_icrc2_cycles_with_payment_arg_works() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Exercise the protocol...
    let api_method = "cost_1b_icrc2_from_caller";
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(expected_user_balance);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make several identical API calls
    for _repetition in 0..5 {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> = setup
            .paid_service
            .update(setup.user, api_method, ())
            .expect("Failed to call the paid service");
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded with a generous prepayment",
        );
        let service_canister_cycles_after = setup.pic.cycle_balance(setup.paid_service.canister_id);
        assert!(
            service_canister_cycles_after > service_canister_cycles_before,
            "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
            service_canister_cycles_before - service_canister_cycles_after
        );
        expected_user_balance -= api_fee + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
    }
}

/// Verifies that the `PaymentType::CallerPaysIcrc2Cycles` payment type works as expected
/// on an API method that takes a payment argument.
///
/// Note: The method used is: `cost_1b``
#[test]
fn caller_pays_by_named_icrc2() {
    let setup = CallerPaysWithIcrc2CyclesTestSetup::default();
    // Add cycles to the wallet
    // .. At first the balance should be zero.
    setup.assert_user_balance_eq(
        0u32,
        "Initially the user balance in the ledger should be zero".to_string(),
    );
    // .. Get enough to play with lots of transactions.
    const LEDGER_FEE: u128 = 100_000_000; // The documented fee: https://internetcomputer.org/docs/current/developer-docs/defi/cycles/cycles-ledger#fees
    let mut expected_user_balance = 100_000_000_000; // Lots of funds to play with.
    setup.fund_user(expected_user_balance);
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Test setup failed when providing the user with funds".to_string(),
    );
    // Ok, now we should be able to make an API call with an ICRC-2 approve.
    let api_method = "cost_1b";
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup.user_approves_payment_for_paid_service(expected_user_balance);
    // Check that the user has been charged for the approve.
    expected_user_balance -= LEDGER_FEE;
    setup.assert_user_balance_eq(
        expected_user_balance,
        "Expected the user balance to be charged for the ICRC2 approve".to_string(),
    );
    // Now make several identical API calls
    for _repetition in 0..5 {
        // Check the balance beforehand
        let service_canister_cycles_before =
            setup.pic.cycle_balance(setup.paid_service.canister_id);
        // Call the API
        let response: Result<String, PaymentError> = setup
            .paid_service
            .update(setup.user, api_method, PaymentType::CallerPaysIcrc2Cycles)
            .expect("Failed to call the paid service");
        assert_eq!(
            response,
            Ok("Yes, you paid 1 billion cycles!".to_string()),
            "Should have succeeded with a generous prepayment",
        );
        let service_canister_cycles_after = setup.pic.cycle_balance(setup.paid_service.canister_id);
        assert!(
            service_canister_cycles_after > service_canister_cycles_before,
            "The service canister needs to charge more to cover its cycle cost!  Loss: {}",
            service_canister_cycles_before - service_canister_cycles_after
        );
        expected_user_balance -= api_fee + LEDGER_FEE;
        setup.assert_user_balance_eq(
            expected_user_balance,
            "Expected the user balance to be the initial balance minus the ledger and API fees"
                .to_string(),
        );
        // But an unauthorized user should not be able to make the same call.
        {
            let response: Result<String, PaymentError> = setup
                .paid_service
                .update(
                    setup.unauthorized_user,
                    api_method,
                    PaymentType::CallerPaysIcrc2Cycles,
                )
                .expect("Failed to call the paid service");
            assert_eq!(
                response,
                Err(PaymentError::LedgerError {
                    ledger: setup.ledger.canister_id(),
                    error: cycles_ledger_client::WithdrawFromError::InsufficientAllowance {
                        allowance: Nat::from(0u32),
                    }
                }),
                "Should have succeeded with a generous prepayment",
            );
            setup.assert_user_balance_eq(
                expected_user_balance,
                "The user should not have been charged for unauthorized spending attempts"
                    .to_string(),
            );
        }
    }
}
