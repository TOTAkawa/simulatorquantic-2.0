use wasm_bindgen::prelude::*;
use num_complex::Complex64;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

#[wasm_bindgen]
pub struct QuantumSimulation {
    state: Vec<Complex64>,
    num_qubits: usize,
}

#[wasm_bindgen]
impl QuantumSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        state[0] = Complex64::new(1.0, 0.0);
        Self { state, num_qubits }
    }

    pub fn reset(&mut self) {
        for i in 0..self.state.len() {
            self.state[i] = Complex64::new(0.0, 0.0);
        }
        self.state[0] = Complex64::new(1.0, 0.0);
    }

    pub fn h(&mut self, target: usize) {
        let bit = 1 << target;
        for i in (0..self.state.len()).step_by(bit * 2) {
            for j in i..i + bit {
                let u = self.state[j];
                let v = self.state[j + bit];
                self.state[j] = (u + v) * FRAC_1_SQRT_2;
                self.state[j + bit] = (u - v) * FRAC_1_SQRT_2;
            }
        }
    }

    pub fn x(&mut self, target: usize) {
        let bit = 1 << target;
        for i in (0..self.state.len()).step_by(bit * 2) {
            for j in i..i + bit { self.state.swap(j, j + bit); }
        }
    }

    pub fn y(&mut self, target: usize) {
        let bit = 1 << target;
        let i_unit = Complex64::new(0.0, 1.0);
        for i in (0..self.state.len()).step_by(bit * 2) {
            for j in i..i + bit {
                let u = self.state[j];
                let v = self.state[j + bit];
                self.state[j] = -v * i_unit;
                self.state[j + bit] = u * i_unit;
            }
        }
    }

    pub fn z(&mut self, target: usize) {
        let bit = 1 << target;
        for i in 0..self.state.len() {
            if (i & bit) != 0 { self.state[i] = -self.state[i]; }
        }
    }

    pub fn s_gate(&mut self, target: usize) {
        let bit = 1 << target;
        let i_unit = Complex64::new(0.0, 1.0);
        for i in 0..self.state.len() {
            if (i & bit) != 0 { self.state[i] *= i_unit; }
        }
    }

    pub fn t_gate(&mut self, target: usize) {
        let bit = 1 << target;
        let phase = Complex64::from_polar(1.0, PI / 4.0);
        for i in 0..self.state.len() {
            if (i & bit) != 0 { self.state[i] *= phase; }
        }
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        let bit_c = 1 << control;
        let bit_t = 1 << target;
        for i in 0..self.state.len() {
            if (i & bit_c) != 0 && (i & bit_t) == 0 {
                self.state.swap(i, i | bit_t);
            }
        }
    }

    pub fn ccx(&mut self, c1: usize, c2: usize, target: usize) {
        let b1 = 1 << c1; let b2 = 1 << c2; let bt = 1 << target;
        for i in 0..self.state.len() {
            if (i & b1) != 0 && (i & b2) != 0 {
                let j = i ^ bt;
                if i < j { self.state.swap(i, j); }
            }
        }
    }

    pub fn get_probabilities(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.norm_sqr()).collect()
    }

    pub fn get_bloch_vector(&self, target: usize) -> Vec<f64> {
        let bit = 1 << target;
        let mut r00 = Complex64::new(0.0, 0.0);
        let mut r01 = Complex64::new(0.0, 0.0);
        for i in 0..self.state.len() {
            if (i & bit) == 0 {
                r00 += self.state[i] * self.state[i].conj();
                r01 += self.state[i] * self.state[i | bit].conj();
            }
        }
        vec![2.0 * r01.re, 2.0 * r01.im, 2.0 * r00.re - 1.0]
    }
}
