use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::time, caller};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

// Enhanced NFT structure with additional fields
#[derive(CandidType, Deserialize, Clone, Serialize)]
struct Nft {
    token_id: u64,
    metadata: String,
    uri: Option<String>,
    creator: Principal,
    created_at: u64,
    balances: HashMap<Principal, u64>, // ERC-1155 style ownership
}

// Approval structure for delegated transfers
#[derive(CandidType, Deserialize, Clone, Default, Serialize)]
struct Approvals {
    // Owner -> Operator -> Approved
    operator_approvals: HashMap<Principal, HashSet<Principal>>,
}

// Event structure for logging
#[derive(CandidType, Deserialize, Clone, Serialize)]
enum Event {
    Mint {
        token_id: u64,
        owner: Principal,
        amount: u64,
        timestamp: u64,
    },
    Transfer {
        token_id: u64,
        from: Principal,
        to: Principal,
        amount: u64,
        timestamp: u64,
    },
    Burn {
        token_id: u64,
        owner: Principal,
        amount: u64,
        timestamp: u64,
    },
    ApprovalForAll {
        owner: Principal,
        operator: Principal,
        approved: bool,
        timestamp: u64,
    },
}

// Canister state using thread_local
thread_local! {
    static NFTS: RefCell<HashMap<u64, Nft>> = RefCell::new(HashMap::new());
    static APPROVALS: RefCell<Approvals> = RefCell::new(Approvals::default());
    static EVENTS: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
    static OWNER: RefCell<Principal> = const { RefCell::new(Principal::anonymous()) };
}

// Initialize the canister
#[ic_cdk::init]
fn init() {
    let owner = caller();
    OWNER.with(|o| *o.borrow_mut() = owner);
}

// Helper function to get the current time
fn now() -> u64 {
    time()
}

// Helper function to check if caller is approved for an owner
fn is_approved_for_all(owner: &Principal, operator: &Principal) -> bool {
    APPROVALS.with(|approvals| {
        approvals
            .borrow()
            .operator_approvals
            .get(owner)
            .map_or(false, |ops| ops.contains(operator))
    })
}

// Helper function to check if caller can transfer tokens
fn can_transfer(from: &Principal) -> bool {
    let caller = caller();
    &caller == from || is_approved_for_all(from, &caller)
}

// Original mint function with enhanced functionality
#[ic_cdk::update]
fn mint(token_id: u64, metadata: String, owner: Principal, amount: u64) {
    let caller = caller();
    let timestamp = now();

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        let nft = nfts.entry(token_id).or_insert(Nft {
            token_id,
            metadata,
            uri: None,
            creator: caller,
            created_at: timestamp,
            balances: HashMap::new(),
        });

        // Update the balance
        *nft.balances.entry(owner).or_insert(0) += amount;
    });

    // Log the event
    EVENTS.with(|events| {
        events.borrow_mut().push(Event::Mint {
            token_id,
            owner,
            amount,
            timestamp,
        });
    });
}

// Enhanced mint function with URI
#[ic_cdk::update]
fn mint_with_uri(token_id: u64, metadata: String, uri: String, owner: Principal, amount: u64) {
    let caller = caller();
    let timestamp = now();

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        let nft = nfts.entry(token_id).or_insert(Nft {
            token_id,
            metadata,
            uri: Some(uri),
            creator: caller,
            created_at: timestamp,
            balances: HashMap::new(),
        });

        // Update the balance
        *nft.balances.entry(owner).or_insert(0) += amount;
    });

    // Log the event
    EVENTS.with(|events| {
        events.borrow_mut().push(Event::Mint {
            token_id,
            owner,
            amount,
            timestamp,
        });
    });
}

// Original balance_of function
#[ic_cdk::query]
fn balance_of(owner: Principal, token_id: u64) -> u64 {
    NFTS.with(|nfts| {
        nfts.borrow()
            .get(&token_id)
            .and_then(|nft| nft.balances.get(&owner))
            .copied()
            .unwrap_or(0)
    })
}

// Batch balance query
#[ic_cdk::query]
fn batch_balance_of(owner: Principal, token_ids: Vec<u64>) -> Vec<u64> {
    NFTS.with(|nfts| {
        let nfts = nfts.borrow();
        token_ids
            .iter()
            .map(|id| {
                nfts.get(id)
                    .and_then(|nft| nft.balances.get(&owner))
                    .copied()
                    .unwrap_or(0)
            })
            .collect()
    })
}

// Enhanced transfer function with authorization check
#[ic_cdk::update]
fn transfer(token_id: u64, from: Principal, to: Principal, amount: u64) -> Result<(), String> {
    // Check if caller is authorized to transfer
    if !can_transfer(&from) {
        return Err("Not authorized to transfer".to_string());
    }

    let timestamp = now();
    let mut success = false;

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&token_id) {
            let from_balance = nft.balances.entry(from).or_insert(0);
            if *from_balance >= amount {
                *from_balance -= amount;
                *nft.balances.entry(to).or_insert(0) += amount;
                success = true;
            }
        }
    });

    if !success {
        return Err("Transfer failed: Token ID does not exist or insufficient balance".to_string());
    }

    // Log the event
    EVENTS.with(|events| {
        events.borrow_mut().push(Event::Transfer {
            token_id,
            from,
            to,
            amount,
            timestamp,
        });
    });

    Ok(())
}

// Batch transfer function
#[ic_cdk::update]
fn batch_transfer(
    token_ids: Vec<u64>,
    from: Principal,
    to: Principal,
    amounts: Vec<u64>,
) -> Result<(), String> {
    if token_ids.len() != amounts.len() {
        return Err("Token IDs and amounts length mismatch".to_string());
    }

    // Check if caller is authorized to transfer
    if !can_transfer(&from) {
        return Err("Not authorized to transfer".to_string());
    }

    for i in 0..token_ids.len() {
        transfer(token_ids[i], from, to, amounts[i])?;
    }

    Ok(())
}

// Burn tokens
#[ic_cdk::update]
fn burn(token_id: u64, owner: Principal, amount: u64) -> Result<(), String> {
    // Check if caller is authorized to burn
    if !can_transfer(&owner) {
        return Err("Not authorized to burn".to_string());
    }

    let timestamp = now();
    let mut success = false;

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&token_id) {
            let owner_balance = nft.balances.entry(owner).or_insert(0);
            if *owner_balance >= amount {
                *owner_balance -= amount;
                success = true;
            }
        }
    });

    if !success {
        return Err("Burn failed: Token ID does not exist or insufficient balance".to_string());
    }

    // Log the event
    EVENTS.with(|events| {
        events.borrow_mut().push(Event::Burn {
            token_id,
            owner,
            amount,
            timestamp,
        });
    });

    Ok(())
}

// Set approval for all
#[ic_cdk::update]
fn set_approval_for_all(operator: Principal, approved: bool) -> Result<(), String> {
    let owner = caller();
    let timestamp = now();

    APPROVALS.with(|approvals| {
        let mut approvals = approvals.borrow_mut();
        let operators = approvals.operator_approvals.entry(owner).or_default();

        if approved {
            operators.insert(operator);
        } else {
            operators.remove(&operator);
        }
    });

    // Log the event
    EVENTS.with(|events| {
        events.borrow_mut().push(Event::ApprovalForAll {
            owner,
            operator,
            approved,
            timestamp,
        });
    });

    Ok(())
}

// Check if an operator is approved for all of an owner's tokens
#[ic_cdk::query]
fn is_approved_for_all_query(owner: Principal, operator: Principal) -> bool {
    is_approved_for_all(&owner, &operator)
}

// Get token metadata
#[ic_cdk::query]
fn get_metadata(token_id: u64) -> Result<String, String> {
    NFTS.with(|nfts| {
        nfts.borrow()
            .get(&token_id)
            .map(|nft| nft.metadata.clone())
            .ok_or("Token ID does not exist".to_string())
    })
}

// Get token URI
#[ic_cdk::query]
fn token_uri(token_id: u64) -> Result<Option<String>, String> {
    NFTS.with(|nfts| {
        nfts.borrow()
            .get(&token_id)
            .map(|nft| nft.uri.clone())
            .ok_or("Token ID does not exist".to_string())
    })
}

// Update token metadata (only by creator)
#[ic_cdk::update]
fn update_metadata(token_id: u64, metadata: String) -> Result<(), String> {
    let caller = caller();
    let mut success = false;

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&token_id) {
            if nft.creator == caller {
                nft.metadata = metadata;
                success = true;
            }
        }
    });

    if !success {
        return Err("Update failed: Token ID does not exist or not authorized".to_string());
    }

    Ok(())
}

// Update token URI (only by creator)
#[ic_cdk::update]
fn update_uri(token_id: u64, uri: String) -> Result<(), String> {
    let caller = caller();
    let mut success = false;

    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&token_id) {
            if nft.creator == caller {
                nft.uri = Some(uri);
                success = true;
            }
        }
    });

    if !success {
        return Err("Update failed: Token ID does not exist or not authorized".to_string());
    }

    Ok(())
}

// Get all tokens owned by an address
#[ic_cdk::query]
fn tokens_of_owner(owner: Principal) -> Vec<(u64, u64)> {
    NFTS.with(|nfts| {
        let nfts = nfts.borrow();
        let mut result = Vec::new();

        for (token_id, nft) in nfts.iter() {
            if let Some(balance) = nft.balances.get(&owner) {
                if *balance > 0 {
                    result.push((*token_id, *balance));
                }
            }
        }

        result
    })
}

// Get recent events (limited to last n events)
#[ic_cdk::query]
fn get_events(limit: usize) -> Vec<Event> {
    EVENTS.with(|events| {
        let events = events.borrow();
        let start = if events.len() > limit {
            events.len() - limit
        } else {
            0
        };
        events[start..].to_vec()
    })
}

// Pre-upgrade hook to save state
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let nfts = NFTS.with(|nfts| nfts.borrow().clone());
    let approvals = APPROVALS.with(|approvals| approvals.borrow().clone());
    let events = EVENTS.with(|events| events.borrow().clone());
    let owner = OWNER.with(|owner| *owner.borrow());

    match ic_cdk::storage::stable_save((nfts, approvals, events, owner)) {
        Ok(_) => (),
        Err(e) => ic_cdk::trap(&format!("Failed to save state: {:?}", e)),
    }
}

// Post-upgrade hook to restore state
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    match ic_cdk::storage::stable_restore::<(HashMap<u64, Nft>, Approvals, Vec<Event>, Principal)>()
    {
        Ok((nfts, approvals, events, owner)) => {
            NFTS.with(|n| *n.borrow_mut() = nfts);
            APPROVALS.with(|a| *a.borrow_mut() = approvals);
            EVENTS.with(|e| *e.borrow_mut() = events);
            OWNER.with(|o| *o.borrow_mut() = owner);
        }
        Err(e) => ic_cdk::trap(&format!("Failed to restore state: {:?}", e)),
    }
}

// Export Candid interface
ic_cdk::export_candid!();
