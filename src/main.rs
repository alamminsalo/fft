extern crate num_complex;
extern crate plotlib;

use std::f64::consts::PI;
use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::page::Page;
use num_complex::Complex;

// i -- imaginary number
const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

fn sineplot(f: f64, ts: f64, ss: usize) -> Vec<(f64,f64)> {
    println!("sine: {} hz for {} seconds with sampling frequency of {}", f, ts, ss);

    // seconds to milliseconds for easier step handling
    let tms = (ts * 1000.0) as usize;

    // precalculate 2πf
    let fc = 2.0 * PI * f;

    // sine generator
    // f(x) = y where x is time
    (0..tms).step_by(ss)
        .map(|ms| {
            // back to seconds
            let t = (ms as f64) / 1000.0;
            let tc = fc * t;
            (t,tc.sin())
        })
    .collect()
}

// Winds up plot around fixed point(0,0) in a circle
// as a function of time => amplitude
// f is for winding frequency
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

// draws linear time plot
fn drawplot(data: &Vec<(f64,f64)>) {
    // We create our scatter plot from the data
    let s1 = Scatter::from_vec(data);

    let x0 = data.first().unwrap().0.round();
    let x1 = data.last().unwrap().0.round();

    // The 'view' describes what set of data is drawn
    let v = View::new()
        .add(&s1)
        .x_range(x0, x1)
        .y_range(-1.0, 1.0);

    println!("{}", Page::single(&v).to_text());
}

// draws plot in a unit circle
fn drawunitcircle(data: &Vec<(f64,f64)>) {
    // We create our scatter plot from the data
    let s1 = Scatter::from_vec(data);

    // The 'view' describes what set of data is drawn
    let v = View::new()
        .add(&s1)
        .x_range(-1.0,1.0)
        .y_range(-1.0,1.0);

    println!("{}", Page::single(&v).to_text());
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
fn ft_analyse(data: &Vec<(f64,f64)>, from: usize, to: usize, stepsize: usize) -> Vec<(f64,f64)> {
    println!("FT analysis between {} - {} hz", from / 1000, to / 1000);
    println!("Stepsize {} hz", (stepsize as f64) / 1000.0);
    (from..to).step_by(stepsize)
        .map(|fs| {
            let f = fs as f64 / 1000.0;

            if fs > 0 {
                // calculate revolutions around unit circle
                let processed = wind_unitcircle(data,f);
                // calculate mean X value
                let mx = calc_mean(&processed);
                // drawunitcircle(&processed);
                (f, -mx)
            } else {
                (f, 0.0)
            }
        })
    .collect()
}

fn main() {
    // sampled time in seconds
    let t = 2.0;
    // sample stepsize, ms
    let ss = 10;

    // generate sinewave samples
    let sine0 = sineplot(30.0,t,ss);
    // drawplot(&sine0);
    let sine1 = sineplot(40.00,t,ss);
    // drawplot(&sine1);

    // add/mix two sines together
    let data = sine0.iter()
        .zip(sine1.iter())
        .map(|(t0,t1)|{
            let x = t0.0;
            let y = t0.1 + t1.1;
            (x,y)
        })
    .collect();

    // plot withing f0 - f1 in mHz with stepsize of mHz
    let data_mx = ft_analyse(&data, 25_000, 45_000, 200);
    drawplot(&data_mx);
}
