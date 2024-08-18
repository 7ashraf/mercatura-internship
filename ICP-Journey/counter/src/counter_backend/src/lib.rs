use std::cell::RefCell;
use candid::types::number::Nat;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

// #[ic_cdk::update]
// fn increment() {
//     COUNTER.with(|counter| *counter.borrow_mut() += 1);
// }
#[ic_cdk::update]
fn increment() {
    COUNTER.with(|counter| *counter.borrow_mut() += 3);
}
/// Get the value of the counter.
#[ic_cdk::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


