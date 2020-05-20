use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use c2_chacha::{ChaCha20};

pub fn encrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = value.to_owned();

    let mut cipher = match ChaCha20::new_var(key, iv) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to encrypt value")),
    };

    cipher.apply_keystream(&mut buffer);

    Ok(buffer)
}

pub fn decrypt(value: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = value.to_owned();

    let mut cipher = match ChaCha20::new_var(key, iv) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Failed to encrypt value")),
    };

    cipher.apply_keystream(&mut buffer);

    Ok(buffer)
}