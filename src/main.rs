use std::{env, process::exit};

use file_encryptor::{decrypt, encrypt, file_exists, get_password, set_password};
use rand::{thread_rng, Rng};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_help();
    } else {
        match args[1].as_str() {
            "encrypt" => encrypt_handle(&args[2]),
            "decrypt" => decrypt_handle(&args[2]),
            _ => print_help(),
        };
        println!("{:?}", args);
    }
}

fn encrypt_handle(filepath: &str) {
    println!("Starting encryption");
    if !file_exists(filepath) {
        println!("No file exists on path: {filepath}");
        exit(1);
    }
    let passwd = set_password();
    println!("Encrypting your file....");
    encrypt(filepath, passwd);
    println!("{filepath} has been encrypted successfully");
}

fn decrypt_handle(filepath: &str) {
    if !file_exists(filepath) {
        println!("No file exists on path: {filepath}");
        exit(1);
    }
    let passwd = get_password();
    println!("Decrypting your file....");
    decrypt(filepath, passwd);
    println!("{filepath} has been decrypted successfully");
}

fn print_help() {
    println!();
    println!("         ----- DAAI Encrypter -----        ");
    println!();
    println!("To encrypt a file, run `crypt encrypt path/to/file`");
    println!("You will be prompted to give a password. Your file");
    println!("will be encrypted with the given password inplace");
    println!();
    println!("To decrypt a file run `crypt decrypt path/to/file`");
    println!("It will ask you for a password which if correct will");
    println!("will decrypt the file inplace");
    println!();
}
