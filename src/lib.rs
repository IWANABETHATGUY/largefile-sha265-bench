mod utils;

use cryptoxide::digest::Digest;
use cryptoxide::sha2::Sha256;
use wasm_bindgen::prelude::*;
use web_sys::console;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
#[wasm_bindgen]
pub struct Sha256Struct {
    sha265: Sha256,
    buffer: Vec<u8>,
    result: Vec<u8>,
}
#[wasm_bindgen]
impl Sha256Struct {
    pub fn new() -> Sha256Struct {
        let buffer = vec![0; 1024 * 1024];
        let result = vec![0; 32];
        Sha256Struct {
            sha265: Sha256::new(),
            buffer,
            result,
        }
    }

    pub fn update(&mut self, len: usize) {
        // self.sha265.(&self.buffer[..len]);
        self.sha265.input(&self.buffer[..len])
    }

    pub fn result(&mut self) -> *const u8 {
        self.sha265.result_str().as_ptr()
    }

    pub fn buffer_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn clear(&mut self) {
        self.sha265.reset();
    }
}
// #[wasm_bindgen]
// pub fn sha265(content: &[u8]) -> *const u8 {
//     let mut hasher = Sha256::new();

//     // write input message
//     hasher.update(content);
//     hasher.finalize().as_ptr()
// }
#[wasm_bindgen]
pub fn sha256(content: &[u8]) -> *const u8 {
    use cryptoxide::digest::Digest;
    use cryptoxide::sha2::Sha256;

    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input(content);
    hasher.result_str().as_ptr()
}
