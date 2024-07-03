use aes::{cipher::KeyInit, Aes128};
use aes_gcm;
use libaes::Cipher;
use pbkdf2::{pbkdf2, pbkdf2_hmac};
use rand::prelude::*;
use rpassword::read_password;
use sha2::Sha256;
use std::{fs, io::Write, path::Path, process::exit};

pub fn encrypt(filepath: &str, passwd: String) {
    let source = fs::read_to_string(filepath).unwrap();

    let mut rng = thread_rng();
    let mut nonce = [0u8; 12];
    for i in 0..12 {
        nonce[i] = rng.gen();
    }

    let mut der_key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(passwd.as_bytes(), &nonce, 4096, &mut der_key);

    let cipher = Cipher::new_256(&der_key);

    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).unwrap();

    let block_size = cipher.block_size();
    let mut encrypted_data = vec![0; text.len() + block_size];
    let count = encrypter.update(text, &mut encrypted_data).unwrap();
    let rest = encrypter.finalize(&mut encrypted_data[count..]).unwrap();
    encrypted_data.truncate(count + rest);
}

pub fn decrypt(filepath: &str, passwd: String) {}

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
