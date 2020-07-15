use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::error::Error;

use rand::prelude::*;
use rand::SeedableRng;

fn hash_value(seed: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    hasher.finish()
}

pub fn generate_key_from_seed(seed: &str) -> Result<[u8; 32], Box<dyn Error>> {
    let hash = hash_value(seed);

    let mut buffer = [0u8; 32];
    
    let mut rng: StdRng = SeedableRng::seed_from_u64(hash);
    rng.fill_bytes(&mut buffer);

    Ok(buffer)
}

pub fn generate_iv_from_seed(seed: &str) -> Result<[u8; 8], Box<dyn Error>> {
    let hash = hash_value(seed);

    let mut buffer = [0u8; 8];
    
    let mut rng: StdRng = SeedableRng::seed_from_u64(hash);
    rng.fill_bytes(&mut buffer);

    Ok(buffer)
}