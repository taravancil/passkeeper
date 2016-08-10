// std
use std::collections::HashMap;

// external
extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;
use self::sodiumoxide::crypto::{ box_, secretbox };

// local
use crypto_utils;
use io;

// Struct for storing key data in $HOME/.passkeeper/data
#[derive(Deserialize, Serialize, Debug)]
pub struct KeyData {
    pub master_key_salt: [u8;32],
    pub sealed_master_privkey: Vec<u8>,
    pub sealed_master_privkey_salt: [u8;24],
    pub master_pubkey: [u8;32],
}

// Struct for storing a site's secret in $HOME/.passkeeper/vault
#[derive(Deserialize, Serialize, Debug)]
pub struct SiteData {
    pub pubkey: [u8;32],
    pub sealed_password: Vec<u8>,
    pub sealed_password_salt: [u8;24],
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vault {
    pub sites: HashMap<String, SiteData>,
}

/// Returns true if $HOME/.passkeeper/ contains a valid Vault and KeyData
pub fn is_initialized() -> bool {
    let vault = io::get_vault();
    let key_data = io::get_key_data();

    vault.is_ok() && key_data.is_ok()
}

/// Initializes passkeeper
pub fn init() {
    if is_initialized() {
        println!("passkeeper is already initialized");
        return
    }

    // Prompt the user for a master password; require verification
    let master_password = match io::prompt_password("Enter a master password", true) {
        Ok(password) => password,
        Err(err) => {
           println!("Error: {}", err.to_string());
           return
        }
    };

    // Create the $HOME/.passkeeper directory
    if io::create_passkeeper_dir().is_err() {
        println!("Error: couldn't make passkeeper directory");
        return
    }

    // Create $HOME/.passkeeper/vault
    if io::create_vault().is_err() {
        println!("Error: couldn't create vault file");
        return
    }

    // Create $HOME/.passkeeper/data
    if io::create_key_data_file().is_err() {
        println!("Error: couldn't create key data file");
        return
    }

    println!("Generating and encrypting your keys...");

    // Derive master key from master password
    let master_key_salt = crypto_utils::gen_nonce();
    let master_key = crypto_utils::get_master_key(
        master_password.into_bytes(),
        master_key_salt);

    // Generate master public and private keypair
    let (pubkey, privkey) = box_::gen_keypair();

    // Encrypt master private key
    let sealed_master_privkey_salt = secretbox::gen_nonce();
    let sealed_master_privkey = secretbox::seal(
        &privkey.0,
        &sealed_master_privkey_salt,
        &secretbox::Key(master_key));

    // Serialize KeyData
    let key_data = KeyData {
        sealed_master_privkey: sealed_master_privkey,
        sealed_master_privkey_salt: sealed_master_privkey_salt.0,
        master_key_salt: master_key_salt,
        master_pubkey: pubkey.0,
    };
    let serialized = serde_json::to_string(&key_data).unwrap();

    // Write key data to $HOME/.passkeeper/data
    let path = io::get_key_data_path();
    if io::write(serialized.as_bytes(), path).is_err() {
        println!("Error: couldn't write key data");
        return
    }
}

/// Prints the site names in the vault
pub fn list() {
    let vault = io::get_vault().unwrap();

    println!("Sites in your vault:");
    for (site, _) in vault.sites {
        println!("{}", site);
    }
}

pub fn add(site: &str) {
    // Verify that passkeeper is initialized
    if !is_initialized() {
        io::prompt_init();
        return
    };

    let mut vault = io::get_vault().unwrap();

    // Fail if the site is already in the vault
    if vault.sites.contains_key(site) {
        println!("A password for {} is already in your vault", &site);
        return
    }

    // Prompt the user for the site password; require verification
    let prompt = format!("Enter a password for {}", site);
    let password =  match io::prompt_password(&prompt, true) {
        Ok(password) => password,
        Err(err) => {
            println!("Error: {}", err.to_string());
            return
        }
    };

    // Generate a keypair for the site
    let (pubkey, privkey) = box_::gen_keypair();

    // Get the master pubkey from the key data store
    let key_data = io::get_key_data().unwrap();
    let master_pubkey = key_data.master_pubkey;

    // Seal the site's password
    let salt = box_::gen_nonce();
    let sealed_password = box_::seal(
        password.as_bytes(),
        &salt,
        &box_::PublicKey(master_pubkey),
        &privkey);

    // Serialize the site's info and add it to the vault
    let site_data = SiteData {
        pubkey: pubkey.0,
        sealed_password: sealed_password,
        sealed_password_salt: salt.0
    };

    vault.sites.insert(String::from(site), site_data);
    match io::update_vault(vault) {
        Ok(_) => println!("Password added for {}", site),
        Err(err) => {
            println!("Error: {}", err.to_string());
            return
        }
    }
}

pub fn remove(site: &str) {
    // Verify that passkeeper is initialized
    if !is_initialized() {
        io::prompt_init();
        return
    }

    let mut vault = io::get_vault().unwrap();

    // Is `site` in the vault?
    if !vault.sites.contains_key(site) {
        println!("No password for {} in your vault", site);
        return
    }

    // Remove the site's entry and update the vault file
    vault.sites.remove(site);
    match io::update_vault(vault) {
        Ok(_) => println!("Password for {} removed from your vault", site),
        Err(err) => {
            println!("Error: {}", err.to_string());
            return
        }
    }
}

pub fn show(site: &str) {
    let master_password =
        match io::prompt_password("Enter your master password", false) {
            Ok(password) => password,
            Err(err) => {
                println!("Error: {}", err.to_string());
                return
            }
        };

    let key_data = io::get_key_data().unwrap();
    let vault = io::get_vault().unwrap();

    // Does the site exist in the vault?
    match vault.sites.get(site) {
        // The site exists in the vault
        Some(site_data) => {
            println!("Decrypting your password for {}...", site);

            // Re-derive master key from master password
            let master_key = crypto_utils::get_master_key(
                master_password.into_bytes(),
                key_data.master_key_salt);

            // Decrypt master private key
            let master_privkey = secretbox::open(
                &key_data.sealed_master_privkey,
                &secretbox::Nonce(key_data.sealed_master_privkey_salt),
                &secretbox::Key(master_key));

            if master_privkey.is_err() {
                println!("Sorry, wrong password");
                return
            }

            // TODO: This is hacky
            // Do I need to clone the slice to get master_password as raw bytes?
            let mut master_password_bytes = [0u8;32];
            master_password_bytes.clone_from_slice(&master_privkey.unwrap());

            // Decrypt the sealed password
            match box_::open(
                &site_data.sealed_password,
                &box_::Nonce(site_data.sealed_password_salt),
                &box_::PublicKey(site_data.pubkey),
                &box_::SecretKey(master_password_bytes)) {
                Ok(pw) => {
                    let password_string = String::from_utf8(pw).unwrap();
                    println!("Password for {}: {}", site, password_string);
                },
                Err(err) => println!("Error: {:?}", err)
            }
        },
        None => println!("No password available for {}", site)
    }
}
