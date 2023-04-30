use std::sync::Arc;

use rustfft::{num_complex::Complex32, Fft, FftPlanner};
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SignalBuffer {
    data: Vec<f32>,
}

#[wasm_bindgen]
impl SignalBuffer {
    pub fn new(length: usize) -> Self {
        Self {
            data: vec![0.; length],
        }
    }

    pub fn digest_chunk(&mut self, chunk: Vec<f32>) {
        self.data = chunk
    }

    pub fn get_length(&self) -> usize {
        self.data.len()
    }
}

#[wasm_bindgen]
pub struct FourierBuffer {
    fft_size: usize,
    fft: Arc<dyn Fft<f32>>,
    time_data: Vec<f32>,
    freq_data: Vec<f32>,
}

#[wasm_bindgen]
impl FourierBuffer {
    pub fn new(fft_size: usize) -> Self {
        set_panic_hook();

        let mut planner = FftPlanner::<f32>::new();
        Self {
            fft_size,
            fft: planner.plan_fft_forward(fft_size),
            time_data: vec![0.; fft_size],
            freq_data: vec![0.; fft_size],
        }
    }

    pub fn perform_fourier(&mut self) {
        let mut data_complex: Vec<Complex32> =
            self.time_data.iter().map(|&d| Complex32::from(d)).collect();
        self.fft.process(&mut data_complex);
        self.freq_data = data_complex.iter().map(|&d| d.re).rev().collect();
    }

    pub fn digest_chunk(&mut self, data: &[f32]) {
        assert_eq!(data.len(), self.fft_size);
        self.time_data = data.to_vec();
        self.perform_fourier();
    }

    pub fn get_dominant_bin(&self) -> Option<usize> {
        let max_amp = self.freq_data.iter().max_by(|&a, &b| a.total_cmp(&b))?;
        let (bin, &amp) = self
            .freq_data
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &a)| a > 0.9 * max_amp)
            .rev()
            .next()?;

        if amp < 1. || bin == 0 {
            return None;
        }
        return Some(bin);
    }

    pub fn get_freq_data(&self) -> Vec<f32> {
        self.freq_data.to_vec()
    }
}
