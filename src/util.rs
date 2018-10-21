// utility module

use std::f64::consts::PI;
use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::page::Page;

// generates sinewave
// arguments:
// f - frequency to generate
// t - time in seconds
// ss - stepsize in seconds
fn sinewave(f: f64, t: f64, sf: f64) -> Vec<(f64,f64)> {
    // precalculate 2πf
    let fc = 2.0 * PI * f;

    // sine generator
    // f(t) = amplitude
    let mut data = vec![];
    let dt = 1.0 / sf;
    let mut t0 = 0.0;
    while t0 < t {
        data.push((t0,(fc * t0).sin()));
        t0 += dt;
    }

    data
}

pub fn generate_sinewaves(t: f64, sf: f64, frequencies: &[f64]) -> Vec<(f64,f64)> {
    // mix and generate samples
    frequencies.into_iter()
        .fold(vec![],|acc,f| {
            println!("add {} hz", f);
            if acc.len() > 0 {
                acc.into_iter()
                    .zip(sinewave(*f,t,sf).into_iter())
                    .map(|(t0,t1)|{
                        let x = t0.0;
                        let y = t0.1 + t1.1;
                        (x,y)
                    })
                    .collect()
            }
            else {
                sinewave(*f,t,sf)
            }
        })
}


// draws linear time plot
pub fn drawplot(data: &Vec<(f64,f64)>) {
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
pub fn drawunitcircle(data: &Vec<(f64,f64)>) {
    // We create our scatter plot from the data
    let s1 = Scatter::from_vec(data);

    // The 'view' describes what set of data is drawn
    let v = View::new()
        .add(&s1)
        .x_range(-1.0,1.0)
        .y_range(-1.0,1.0);

    println!("{}", Page::single(&v).to_text());
}
