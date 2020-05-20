use crypto::{encrypt, decrypt};

#[test]
fn should_be_possible_to_encrypt_value() {
    let key = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let iv = vec![1,2,3,4,5,6,7,8];
    let data = vec![1,2,3];
    
    let encrypted_data = encrypt(&data, &key, &iv);

    assert!(encrypted_data == Ok(vec![204,154,33]));
}

#[test]
fn should_be_possible_to_decrypt_value() {
    let key = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let iv = vec![1,2,3,4,5,6,7,8];
    let data = vec![204,154,33];
    
    let encrypted_data = decrypt(&data, &key, &iv);

    assert!(encrypted_data == Ok(vec![1,2,3]));
}