mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test() {
    alert("working")
}


#[wasm_bindgen]
pub struct TestAnalyser {
    fft_size: usize,
    samplerate: usize
}

#[wasm_bindgen]
impl TestAnalyser {
    pub fn new(fft_size: usize, samplerate: usize) -> Self {
        TestAnalyser {fft_size, samplerate}
    }

    pub fn process(&self, data: &[u8]) -> usize {
        // return data.len()
        data.iter()
            .enumerate()
            .max_by_key(|(_, &amp)| amp)
            .map(|(i, _)| i)
            .unwrap()
    }
}
