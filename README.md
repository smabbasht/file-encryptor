# Crypt  🔐

### A simple file-encryptor written in Rust 🦀

This package uses the **Advanced Encryption Standard** (AES) for file encryption and decryption, It does so by leveraging the **Password-based-key-derivation-function v2** (PKBFD2) for acquiring a key based on the provided password and randomized nonce. It then uses a `AES256Cipher` to encrypt or decrypt the files. The whole program is written in rust. 

### Usage

#### Using Cargo
To encrypt a file, run `cargo run encrypt path/to/file`
You will be prompted to give a password. Your file
will be encrypted with the given password inplace.

To decrypt a file run `cargo run decrypt path/to/file`
It will ask you for a password which if correct will
will decrypt the file inplace.

#### Using the stand-alone binary
If you only want to use the binary, you just need to run `cargo build --release`, a binary will be created as `target/release/crypt`. This `crypt` file is what you need. You can move it anywhere and it would work flawlessly. Example usage could be `crypt encrypt path/to/file`
