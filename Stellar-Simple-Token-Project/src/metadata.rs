use soroban_sdk::{Env, String};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

pub fn read_decimal(e: &Env) -> u32 {
    // ::new() is a conventional Rust syntax for calling a constructor or associated function of a type. It creates a new instance of TokenUtils.
    // creates a new instance of TokenUtils by calling its new associated function, passing an Env object e.
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().decimal
}

pub fn read_name(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().name
}

pub fn read_symbol(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().symbol
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let util = TokenUtils::new(e);
    // &metadata passes a reference to the metadata parameter to the set_metadata method. This means that the metadata parameter is not moved or consumed by the method, allowing it to be reused after the call.
    // update or set the metadata of a token using the TokenUtils instance util. It retrieves the metadata object associated with the token, then calls the set_metadata method to update it with the provided metadata.
    util.metadata().set_metadata(&metadata);
}
