extern crate num_complex;

use std::f32::consts::PI;
use num_complex::Complex;

// Phasor (vector)
pub struct Phasor {
    // phasor length
    pub length: f32,

    // angle in radians
    pub angle: f32,
}

impl Phasor {
    // creates new phasor from x,y coordinates
    pub fn new(x: f32, y: f32) -> Phasor {
        Phasor {
            length: (x.powf(2f32) + y.powf(2f32)).sqrt(),
            angle: 
        }
    }
}

