use crate::util::cycles_depositor::{self, CyclesDepositorPic};
use crate::util::cycles_ledger::{
    Account, ApproveArgs, CyclesLedgerPic, InitArgs as LedgerInitArgs, LedgerArgs,
};
use crate::util::pic_canister::{PicCanister, PicCanisterBuilder, PicCanisterTrait};
use candid::{encode_one, Nat, Principal};
use example_paid_service_api::InitArgs;
use ic_papi_api::{principal2account, Icrc2Payer, PaymentError, PaymentType};
use pocket_ic::PocketIc;
use serde_bytes::ByteBuf;
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
    /// Another user
    user2: Principal,
    /// Unauthorized user
    unauthorized_user: Principal,
    /// User's wallet.  We use the cycles wallet so that we can top it up easily, but any source of funds will do, with any ICRC-2 token.
    wallet: CyclesDepositorPic,
}
impl Default for CallerPaysWithIcRc2TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());
        let ledger = CyclesLedgerPic::from(
            PicCanisterBuilder::default()
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
                    ledger: Some(ledger.canister_id()),
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
        let unauthorized_user =
            Principal::from_text("rg3gz-22tjp-jh7hl-migkq-vb7in-i2ylc-6umlc-dtbug-v6jgc-uo24d-nqe")
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
            user2,
            unauthorized_user,
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
fn caller_pays_by_icrc2() {
    let setup = CallerPaysWithIcRc2TestSetup::default();
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
    for payment in (api_fee - 5)..(api_fee + 5) {
        for _repetition in 0..2 {
            // Pre-approve payment
            setup
                .ledger
                .icrc_2_approve(
                    setup.user,
                    &ApproveArgs {
                        spender: Account {
                            owner: setup.paid_service.canister_id(),
                            subaccount: None,
                        },
                        amount: Nat::from(payment + LEDGER_FEE),
                        ..ApproveArgs::default()
                    },
                )
                .expect("Failed to call the ledger to approve")
                .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
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

#[test]
fn caller_pays_by_icrc2_prepayment() {
    let setup = CallerPaysWithIcRc2TestSetup::default();
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
    setup
        .ledger
        .icrc_2_approve(
            setup.user,
            &ApproveArgs {
                spender: Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: None,
                },
                amount: Nat::from(expected_user_balance),
                ..ApproveArgs::default()
            },
        )
        .expect("Failed to call the ledger to approve")
        .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
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

#[test]
fn caller_pays_by_named_icrc2() {
    let setup = CallerPaysWithIcRc2TestSetup::default();
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
    // Ok, now we should be able to make an API call with EITHER an ICRC-2 approve or attached cycles, by declaring the payment type.
    // In this test, we will exercise the ICRC-2 approve.
    let api_method = "cost_1b";
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup
        .ledger
        .icrc_2_approve(
            setup.user,
            &ApproveArgs {
                spender: Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: None,
                },
                amount: Nat::from(expected_user_balance),
                ..ApproveArgs::default()
            },
        )
        .expect("Failed to call the ledger to approve")
        .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
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
            .update(setup.user, api_method, PaymentType::CallerIcrc2)
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
                    PaymentType::CallerIcrc2,
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

/// Here `user`` is a patron, and pays on behalf of `user2`.
#[test]
fn patron_pays_by_named_icrc2() {
    let setup = CallerPaysWithIcRc2TestSetup::default();
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
    // Ok, now we should be able to make an API call with EITHER an ICRC-2 approve or attached cycles, by declaring the payment type.
    // In this test, we will exercise the ICRC-2 approve.
    let api_method = "cost_1b";
    let api_fee = 1_000_000_000u128;
    // Pre-approve payment
    setup
        .ledger
        .icrc_2_approve(
            setup.user,
            &ApproveArgs {
                spender: Account {
                    owner: setup.paid_service.canister_id(),
                    subaccount: Some(principal2account(&setup.user2)),
                },
                amount: Nat::from(expected_user_balance),
                ..ApproveArgs::default()
            },
        )
        .expect("Failed to call the ledger to approve")
        .expect("Failed to approve the paid service to spend the user's ICRC-2 tokens");
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
        let payment_arg = PaymentType::PatronIcrc2(setup.user);
        let response: Result<String, PaymentError> = setup
            .paid_service
            .update(setup.user2, api_method, payment_arg)
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
                .update(setup.unauthorized_user, api_method, payment_arg)
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
