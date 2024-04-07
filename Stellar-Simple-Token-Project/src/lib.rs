// this states that we does not using any standard library
#![no_std]

// all modules listed in project
mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;
mod test;

pub use crate::contract::TokenClient;
