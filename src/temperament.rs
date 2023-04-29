use wasm_bindgen::prelude::*;

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
    pub fn from_index(index: usize) -> Self {
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

#[wasm_bindgen]
pub struct NoteWithOffset(pub Note, pub f64);

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
        let (_octaves, norm_freq) = normalize_frequency(self.tuning_freq, freq);

        // How many halftones from the tuning frequency (per definition in [0, 12])
        let halftones = self.to_halftones(norm_freq);
        let closest_halftones = halftones.round() as usize;
        let closest_cents = 100. * (closest_halftones as f64);
        let cents = 100. * halftones;
        let offset_cents = cents - closest_cents;
        NoteWithOffset(Note::from_index(closest_halftones), offset_cents)
    }
}
