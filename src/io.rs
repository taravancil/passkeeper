// std
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{ Error, ErrorKind, stdin };
use std::io::prelude::*;
use std::path::{ PathBuf };

// passkeeper
use vault;

// external
extern crate serde;
extern crate serde_json;

/// Creates $HOME/.passkeeper/
pub fn create_passkeeper_dir() -> Result<(), Error> {
    let path = env::home_dir().unwrap().join(".passkeeper");
    try!(fs::create_dir(path));
    Ok(())
}

/// Creates $HOME/.passkeeper/vault
pub fn create_vault() -> Result<fs::File, Error> {
    let mut f = try!(fs::File::create(get_vault_path()));
    let vault = vault::Vault { sites: HashMap::new() };
    let serialized = serde_json::to_string(&vault).unwrap();
    try!(f.write_all(&serialized.as_bytes()));

    Ok((f))
}

/// Creates $HOME/.passkeeper/data
pub fn create_key_data_file() -> Result<fs::File, Error> {
    let path = env::home_dir().unwrap().join(".passkeeper").join("data");
    let f = try!(fs::File::create(path));

    Ok((f))
}

fn get_passkeeper_directory() -> PathBuf {
    env::home_dir().unwrap().join(".passkeeper")
}

pub fn get_vault_path() -> PathBuf {
    get_passkeeper_directory().join("vault")
}

pub fn get_key_data_path() -> PathBuf {
    get_passkeeper_directory().join("data")
}

/// Returns the key data or an Error
pub fn get_key_data() -> Result<vault::KeyData, Error> {
    let path = get_key_data_path();

    match fs::File::open(&path) {
        Ok(f) => {
            let mut f = f;
            let mut serialized = String::new();
            f.read_to_string(&mut serialized).unwrap();
            let key_data: vault::KeyData = serde_json::from_str(&serialized).unwrap();
            return Ok(key_data)
        }
        Err(err) => return (Err(err))
    }
}

/// Returns the vault or an Error
pub fn get_vault() -> Result<vault::Vault, Error> {
    let path = get_vault_path();

    match fs::File::open(&path) {
        Ok(f) => {
            let mut f = f;
            let mut serialized = String::new();
            f.read_to_string(&mut serialized).unwrap();
            let vault: vault::Vault = serde_json::from_str(&serialized).unwrap();
            return Ok(vault)
        }
        Err(err) => return Err(err)
    }
}

/// Replaces the existing vault file `vault`
pub fn update_vault(vault: vault::Vault) -> Result<(), Error>{
    let serialized = serde_json::to_string(&vault).unwrap();
    try!(write(serialized.as_bytes(), get_vault_path()));

    Ok(())
}

/// Prompts user to initialize passkeeper
pub fn prompt_init() {
    println!("passkeeper is not initialized\nRun passkeeper init");
    return
}

/// Prompts user for input and reads one line from stdin and returns the resulting
/// string or an io::Error
pub fn prompt_input(prompt: &str) -> Result<String, Error> {
    let mut input = String::new();
    println!("{}:", prompt);

    // Only read one line
    match stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim trailing newline
            let trimmed = input.trim().to_string();
            if trimmed.len() == 0 {
                return Err(
                    Error::new(ErrorKind::InvalidInput, "Empty password"))
            }
            return Ok(trimmed);
        },
        Err(err) => {
            return Err(Error::new(ErrorKind::InvalidInput, err.to_string()))
        }
    };
}

/// Prompts user for a password; prompts twice if `verify` is true. Returns
/// a Result with either a password String or an InvalidInput Error.
pub fn prompt_password(prompt: &str, verify: bool) -> Result<String, Error> {
    let password = prompt_input(&prompt);
    let mut password_verify: Result<String, Error> = Ok(String::from(""));

    if verify {
        let verify_prompt = format!("{} again", prompt);
        password_verify = prompt_input(&verify_prompt);
    }

    // Check that the user entered a password(s)
    if password.is_err() || verify && password_verify.is_err() {
        return Err(Error::new(ErrorKind::InvalidInput, "You must enter a password"))
    }

    // If `verify` is true, check that the passwords are the same
    let final_password = password.unwrap();
    if verify && final_password != password_verify.unwrap() {
        return Err(Error::new(ErrorKind::InvalidInput, "Passwords do not match"))
    }

    Ok(final_password)
}

/// Writes `b` bytes to the file at `path`
pub fn write(b: &[u8], path: PathBuf) -> Result<(), Error> {
    let mut buffer = try!(fs::File::create(path));
    try!(buffer.write_all(b));
    Ok(())
}
