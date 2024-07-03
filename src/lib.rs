use aes::cipher::KeyInit;
use aes_gcm::{aead::Aead, Aes256Gcm, Nonce};
use pbkdf2::pbkdf2_hmac;
use rand::prelude::*;
use rpassword::read_password;
use sha2::Sha256;
use std::{fs, io::Write, path::Path, process::exit};

pub fn encrypt(filepath: &str, passwd: String) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read(filepath)?;
    let mut rng = thread_rng();
    let mut salt = [0u8; 16];
    rng.fill_bytes(&mut salt);

    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(passwd.as_bytes(), &salt, 100_000, &mut key);

    let cipher = Aes256Gcm::new(key.as_slice().into());
    let nonce = Nonce::from_slice(&salt[..12]);

    let encrypted_data = cipher
        .encrypt(nonce, source.as_ref())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;

    let mut output = Vec::new();
    output.extend_from_slice(&salt);
    output.extend_from_slice(&encrypted_data);

    fs::write(filepath, output)?;
    Ok(())
}

pub fn decrypt(filepath: &str, passwd: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;

    if data.len() < 16 {
        return Err("File is too short to be a valid encrypted file".into());
    }

    let (salt, encrypted_data) = data.split_at(16);
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(passwd.as_bytes(), salt, 100_000, &mut key);

    let cipher = Aes256Gcm::new(key.as_slice().into());
    let nonce = Nonce::from_slice(&salt[..12]);

    let decrypted_data = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| format!("Decryption failed: {:?}", e))?;

    fs::write(filepath, decrypted_data)?;
    Ok(())
}

pub fn get_password() -> String {
    println!("Enter password for encryption");
    read_password().unwrap()
}

pub fn set_password() -> String {
    println!("Enter password for encryption");
    std::io::stdout().flush().unwrap();
    let passwd = read_password().unwrap();
    println!("Confirm the password");
    std::io::stdout().flush().unwrap();
    let passwd_confirm = read_password().unwrap();
    if passwd != passwd_confirm {
        println!("Passwords do not match");
        exit(1);
    } else {
        passwd
    }
}

pub fn file_exists(filepath: &str) -> bool {
    Path::new(filepath).is_file()
}
