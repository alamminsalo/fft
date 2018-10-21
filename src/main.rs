extern crate num_complex;
extern crate plotlib;

use std::f64::consts::PI;
use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::page::Page;
use num_complex::Complex;

// i -- imaginary number
const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

fn sineplot(ts: f64, f: f64, ss: usize) -> Vec<(f64,f64)> {
    println!("sine: {} hz for {} seconds.", f, ts);
    println!("{} sampling frequency", ss);

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

fn main() {
    // generate sinewave samples
    let sine0 = sineplot(2.0,30.00,10);
    // drawplot(&sine0);

    let sine1 = sineplot(2.0,40.00,10);
    // drawplot(&sine1);

    // add two sines together
    let data = sine0.iter()
        .zip(sine1.iter())
        .map(|(t0,t1)|{
            let x = t0.0;
            let y = t0.1 + t1.1;
            (x,y)
        })
        .collect();

    // plot withing f0 - f1 in mHz
    let data_mx: Vec<(f64,f64)> = (25_000..45_000)
        .step_by(200)
        .map(|fs| {
            let f = fs as f64 / 1000.0;

            if fs > 0 {
                // calculate revolutions around unit circle
                let processed = wind_unitcircle(&data,f);
                // calculate mean X value
                let mx = processed
                    .into_iter()
                    .enumerate()
                    .fold(0.0, |mean, (idx, x)| {
                        if idx == 0 {
                            x.0
                        }
                        else {
                            ((mean * (idx as f64)) + x.1) / (idx as f64 + 1.0)
                        }
                    });
                // drawunitcircle(&processed);
                (f, -mx)
            } else {
                (f, 0.0)
            }
        })
    .collect();

    println!("FT analysis graph");
    drawplot(&data_mx);
}
