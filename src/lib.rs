// FFT lib
// (actually DSFT - damn slow fourier transform right now)
extern crate num_complex;

use std::f64::consts::PI;
use num_complex::Complex;

const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

pub struct Phasor {
    pub frequency: f64,
    pub complex: Complex<f64>,
}

pub struct Sample {
    // sample data [-1.0-1.0,...]
    pub data: Vec<f64>,
    // samplerate eg: 44_100
    pub rate: usize
}

impl Sample {
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
    
    pub fn max_amplitude(&self) -> f64 {
        self.data.iter().fold(0.0, |acc: f64, &xy|{
            acc.max(xy.abs())
        })
    }

    pub fn with_time(&self) -> Vec<(f64,f64)> {
        let dt = 1.0 / self.rate as f64;
        self.data.iter()
            .enumerate()
            .map(|(idx,&a)| (idx as f64 * dt, a))
            .collect()
    }
}

// Winds up plot around fixed point(0,0) in unit circle
// as a function of f(t) = amplitude.
// Argument f is for winding frequency
// sf : sampling frequency (eg. 44100hz)
pub fn graph_circle(sample: &Sample, f: f64) -> Vec<Complex<f64>> {
    // precalculate -2πf
    let fc = 2.0 * PI * f;
    // cycles per second
    let sc = 1.0 / sample.rate as f64;
    // winding machine
    sample.data.iter()
        .enumerate()
        .map(|(idx,a)| {
            let t = sc * idx as f64;
            (I * fc * t).exp() * a
        })
    .collect()
}

// calculates mean average vector length from array of (f64,64)
fn calc_mean(data: Vec<Complex<f64>>) -> Complex<f64> {
    let mut mx = Complex{ re: 0.0, im: 0.0 };
    data.into_iter()
        .enumerate()
        .for_each(|(idx, c)| {
            let i = idx as f64;
            mx = ((mx * i) + c) / (i + 1.0);
        });
    mx
}

// Returns sampled FT analysis vector
pub fn analyze(sample: &Sample, min: f64, max: f64, ss: f64) -> Vec<(f64,Complex<f64>)> {
    println!("FT analysis: {} => {}, step {} hz", min, max, ss);

    // calculate data points
    let mut ft_data = vec![];
    let mut f = min;
    while f <= max {
        // calculate revolutions around unit circle
        let processed = graph_circle(sample, f);
        // calculate mean from y-axis values
        let mx = calc_mean(processed);
        ft_data.push((f, mx));

        f += ss;
    }

    ft_data
}

// Returns FT analysis float value
// for a frequency value
pub fn analyze_freq(sample: &Sample, f: f64) -> Complex<f64> {
    calc_mean(graph_circle(sample, f))
}

// finds a local max inside
pub fn max(data: &[Phasor]) -> usize {
    data.iter()
        .enumerate()
        .fold((0,Complex{re: 0.0, im: 0.0}),|acc,(idx,p)| {
        if acc.1.norm_sqr() > p.complex.norm_sqr() {
            acc
        }
        else {
            (idx,p.complex)
        }
    }).0
}

