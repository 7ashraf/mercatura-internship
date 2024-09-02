
#[macro_use]
extern crate ic_cdk_macros;

#[macro_use]
extern crate serde;

use std::cell::RefCell;
use candid::{CandidType, Encode, Principal};
use std::collections::HashMap;
use ic_cdk::init;
use std::collections::BTreeMap;

#[derive(CandidType, Deserialize)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Ord)]
#[derive(PartialOrd)]
enum Role {
    Owner, 
    Admin,
    Authorized,
}
#[derive(CandidType, Deserialize)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Ord)]
#[derive(Eq)]
#[derive(PartialOrd)]
enum Permission{
    Assign_Role, 
    Lowest,
}
#[derive(CandidType, Deserialize)]
#[derive(Default)]
struct State {
    role_request: HashMap<Principal, Role>,
    roles: HashMap<Principal, Role>,
    permissions: HashMap<Role, Permission>,
}

type RolesStore = BTreeMap<Principal, Role>;
type PermissionsStore = BTreeMap<Role, Permission>;


thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static ROLES_STORE: RefCell<RolesStore> = RefCell::default();
    static PERMISSIONS_STORE: RefCell<PermissionsStore> = RefCell::default();

}

#[ic_cdk::update]
fn init() {
    // STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    //     println!("Initializing access control system");
    //     state.permissions.insert(Role::Owner, Permission::Assign_Role);
    //     state.permissions.insert(Role::Admin, Permission::Assign_Role);
    //     state.permissions.insert(Role::Authorized, Permission::Lowest);
    //     println!("{:?}", state.permissions);
    //     state.roles.insert(owner, Role::Owner);
        
    // });

    // PERMISSIONS_STORE.with(|permissions| {
    //     permissions
    //     .borrow_mut()
    //     .insert(Role::Owner, Permission::Assign_Role);
    //     .insert(Role::Admin, Permission::Assign_Role);
    //     .insert(Role::Authorized, Permission::Lowest);
    // });

    PERMISSIONS_STORE.with(|permissions| {
        let mut permissions = permissions.borrow_mut();
        println!("Initializing access control system");
        permissions.insert(Role::Owner, Permission::Assign_Role);
        permissions.insert(Role::Admin, Permission::Assign_Role);
        permissions.insert(Role::Authorized, Permission::Lowest);
    });

    let owner: Principal = ic_cdk::caller();

    ROLES_STORE.with(|roles| {
        let mut roles = roles.borrow_mut();
        println!("Initializing access control system");
        roles.insert(owner, Role::Owner);
    });

}
#[ic_cdk::query]
fn greet(name: String) -> String {
    let caller = ic_cdk::caller();
    // let role = STATE.with(|state| {
    //     let state = state.borrow();
    //     state.roles.get(&caller).unwrap().clone();
    //     match state.roles.get(&caller) {
    //         Some(&ref role) => role.clone(),
    //         None => Role::Authorized,
    //     }
    // });

    let role = ROLES_STORE.with(|roles| {
        roles
        .borrow()
        .get(&caller)
        .cloned()
        .unwrap()
        
    });

    // let permission = STATE.with(|state| {
    //     let state = state.borrow();
    //     match state.permissions.get(&role) {
    //         Some(&permission) => permission,
    //         None => Permission::Lowest,
    //     }
    // });

    let permission = PERMISSIONS_STORE.with(|permissions| {
        permissions
        .borrow()
        .get(&role)
        .cloned()
        .unwrap()
    });

    let message = match permission {
        (Permission::Assign_Role) => format!("Hello, {}! You have a role with administrative privileges. ", name),
        (Permission::Lowest) => format!("Hello, {}! You have an authorized account. Would you like to play a game?", name),
        _ => "You do not have permission to greet".to_string(),
    };

    format!("Hello, {}!", message)
}

#[ic_cdk::query]
fn get_role() -> Role {
    let caller = ic_cdk::caller();
    // STATE.with(|state| {
    //     let state = state.borrow();
    //     match state.roles.get(&caller) {
    //         Some(&ref role) => role.clone(),
    //         None => Role::Authorized,
    //     }
    // })
    
    ROLES_STORE.with(|roles| {
        roles
        .borrow()
        .get(&caller)
        .cloned()
        .unwrap()
    })
}

#[ic_cdk::query]

fn has_permission(role: Role, permission: Permission) -> bool {
    // STATE.with(|state| {
    //     let state = state.borrow();
    //     state.permissions.get(&role).unwrap() == &permission
    // })
    PERMISSIONS_STORE.with(|permissions| {
        permissions
        .borrow()
        .get(&role)
        .cloned()
        .unwrap() == permission
    })
}

fn require_permission(permission: Permission) {
    let caller = ic_cdk::caller();
    // let role = STATE.with(|state| {
    //     let state = state.borrow();
    //     state.roles.get(&caller).unwrap().clone();
    //     match state.roles.get(&caller) {
    //         Some(&ref role) => role.clone(),
    //         None => Role::Authorized,
    //     }
    // });

    let role = ROLES_STORE.with(|roles| {
        roles
        .borrow()
        .get(&caller)
        .cloned()
        .unwrap()
    });

    if !has_permission(role, permission) {
        ic_cdk::trap("Permission denied");
    }
}

#[ic_cdk::update]
fn assign_role(principal: Principal, role: Role) {
    require_permission(Permission::Assign_Role);
    // STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    //     state.roles.insert(principal, role);
    // });
    ROLES_STORE.with(|roles| {
        roles
        .borrow_mut()
        .insert(principal, role);
    });
}

#[ic_cdk::query]
fn my_role() -> Role {
    let caller = ic_cdk::caller();
    // STATE.with(|state| {
    //     let state = state.borrow();
    //     state.roles.get(&caller).unwrap().clone();
    //     match state.roles.get(&caller) {
    //         Some(&ref role) => role.clone(),
    //         None => Role::Authorized,
    //     }
    // })
    ROLES_STORE.with(|roles| {
        roles
        .borrow()
        .get(&caller)
        .cloned()
        .unwrap_or(Role::Authorized)
    })
}


#[ic_cdk::update]
fn set_owner(){
    let caller = ic_cdk::caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.roles.insert(caller, Role::Owner);
    });
}

ic_cdk::export_candid!();