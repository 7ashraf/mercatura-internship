use ic_agent::{Agent, export::Principal};
use candid::{Encode, Decode, CandidType, Nat};
use serde::Deserialize;
use std::error::Error;
use ic_utils::canister::Canister;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //let some_account_id: Vec<u8> = vec![2ddeac815a971d820867ca38316d9d35c746f8e610c72001e4745ccc5ac87741];

    // let icp_ledger = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    // let response = agent.update(&icp_ledger, "account_balance")
    //     .with_arg(Encode!(&AccountBalanceArgs { account: some_account_id })?)
    //     .call_and_wait()
    //     .await?;
    // let tokens = Decode!(&response, Tokens)?;

    let some_account_id: Vec<u8> = vec![2, 221, 234, 200, 21, 169, 113, 216, 32, 134, 124, 163, 131, 22, 217, 211, 92, 116, 111, 142, 97, 12, 114, 0, 30, 71, 69, 204, 92, 200, 119, 65];
    let agent = create_agent("http://localhost:8000", false).await?;

    let canister = Canister::builder()
    .with_agent(&agent)
    .with_canister_id(Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap())
    .build()?;
    let response = canister.query_("account_balance", Encode!(&AccountBalanceArgs { account: some_account_id })?).await?;

    println!("Account balance: {:?}", response);
    Ok(())
    
}

pub async fn create_agent(url: &str, is_mainnet: bool) -> Result<Agent> {
    let agent = Agent::builder().with_url(url).build()?;
    if !is_mainnet {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}