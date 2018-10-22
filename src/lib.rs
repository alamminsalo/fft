// FFT lib
// (actually DSFT - damn slow fourier transform right now)
extern crate num_complex;

use std::f64::consts::PI;
use num_complex::Complex;

const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

// Winds up plot around fixed point(0,0) in unit circle
// as a function of f(t) = amplitude.
// Argument f is for winding frequency
// sf : sampling frequency (eg. 44100hz)
pub fn graph_circle(data: &[f64], sf: f64, f: f64) -> Vec<(f64,f64)> {
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

// calculates mean average vector length from array of (f64,64)
fn calc_mean(data: Vec<(f64,f64)>) -> f64 {
    let mut mx = 0.0;
    let mut my = 0.0;
    data.into_iter()
        .enumerate()
        .for_each(|(idx, xy)| {
            let i = idx as f64;
            mx = ((mx * i) + xy.0) / (i + 1.0);
            my = ((my * i) + xy.1) / (i + 1.0);
        });

    // return vector length from origo
    (mx.powf(2.0) + my.powf(2.0)).sqrt()
}

// Returns sampled FT analysis vector
pub fn analyze(data: (&[f64], f64), min: f64, max: f64, ss: f64, plot_circle: bool) -> Vec<(f64,f64)> {
    println!("FT analysis: {} => {}, step {} hz", min, max, ss);

    // calculate data points
    let mut ft_data = vec![];
    let mut f = min;
    while f <= max {
        // calculate revolutions around unit circle
        let processed = graph_circle(data.0, data.1, f);
        // calculate mean from y-axis values
        let mx = calc_mean(processed);
        ft_data.push((f, mx));

        f += ss;
    }

    ft_data
}

// Returns FT analysis float value
// for a frequency value
pub fn analyze_freq(data: (&[f64], f64), f: f64) -> f64 {
    let processed = graph_circle(data.0, data.1, f);
    calc_mean(processed)
}




