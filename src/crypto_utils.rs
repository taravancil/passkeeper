extern crate crypto;
extern crate rand;

use self::crypto::scrypt::{ ScryptParams, scrypt };
use self::rand::{thread_rng, Rng};

/// Returns 32 random bytes
pub fn gen_nonce() -> [u8;32] {
    let mut salt = [0u8; 32];
    thread_rng().fill_bytes(&mut salt);
    salt
}

/// Returns default scrypt params
/// TODO: allow user to control these (within reason)?
pub fn get_scrypt_params() -> ScryptParams {
    let log_n: u8 = 5;
    let p: u32 = 8;
    let r: u32 = 1;

    ScryptParams::new(log_n, p, r)
}

/// Derives a private key from `password` with scrypt, returns resulting key
pub fn get_master_key(password: Vec<u8>, salt: [u8;32]) -> [u8;32] {
    let params = get_scrypt_params();
    let mut result = [0u8;32];
    scrypt(&password, &salt, &params, &mut result);

    result
}
