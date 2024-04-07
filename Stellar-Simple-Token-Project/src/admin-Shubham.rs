// import Soroban SDK with its address and environment
use soroban_sdk::{Address, Env};

// import from crate == package name storage_types and from that enum named DataKey
use crate::storage_types::DataKey;

// Boolean function to check if contract has admintrator or not &Var is used to pass reference
pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

// Function to get administrator - it takes an instance of the Environment (&Env), returns admin address
pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
    // unwrap() is used to return the value as address
}

// Function to set new admin  - It takes two arguments : An instance of Environment (&Env) and a new admin address
// Write new admin into the smart contract's storage
pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
