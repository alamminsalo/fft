extern crate num_complex;
extern crate plotlib;

pub mod util;

// FFT lib
// (actually DSFT - damn slow fourier transform right now)

use std::f64::consts::PI;
use num_complex::Complex;

// i
const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

// Winds up plot around fixed point(0,0) in unit circle
// as a function of f(t) = amplitude.
// Argument f is for winding frequency
fn wind_unitcircle(data: &Vec<(f64,f64)>, f: f64) -> Vec<(f64,f64)> {
    // precalculate -2πf
    let fc = -2.0 * PI * f;
    // winding machine
    data.into_iter()
        .map(|(t,a)| {
            let xy = (I * fc * t).exp() * a;
            (xy.re, xy.im)
        })
    .collect()
}

// calculates mean average from array of (f64,f64) tuples,
// using the second value of the tuple
fn calc_mean(data: &[(f64,f64)]) -> f64 {
    data.into_iter()
        .enumerate()
        .fold(0.0, |mean, (idx, x)| {
            if idx == 0 {
                x.1
            }
            else {
                ((mean * (idx as f64)) + x.1) / (idx as f64 + 1.0)
            }
        })
}

// Returns sampled FT analysis vector
pub fn analyze(data: &Vec<(f64,f64)>, min: f64, max: f64, ss: f64) -> Vec<(f64,f64)> {
    println!("FT analysis: [{} - {}], step {} hz", min, max, ss);

    // calculate data points
    let mut ft_data = vec![];
    let mut f = min;
    while f <= max {
        // calculate revolutions around unit circle
        let processed = wind_unitcircle(data,f);
        // calculate mean X value
        let mx = calc_mean(&processed);
        // drawunitcircle(&processed);
        ft_data.push((f, -mx));

        f += ss;
    }

    ft_data
}


