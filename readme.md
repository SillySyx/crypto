Easy to use byte encryption


# Features
* Generate keys and initialization vectors based on seeds
* Uses Chacha20 stream cipher to perform the transformation of the bytes


# Installation
This library has not been published to cargo as a package, to use it you can add it as a submodule for your project.

Add as a submodule
```
git submodule add https://github.com/SillySyx/crypto.git
```

Add to your dependencies
```
crypto = { path = "./crypto" }
```


# Examples
Encryption
```
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

```

Decryption
```
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
```