/*_______               .__                  _____ 
\______  \_____    _____|  |______________ _/ ____\
    /    /\__  \  /  ___/  |  \_  __ \__  \\   __\ 
   /    /  / __ \_\___ \|   Y  \  | \// __ \|  |   
  /____/  (____  /____  >___|  /__|  (____  /__|   
               \/     \/     \/           \/       
*/

use std::cell::RefCell;

use candid::{CandidType, Deserialize, Nat, Principal};
use std::collections::BTreeMap;
type Blob = Vec<u8>;

/// Auction item. Exported as Candid type.
#[derive(CandidType, Deserialize, Clone)]
struct Item {
    /// Auction title
    title: String,
    /// Auction description
    description: String,
    /// Image binary data, currently only PNG supported.
    image: Blob,
}

/// Auction bid. Exported as Candid type.
#[derive(CandidType, Deserialize, Clone)]
struct Bid {
    /// Price in the unit of the currency (ICP).
    price: Nat,
    /// Point in time of the bid, measured as the
    /// remaining until the closing of the auction.
    time: Nat,
    /// Authenticated user id of this bid.
    originator: Principal,
}

/// Auction identifier that is generated and associated
/// by the actor to later retrieve an auction.
/// Shared type.
type AuctionId = Nat;

/// Reduced information of an auction. Exported as Candid type.
#[derive(CandidType, Deserialize, Clone)]
struct AuctionOverview {
    /// Id associated to the auction serving for retrieval.
    id: AuctionId,
    /// Item sold in the auction.
    item: Item,
}

/// Detailed information of an auction. Exported as Candid type.
#[derive(CandidType, Deserialize, Clone)]
#[allow(non_snake_case)]
struct AuctionDetails {
    /// Item sold in the auction.
    item: Item,
    /// Series of valid bids in this auction, sorted by price.
    bidHistory: Vec<Bid>,
    /// Remaining time until the end of the auction.
    /// `0` means that the auction is closed.
    /// The last entry in `bidHistory`, if existing, denotes
    /// the auction winner.
    remainingTime: Nat,
}

/// Internal type, combining all information about an auction.
struct Auction {
    id: AuctionId,
    details: AuctionDetails,
}

type Auctions = BTreeMap<AuctionId, Auction>;

thread_local! {
    static AUCTIONS: RefCell<Auctions> = RefCell::default();
    static ID_COUNTER: RefCell<u64> = RefCell::new(0);
    //static TIMER: RefCell<Option<timerId: Nat>> = RefCell::new(None);
}

/// Install a recurring timer to close expired auctions.
#[ic_cdk::init]
fn init() {
    //let timer_id = ic_cdk::timer::set_timer_interval(std::time::Duration::from_secs(1), tick);
    //TIMER.with(|t| *t.borrow_mut() = Some(timer_id));
    // TODO: Implementation
}

fn tick() {
    AUCTIONS.with(|auctions| {
        let mut auctions = auctions.borrow_mut();
        for auction in auctions.values_mut() {
            if auction.details.remainingTime > Nat::from(0 as u64) {
                auction.details.remainingTime -= Nat::from(1 as u64);
            }
        }
    });
}

/// The timer needs to be reinstalled on canister upgrade.
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    init();
}

/// Register a new auction that is open for the defined duration.
#[ic_cdk::update]
#[allow(non_snake_case)]
fn newAuction(item: Item, duration: Nat) {
    // TODO: Implementation
    let id = generate_auction_id();
    let auction = Auction {
        id: id.clone(),
        details: AuctionDetails {
            item,
            bidHistory: Vec::new(),
            remainingTime: duration,
        },
    };
    AUCTIONS.with(|auctions| auctions.borrow_mut().insert(id, auction));
}
fn generate_auction_id() -> AuctionId {
    ID_COUNTER.with(|counter| {
        let mut id = counter.borrow_mut();
        *id += 1;
        Nat::from(*id)
    })
}

/// Retrieve all auctions (open and closed) with their ids and reduced overview information.
/// Specific auctions can be separately retrieved by `getAuctionDetail`.
#[ic_cdk::update]
#[allow(non_snake_case)]
fn getOverviewList() -> Vec<AuctionOverview> {
    // TODO: Implementation
    AUCTIONS.with(|auctions| {
        auctions.borrow().values().map(|auction| {
            AuctionOverview {
                id: auction.id.clone(),
                item: auction.details.item.clone(),
            }
        }).collect()
    })
}

/// Retrieve the detail information of auction by its id.
/// The returned detail contain status about whether the auction is active or closed,
/// and the bids make so far.
#[ic_cdk::update]
#[allow(non_snake_case)]
fn getAuctionDetails(auction_id: AuctionId) -> AuctionDetails {
    // TODO: Implementation
    AUCTIONS.with(|auctions| {
        auctions.borrow().get(&auction_id).map(|auction| auction.details.clone())
    }).expect("Auction not found")
}

/// Make a new bid for a specific auction specified by the id.
/// Checks that:
/// * The user (`ic_cdk::caller()`) is authenticated.
/// * The price is valid, higher than the last bid, if existing.
/// * The auction is still open (not finished).
/// If valid, the bid is appended to the bid history.
/// Otherwise, traps with an error.
#[ic_cdk::update]
#[allow(non_snake_case)]
fn makeBid(auction_id: AuctionId, price: Nat) {
    // TODO: Implementation
    let originator = ic_cdk::caller();

    if originator == Principal::anonymous() {
        ic_cdk::trap("Anonymous caller");
    }

    AUCTIONS.with(|auctions| {
        let mut auctions = auctions.borrow_mut();
        let auction = auctions.get_mut(&auction_id).expect("Auction not found");

        let last_bid_price = auction.details.bidHistory.last().map_or(Nat::from(1 as u64), |bid| bid.price.clone());
        if price <= last_bid_price {
            ic_cdk::trap("Price too low");
        }

        // if auction.details.remainingTime == Nat::from(0 as u64) {
        //     ic_cdk::trap("Auction closed");
        // }

        auction.details.bidHistory.push(Bid {
            price,
            time: auction.details.remainingTime.clone(),
            originator,
        });
    });
    
}
