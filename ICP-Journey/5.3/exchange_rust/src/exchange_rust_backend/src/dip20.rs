use candid::{CandidType, Deserialize, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;
pub struct DIP20 {
    principal: Principal,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
}
pub type TxReceipt = Result<Nat, TxError>;

#[allow(non_snake_case)]
#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    pub amount: NumTokens,
    pub to_account: Account,
}

impl DIP20 {
    pub fn new(principal: Principal) -> Self {
        DIP20 { principal }
    }

    // pub async fn transfer(&self, target: Principal, amount: Nat) -> TxReceipt {
    //     let call_result: Result<(TxReceipt,), _> =
    //         ic_cdk::api::call::call(self.principal, "transfer", (target, amount)).await;

    //     call_result.unwrap().0
    // }

    // pub async fn transfer_from(
    //     &self,
    //     source: Principal,
    //     target: Principal,
    //     amount: Nat,
    // ) -> TxReceipt {
    //     let call_result: Result<(TxReceipt,), _> =
    //         ic_cdk::api::call::call(self.principal, "icrc2_transfer_from", (source, target, amount)).await;

    //     call_result.unwrap().0
    // }

pub async fn transfer(&self, args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()),
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        self.principal,
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

    pub async fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        let call_result: Result<(Nat,), _> =
            ic_cdk::api::call::call(self.principal, "icrc2_allowance", (owner, spender)).await;

        call_result.unwrap().0
    }

    pub async fn get_metadata(&self) -> Metadata {
        let call_result: Result<(Metadata,), _> =
            ic_cdk::api::call::call(self.principal, "getMetadata", ()).await;

        call_result.unwrap().0
    }
}