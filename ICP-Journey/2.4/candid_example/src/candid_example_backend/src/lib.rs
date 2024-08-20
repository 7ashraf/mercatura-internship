use std::cell::RefCell;
use candid::types::number::Nat;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0 as u64));
}


#[ic_cdk::update]
fn add(d: Nat) {
    COUNTER.with(|counter| *counter.borrow_mut() += d);
}

#[ic_cdk::update]
fn subtract(d: Nat) {
    COUNTER.with(|counter| *counter.borrow_mut() -= d);
}
/// Get the value of the counter.
#[ic_cdk::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}



