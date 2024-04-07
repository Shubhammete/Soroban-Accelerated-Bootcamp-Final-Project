//! This contract demonstrates a sample implementation of the Soroban token
//! interface.

// imported modules and their functions
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

// soroban sdk imports
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

// function to check if an account has positive balance
fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

// here contract attribute applied to a struct 
#[contract]
pub struct Token;


// attribute applied to the implementation block.
#[contractimpl]

// impl we cannot declare functions directly so we create impl and then attach it to struct
impl Token {
    // function to initialize the admin
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        // check if already initialized
        if has_administrator(&e) {
            panic!("already initialized")
        }
        // if not then initialize
        write_administrator(&e, &admin);

        // check if value is less than maximum value of u8 and .into() to convert it into other type like deimal here
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        // write metadata
        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        // check if amount is positive
        check_nonnegative_amount(amount);
        // get admin
        let admin = read_administrator(&e);
        admin.require_auth(); // it is function in soroban sdk to auth the admin

        // we extend TTL
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // then receive the balance
        receive_balance(&e, to.clone(), amount);
        // we trigger the mint function with TokenUtils instance
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    // to set new admin
    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();  

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // update admin
        write_administrator(&e, &new_admin);
        // invoke function with TokenUtils instance
        TokenUtils::new(&e).events().set_admin(admin, new_admin);
    }
}

#[contractimpl]
// here we are implementing token::Interface as trait
impl token::Interface for Token {
    // allowance  returns how much spender can use from owner account
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        // extend TTL
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        // return read allowance functions amount
        read_allowance(&e, from, spender).amount
    }

    // aprove transcation
    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        // auth the user
        from.require_auth();
        // check for elgible amount
        check_nonnegative_amount(amount);

        // extend we extend TTL
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // write allowance function called
        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        // here we are invoking approve function with TokenUtils instance
        TokenUtils::new(&e).events().approve(from, spender, amount, expiration_ledger);
    }

    // check balance of user
    fn balance(e: Env, id: Address) -> i128 {
        // extend TTL
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        // invoke read balance
        read_balance(&e, id)
    }

    // transfer token
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // reduce amount from sender
        spend_balance(&e, from.clone(), amount);
        // add amount to  recipient
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // reduce allowance from sender
        spend_allowance(&e, from.clone(), spender, amount);
        // reduce amount from sender
        spend_balance(&e, from.clone(), amount);
        // add amount to  recipient
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // reduce amount from sender
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }

    // returns the decimals, name, and symbol of the token, respectively.
    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
