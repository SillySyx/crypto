use std::error::Error;

use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use c2_chacha::ChaCha20;

pub fn encrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    apply_transformation_of_bytes(value, key, iv)
}

pub fn decrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    apply_transformation_of_bytes(value, key, iv)
}

fn apply_transformation_of_bytes(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer = value.to_owned();

    let mut cipher = match ChaCha20::new_var(key, iv) {
        Ok(value) => value,
        Err(_) => return Err(Box::from("Failed to encrypt value")),
    };

    cipher.apply_keystream(&mut buffer);

    Ok(buffer)
}