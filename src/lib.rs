mod utils;

use wasm_bindgen::prelude::*;

use sha2::{Digest, Sha256};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sha265(buffer: Vec<u8>) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(buffer);
    let result = hasher.finalize();
    result.as_slice().into()
}
