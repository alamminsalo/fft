extern crate num_complex;

use std::f64::consts::PI;
use num_complex::Complex;

// Phasor (vector)
pub struct Phasor {
    // phasor length
    pub length: f64,

    // angle in radians
    pub angle: f64,
}

impl Phasor {
    // creates new phasor from x,y coordinates
    pub fn new(x: f64, y: f64) -> Phasor {
        Phasor {
            length: (x.powf(2f64) + y.powf(2f64)).sqrt(),
            angle: 
        }
    }
}

