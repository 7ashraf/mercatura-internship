/*_______               .__                  _____ 
\______  \_____    _____|  |______________ _/ ____\
    /    /\__  \  /  ___/  |  \_  __ \__  \\   __\ 
   /    /  / __ \_\___ \|   Y  \  | \// __ \|  |   
  /____/  (____  /____  >___|  /__|  (____  /__|   
               \/     \/     \/           \/       
*/

#[macro_use]
extern crate ic_cdk_macros;

#[macro_use]
extern crate serde;

use std::cell::RefCell;
use candid::{ CandidType, Deserialize, Int, Nat};
use candid::{candid_method, export_service, Principal};
use ic_cdk::caller;
mod dip20;
use dip20::DIP20;
use std::collections::BTreeMap;
use std::borrow::BorrowMut;
type Balances_A = BTreeMap<Principal, Nat>;
type Balances_B = BTreeMap<Principal, Nat>;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

type tokenAdresses = BTreeMap<Nat, Principal>;


#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
struct init_args {
    token_a: Principal,
    token_b: Principal,
}


// struct state {
//     token_a: Principal,
//     token_b: Principal,
//     balances_a: HashMap<Principal, Nat>,
//     balances_b: HashMap<Principal, Nat>,
// }
#[derive(CandidType, Deserialize)]
struct swap_args {
    user_a: Principal,
    user_b: Principal,
}

#[ic_cdk::update]
pub fn init(args: init_args) {

    // TOKEN_A.with(|token_a| *token_a.borrow_mut() = args.token_a);
    // TOKEN_B.with(|token_b| *token_b.borrow_mut() = args.token_b);

    TOKEN_ADDRESSES.with(|token_addresses| {
        let mut token_addresses = token_addresses.borrow_mut();
        token_addresses.insert(Nat::from(0 as u32), args.token_a);
        token_addresses.insert(Nat::from(1 as u32), args.token_b);
    });
}


//define storage for tokens
thread_local! {
    static BALANCES_A: RefCell<Balances_A>=RefCell::default();
    static BALANCES_B: RefCell<Balances_B>=RefCell::default();
    // static TOKEN_A: RefCell<Principal> = RefCell::default();
    // static TOKEN_B: RefCell<Principal> = RefCell::default();
    static TOKEN_ADDRESSES: RefCell<tokenAdresses> = RefCell::default();
}
#[ic_cdk::update]
pub async fn deposit_token_A() -> Result<Nat, TransferFromError> {
    let caller = ic_cdk::caller();
    let token = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(0 as u32)).unwrap().clone()
    });
    let token = DIP20::new(token);
    //let dip_fee = token.get_metadata().await.fee;

    //let allowance = token.allowance(caller, ic_cdk::api::id()).await;

    //let available = allowance - dip_fee;
    let available = Nat::from(1 as u32);
    //let available = allowance.clone();

    let args = dip20::TransferArgs {
        amount: available.clone(),
        to_account: Account::from(ic_cdk::api::id()),
    };


    token
        .transfer(args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));
        

        BALANCES_A.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(caller).or_insert(Nat::from(0 as u32));
        *balance += available.clone();
    });

    

    Ok(available)
}

#[ic_cdk::update]

pub async fn deposit_token_B() -> Result<Nat, TransferFromError> {
    let caller = ic_cdk::caller();
    let token = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(1 as u32)).unwrap().clone()
    });
    let token = DIP20::new(token);
    //let dip_fee = token.get_metadata().await.fee;

    //let allowance = token.allowance(caller, ic_cdk::api::id()).await;

    //let available = allowance - dip_fee;
    let available = Nat::from(1 as u32);
    //let available = allowance.clone();

    let args = dip20::TransferArgs {
        amount: available.clone(),
        to_account: Account::from(ic_cdk::api::id()),
    };


    token
        .transfer(args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));
        

        BALANCES_B.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(caller).or_insert(Nat::from(0 as u32));
        *balance += available.clone();
    });

    

    Ok(available)
}
#[ic_cdk::update]
async fn withdraw_token_A(amount: Nat) -> Result<Nat, DepositErr> {
    let caller = ic_cdk::caller();
    BALANCES_A.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(caller).or_insert(Nat::from(0 as u32));
        if *balance < amount {
            //return Err(DepositErr::BalanceLow);
        }
        *balance -= amount.clone();
        
    });

    let token = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(0 as u32)).unwrap().clone()
    });
    let token = DIP20::new(token);

    let args = dip20::TransferArgs {
        amount: amount.clone(),
        to_account: Account::from(caller),
    };

    token
        .transfer(args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));

    Ok(amount)
}

#[ic_cdk::update]
async fn withdraw_token_B(amount: Nat) -> Result<Nat, DepositErr> {
    let caller = ic_cdk::caller();
    BALANCES_B.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(caller).or_insert(Nat::from(0 as u32));
        if *balance < amount {
            //return Err(DepositErr::BalanceLow);
        }
        *balance -= amount.clone();
        
    });

    let token = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(1 as u32)).unwrap().clone()
    });
    let token = DIP20::new(token);

    let args = dip20::TransferArgs {
        amount: amount.clone(),
        to_account: Account::from(caller),
    };

    token
        .transfer(args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));

    Ok(amount)
}

//withdraw function with specifid token

//swap function between two users
#[ic_cdk::update]
fn swap(user_a: Principal, user_b: Principal) {
    //get balances of user_a and user_b
    let balance_a = BALANCES_A.with(|balances| {
        let balances = balances.borrow();
        balances.get(&user_a).unwrap().clone()
    });
    let balance_b = BALANCES_B.with(|balances| {
        let balances = balances.borrow();
        balances.get(&user_b).unwrap().clone()
    });

    //get token addresses
    let token_a = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(0 as u32)).unwrap().clone()
    });
    let token_b = TOKEN_ADDRESSES.with(|token_addresses| {
        let token_addresses = token_addresses.borrow();
        token_addresses.get(&Nat::from(1 as u32)).unwrap().clone()
    });

    //get token instances
    let token_a = DIP20::new(token_a);
    let token_b = DIP20::new(token_b);

    let balance_a = BALANCES_A.with(|balances| {
        let balances = balances.borrow();
        balances.get(&user_a).unwrap().clone()
    });

    let balance_b = BALANCES_B.with(|balances| {
        let balances = balances.borrow();
        balances.get(&user_b).unwrap().clone()
    });

    

    //swap tokens
    BALANCES_A.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(user_b).or_insert(Nat::from(0 as u32));
        *balance += balance_a.clone();
    });



    BALANCES_B.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(user_a).or_insert(Nat::from(0 as u32));
        *balance += balance_b.clone();
    });



    //delete balances
    BALANCES_A.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(user_a).or_insert(Nat::from(0 as u32));
        *balance -= balance_a.clone();
    });

    BALANCES_B.with(|balances| {
        let mut balances = balances.borrow_mut();
        let balance = balances.entry(user_b).or_insert(Nat::from(0 as u32));
        *balance -= balance_b.clone();
    });
    
}

#[ic_cdk::query]
fn get_balances() -> (Balances_A, Balances_B) {
    let balances_a = BALANCES_A.with(|balances| balances.borrow().clone());
    let balances_b = BALANCES_B.with(|balances| balances.borrow().clone());
    (balances_a, balances_b)
}



#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,

}
ic_cdk::export_candid!();