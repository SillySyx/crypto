use std::error::Error;
use crypto::{generate_iv_from_seed, generate_key_from_seed, encrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let seed = "my secret seed";
    let key = generate_key_from_seed(seed)?;
    let iv = generate_iv_from_seed(seed)?;
    
    let value = String::from("Hello world");
    let bytes = value.as_bytes();
    
    let encrypted_bytes = encrypt(&bytes, &key, &iv)?;

    assert_eq!(encrypted_bytes, [145, 85, 81, 16, 179, 235, 33, 143, 246, 209, 147]);

    Ok(())
}
