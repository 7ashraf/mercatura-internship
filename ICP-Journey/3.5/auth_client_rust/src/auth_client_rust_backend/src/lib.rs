use candid::{ Principal};
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn whoami() -> String {
    ic_cdk::caller().to_string()
} 
