use std::sync::Arc;

use crate::{buffer_analyser::SignalBuffer, utils::set_panic_hook};
use rustfft::{num_complex::Complex32, Fft, FftPlanner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TimeSignalAnalyser {
    fft_size: usize,
    fft: Arc<dyn Fft<f32>>,
    //    internal_state: Option<&[f32]>,
}

#[wasm_bindgen]
impl TimeSignalAnalyser {
    pub fn new(fft_size: usize) -> Self {
        set_panic_hook();

        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_size);
        Self { fft_size, fft }
    }

    pub fn fourier_transform(&mut self, data: &[f32]) -> Vec<f32> {
        let mut data_complex: Vec<Complex32> = data.iter().map(|&d| Complex32::from(d)).collect();
        self.fft.process(&mut data_complex);

        let fourier_real: Vec<f32> = data_complex.iter().map(|&d| d.re).rev().collect();
        fourier_real
    }

    pub fn digest_chunk(&mut self, data: &[f32]) -> Option<usize> {
        assert_eq!(data.len(), self.fft_size);
        let fourier_real = self.fourier_transform(&data);
        let max_amp = fourier_real.iter().max_by(|&a, &b| a.total_cmp(b))?;

        let (index, &amp) = fourier_real
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &a)| a > 0.8 * max_amp)
            .rev()
            .next()?;
        // .max_by(|(_, &a), (_, &b)| a.total_cmp(b))?;
        // .map(|(index, _)| index);

        // let bin = data.len() - index;
        let bin = index;

        if amp < 1. {
            return None;
        }

        if bin == 0 {
            return None;
        }

        return Some(bin);
    }

    pub fn digest_buffer(&mut self, buffer: &SignalBuffer) -> String {
        buffer.get_length();
        return format!("{:p}", &buffer);
        // unsafe { addr_of!(buffer).read_unaligned() }
    }
}
