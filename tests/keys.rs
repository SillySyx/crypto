use std::error::Error;

use crypto::{generate_iv_from_seed, generate_key_from_seed};

#[test]
fn should_be_possible_to_generate_key_from_seed() -> Result<(), Box<dyn Error>> {
    let seed = "my super secret that i will take to my grave";
    let key = generate_key_from_seed(seed)?;

    assert!(key == [76, 217, 106, 130, 128, 75, 199, 167, 106, 156, 157, 241, 221, 2, 51, 190, 200, 200, 169, 101, 229, 154, 104, 81, 77, 55, 21, 223, 208, 244, 227, 88]);

    Ok(())
}

#[test]
fn should_be_possible_to_generate_iv_from_seed() -> Result<(), Box<dyn Error>> {
    let seed = "resource that i base my iv on";
    let key = generate_iv_from_seed(seed)?;

    assert!(key == [92, 129, 36, 125, 146, 89, 200, 180]);

    Ok(())
}