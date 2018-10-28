// FFT lib
// (actually DSFT - damn slow fourier transform right now)
extern crate num_complex;

pub mod util;

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
    // precalculate 2πf
    let fc = 2.0 * PI * f;
    // cycles per second
    let sc = 1.0 / sample.rate as f64;
    // winding machine
    sample.data.iter()
        .enumerate()
        .map(|(i,a)| {
            let t = sc * i as f64;
            let c = (I * fc * t).exp() * a;
            // TODO: why does this have to be negated
            Complex{re: c.im, im: c.re}
        })
    .collect()
}

// calculates mean average vector length from array of (f64,64)
fn calc_mean(data: Vec<Complex<f64>>) -> Complex<f64> {
    data.into_iter()
        .enumerate()
        .fold(Complex{re:0.0,im:0.0},
              |acc, (i, c)| {
                  ((acc * i as f64) + c) / (i + 1) as f64
              })
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
            if acc.1.to_polar().0 > p.complex.to_polar().0 {
                acc
            }
            else {
                (idx,p.complex)
            }
        }).0
}

#[cfg(test)]
mod tests {
use std::f64::consts::PI;
use super::*;

#[test]
    fn test_circle_single(){
        let sine = util::sinewaves(1.0, 1000, &[(5.0,0.0)]);
        let circle = graph_circle(&sine, 5.0);
        let center = calc_mean(circle);
        assert!(center.re > 0.45);
    }

#[test]
    fn test_circle_multi(){
        let sine = util::sinewaves(1.0, 1000, &[(5.0,0.0),(10.0,0.0)]);

        let circle = graph_circle(&sine, 5.0);
        let center = calc_mean(circle);
        assert!(center.re > 0.45);

        let circle = graph_circle(&sine, 10.0);
        let center = calc_mean(circle);
        assert!(center.re > 0.45);
    }

#[test]
    fn test_circle_phase(){
        let sine = util::sinewaves(1.0, 1000, &[(5.0,90.0)]);

        let circle = graph_circle(&sine, 5.0);
        let polar = calc_mean(circle).to_polar();
        let deg = polar.1 * 180.0 / PI;
        assert!(polar.0 > 0.45);
        assert!(deg > 89.9 && deg < 90.1);
    }

#[test]
    fn test_circle_phase_multi(){
        let sine = util::sinewaves(1.0, 1000, &[(5.0,90.0),(60.0,0.0)]);

        let circle = graph_circle(&sine, 5.0);
        let polar = calc_mean(circle).to_polar();
        let deg = polar.1 * 180.0 / PI;
        assert!(polar.0 > 0.45);
        assert!(deg > 89.9 && deg < 90.1);

        let circle = graph_circle(&sine, 60.0);
        let polar = calc_mean(circle).to_polar();
        let deg = polar.1 * 180.0 / PI;
        assert!(polar.0 > 0.45);
        assert!(deg > -0.1 && deg < 0.1);

        let sine = util::sinewaves(1.0, 1000, &[(5.0,180.0),(60.0,270.0)]);

        let circle = graph_circle(&sine, 5.0);
        let polar = calc_mean(circle).to_polar();
        let deg = polar.1 * 180.0 / PI;
        assert!(polar.0 > 0.45);
        println!("{}", deg);
        assert!(deg > -180.1 && deg < -179.1);

        let circle = graph_circle(&sine, 60.0);
        let polar = calc_mean(circle).to_polar();
        let deg = polar.1 * 180.0 / PI;
        println!("{}", deg);
        assert!(polar.0 > 0.45);
        assert!(deg > -90.1 && deg < 89.1);
    }
}
