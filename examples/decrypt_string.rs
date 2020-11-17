use std::error::Error;
use crypto::{generate_iv_from_seed, generate_key_from_seed, decrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let seed = "my secret seed";
    let key = generate_key_from_seed(seed)?;
    let iv = generate_iv_from_seed(seed)?;
    
    let encrypted_bytes = [145, 85, 81, 16, 179, 235, 33, 143, 246, 209, 147];

    let bytes = decrypt(&encrypted_bytes, &key, &iv)?;
    let value = String::from_utf8(bytes)?;

    assert_eq!(value, "Hello world");

    Ok(())
}
