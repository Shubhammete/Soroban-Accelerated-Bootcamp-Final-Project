use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use soroban_sdk::{Address, Env};

// function to read the allowance value it takes  two arguments: owner and spender. It returns an Allowance value
pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    // here key is accesing the AllowanceDataKey from Allowance variant of DataKey Enum
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    // here if Some is used to do pattern matching so if allownace is present and is equal to AllowanceValue in Environment then execute the block
    // Here in Temperory storage we pass get function with two parameters
    // the first one (_) is for Rust's type inference system to figure out, and the second one is explicitly specifying that the return type should be AllowanceValue. Finally, it passes a reference to key as an argument to the get method, which is likely used to retrieve some data from the storage based on that key.
    // allownace here is AllowanceValue
    // if condition fails we create new AllowanceValue with amount 0 and expiration_ledger 0
    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        
        // here condition passes then check if if expiration_legder is less than env ledger sequence
        // If the allowance expiration is before the current blockchain state, it means the permission is no longer valid and has expired.
        if allowance.expiration_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

// function to write allowance
pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    // explicity create new allowance of type AllowanceValue
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    // show error when amount is positive but ledger sequence is less than env one
    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    // here we are setting allowance by passing key clone to not transfer the key ownership and update the value associated with the key in the storage system to the value of allowance.
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &allowance);

    // if amount is eligible then
    if amount > 0 {
        // calculates the duration for which the allowance will remain valid. It subtracts the current ledger sequence number (e.ledger().sequence()) from the expiration_ledger value to get the difference.
        // The checked_sub method is used to perform the subtraction, and unwrap is called to extract the result. This is assuming that expiration_ledger is greater than or equal to the current ledger sequence number, otherwise it would panic
        let live_for = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap();
        // here it extends the time-to-live (TTL) of the key in the temporary storage. It likely means that the allowance is being extended for the duration specified by live_for. The extend_ttl method is used to update the TTL of the key. The first argument is a reference to the key, the second argument is the new TTL, and the third argument is a backup TTL in case of error.
        e.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
}


// spend allowance is to spend the allowance
pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    // here we use read allowance to get the allowance
    let allowance = read_allowance(e, from.clone(), spender.clone());
    // if spending amount is more than owning amounthen it throw error 
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    // if all good then update new allowance
    write_allowance(
        e,
        from,
        spender,
        allowance.amount - amount,   // update the amount
        allowance.expiration_ledger,
    );
}
