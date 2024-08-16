#[macro_use]
extern crate serde;

use candid::{Decode, Deserialize, Encode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::collections::BTreeMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
type VotesStore = BTreeMap<String, u64>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static VOTES_STORAGE: RefCell<VotesStore> = RefCell::default();

}

#[ic_cdk::update]
fn cast_vote(vote: String) -> Option<u64> {
    VOTES_STORAGE.with(|votes| {
        let mut votes = votes.borrow_mut();
        let count = votes.entry(vote).or_insert(0);
        *count += 1;
        Some(*count)
    })
}

#[ic_cdk::query]
fn get_votes() -> VotesStore {
    VOTES_STORAGE.with(|votes| votes.borrow().clone())
}

ic_cdk::export_candid!();