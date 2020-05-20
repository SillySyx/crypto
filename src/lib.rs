extern crate c2_chacha;
extern crate rand;

mod cryptography;
mod keys;

pub use {
    cryptography::{encrypt, decrypt},
    keys::{generate_key_from_seed, generate_iv_from_seed},
};