use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

// BALANCE_LIFETIME_THRESHOLD determines how long a balance can remain in storage without being refreshed,
// and BALANCE_BUMP_AMOUNT determines how much the balance is increased when it's refreshed to maintain its validity.
// These constants are used to manage the expiration and renewal of balances in the storage system.

// function reads the balance and returns balance
pub fn read_balance(e: &Env, addr: Address) -> i128 {
    // this refers to the Balance by passing address
    let key = DataKey::Balance(addr);

    // the condition is Check if there's an existing balance associated with the provided key in persistent storage.
    //a turbofish operator (::<...>) used to provide generic type parameters to the get method. It specifies that we expect the returned data to be of type i128, and the key used to retrieve the data is of type DataKey.
    // &key is the reference to the specific key you want to retrieve data for, and DataKey is the type of key expected by the storage system to locate and retrieve the data.
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        // If a balance exists, extend its lifetime in storage
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        // extend_ttl is a function that refreshes or extends the validity of a key-value pair in the storage system.
        // It ensures that the data associated with the key remains accessible for a longer period by updating its expiration time based on the provided threshold and bump amount.
        //  The TTL is the amount of time that data is considered valid or alive. After this time period expires, the data is considered expired or stale.

        // return balance
        balance
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    // represents a balance associated with a particular address (addr)
    let key = DataKey::Balance(addr);

    // set the amount of specific key
    e.storage().persistent().set(&key, &amount);
    // Extend TTL again
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

// function to recieve balance
pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    // Reading balance
    let balance = read_balance(e, addr.clone());
    // updating balance
    write_balance(e, addr, balance + amount);

    //read_balance function:

    // This function is responsible for reading the balance associated with a given address from the storage system.
    // It abstracts away the details of how the balance is retrieved, such as accessing the storage system and handling TTL extensions.
    // It returns the balance after ensuring its validity by extending its TTL if necessary.

    // Creating the key using DataKey::Balance(addr):

    // This line creates a key that uniquely identifies the balance associated with the provided address (addr).
    // It's used to specify which data item to retrieve from the storage system when reading or writing the balance.
    // The key is passed to the storage system to locate and operate on the corresponding balance data.

    // BOTH CODE ARE SAME
    // let balance = read_balance(e, addr.clone());

    //AND

    // let key = DataKey::Balance(addr);
    // if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
    // Do something with the balance...
    // }
}

// function to spend balance
pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);

    /*
    same logic for write balance
    write_balance(e, addr, balance);
    */

    // let key = DataKey::Balance(addr);
    //e.storage().persistent().set(&key, &balance);
    // e.storage().persistent().extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
