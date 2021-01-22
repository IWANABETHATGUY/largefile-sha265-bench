use cryptoxide::digest::Digest;
use cryptoxide::sha2::Sha256;

// create a Sha256 object
pub struct Sha256Struct {
    sha265: Sha256,
    buffer: Vec<u8>,
}
impl Sha256Struct {
    pub fn new() -> Sha256Struct {
        let mut buffer = Vec::with_capacity(1024);
        for i in 0..1024 {
            buffer.push(0);
        }
        Sha256Struct {
            sha265: Sha256::new(),
            buffer,
        }
    }

    pub fn sha265(&self, len: usize) -> Vec<u8> {
        // Sha256::digest(&self.buffer[..len]).as_slice().to_vec()
        // self.sha265.input(input)
        vec![]
    }

    pub fn buffer_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
}

fn main() {
    // create a Sha256 object
}
