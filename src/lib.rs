mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct TestAnalyser {
    fft_size: usize,
    samplerate: usize,
    tuning_freq: f64,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Note {
    A = "A",
    Bb = "Bb",
    B = "B",
    C = "C",
    Db = "Db",
    D = "D",
    Eb = "Eb",
    E = "E",
    F = "F",
    Gb = "Gb",
    G = "G",
    Ab = "Eb",
}

impl Note {
    fn from_index(index: usize) -> Self {
        [
            Note::A,
            Note::Bb,
            Note::B,
            Note::C,
            Note::Db,
            Note::D,
            Note::Eb,
            Note::E,
            Note::F,
            Note::Gb,
            Note::G,
            Note::Ab,
            Note::A,
        ][index]
    }
}

fn normalize_frequency(tuning_freq: f64, mut frequency: f64) -> (i32, f64) {
    // clamps a given frequency into the octave above the tuning frequency
    // and returns the number of octave shifts
    assert_ne!(frequency, 0.);
    let mut octaves = 0;
    while frequency > 2. * tuning_freq {
        frequency /= 2.;
        octaves += 1;
    }
    while frequency <= tuning_freq {
        frequency *= 2.;
        octaves -= 1;
    }
    (octaves, frequency)
}

#[wasm_bindgen]
pub struct NoteWithOffset(pub Note, pub f64);

#[wasm_bindgen]
pub struct EqualTemperament {
    tuning_freq: f64,
}

#[wasm_bindgen]
impl EqualTemperament {
    pub fn new(tuning_freq: f64) -> Self {
        Self { tuning_freq }
    }

    pub fn to_halftones(&self, freq: f64) -> f64 {
        12. * (freq / self.tuning_freq).log2()
    }

    pub fn to_cents(&self, freq: f64) -> f64 {
        100. * self.to_halftones(freq)
    }

    pub fn get_closest_note(&self, freq: f64) -> NoteWithOffset {
        // norm_freq is in [tuning_freq, 2*tuning_freq)
        let (octaves, norm_freq) = normalize_frequency(self.tuning_freq, freq);

        // How many halftones from the tuning frequency (per definition in [0, 12])
        let halftones = self.to_halftones(norm_freq);
        let closest_halftones = halftones.round() as usize;
        let closest_cents = 100. * (closest_halftones as f64);
        let cents = 100. * halftones;
        let offset_cents = cents - closest_cents;
        NoteWithOffset(Note::from_index(closest_halftones), offset_cents)
    }
}

#[wasm_bindgen]
impl TestAnalyser {
    pub fn new(tuning_freq: f64, fft_size: usize, samplerate: usize) -> Self {
        Self {
            tuning_freq,
            fft_size,
            samplerate,
        }
    }

    pub fn get_dominant_bucket(&self, data: &[u8]) -> Option<usize> {
        let result = data.iter().enumerate().max_by_key(|(_, &amp)| amp);
        // let threshold = (0.7*(*data.iter().max().unwrap() as f64)) as u8;
        // let result = data.iter().enumerate().filter(|(_, &amp)| amp >= threshold).next();

        if let Some((_, 0)) = result {
            // No sound
            None
        } else if let Some((0, _)) = result {
            //
            None
        } else if let Some((i, _)) = result {
            // Loudest frequency found
            Some(i)
        } else {
            // no data, this case should not occur
            None
        }
    }

    pub fn get_dominant_frequency(&self, data: &[u8]) -> Option<usize> {
        let bucket = self.get_dominant_bucket(data)?;
        Some(bucket * self.samplerate / self.fft_size)
    }
}
