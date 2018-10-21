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
// sf : sampling frequency (eg. 44100hz)
fn wind_unitcircle(data: &[f64], sf: f64, f: f64) -> Vec<(f64,f64)> {
    // precalculate -2πf
    let fc = 2.0 * PI * f;
    // cycles per second
    let sc = 1.0 / sf;
    // winding machine
    data.into_iter()
        .enumerate()
        .map(|(idx,a)| {
            let t = sc * idx as f64;
            let xy = (I * fc * t).exp() * a;
            (xy.im, xy.re)
        })
    .collect()
}

// calculates mean average from array of f64
fn calc_mean(data: Vec<f64>) -> f64 {
    data.into_iter()
        .enumerate()
        .fold(0.0, |mean, (idx, x)| {
            if idx == 0 {
                x
            }
            else {
                ((mean * (idx as f64)) + x) / (idx as f64 + 1.0)
            }
        })
}

// Returns sampled FT analysis vector
pub fn analyze(data: (&[f64], f64), min: f64, max: f64, ss: f64, plot_winding: bool) -> Vec<(f64,f64)> {
    println!("FT analysis: {} => {}, step {} hz", min, max, ss);

    // calculate data points
    let mut ft_data = vec![];
    let mut f = min;
    while f <= max {
        // calculate revolutions around unit circle
        let processed = wind_unitcircle(data.0, data.1, f);
        if plot_winding {
            println!("Circle: {} hz", f);
            util::drawunitcircle(&processed);
        }
        // calculate mean from y-axis values
        let mx = calc_mean(processed.into_iter().map(|(x,_)| x).collect());
        ft_data.push((f, mx));

        f += ss;
    }

    ft_data
}


