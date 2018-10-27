// utility module

use std::f64::consts::PI;

use fft;

// generates sinewave
// arguments:
// f - frequency to generate
// t - time in seconds
// ss - stepsize in seconds
fn sinewave(f: f64, p: f64, t: f64, sr: usize) -> fft::Sample {
    // precalculate 2πf
    let fc = 2.0 * PI * f;

    // precalculate phase in radians
    let rad = p * PI / 180.0;

    // sine generator
    // f(t) = amplitude
    let mut data = vec![];
    let dt = 1.0 / sr as f64;
    let mut t0 = 0.0;
    while t0 < t {
        let i = (fc * t0 + rad).sin();
        data.push(i);
        t0 += dt;
    }

    fft::Sample{ data: data, rate: sr }
}

// generates sinewaves from list of (freq,phase) pairs
pub fn generate_sinewaves(t: f64, sr: usize, frequencies: &[(f64,f64)]) -> fft::Sample {
    // mix and generate samples
    fft::Sample{
        data: frequencies.into_iter()
        .fold(vec![],|acc,fp| {
            println!("{} hz, {} phase", fp.0, fp.1);
            if acc.len() > 0 {
                acc.into_iter()
                    .zip(sinewave(fp.0,fp.1,t,sr).data.into_iter())
                    .map(|(t0,t1)|{
                        t0 + t1
                    })
                    .collect()
            }
            else {
                sinewave(fp.0,fp.1,t,sr).data
            }
        }),
        rate: sr
    }
}

// parses list of "freq:phase" strings to vector of (f64,f64)
pub fn parse_freq_phase_pairs(fplist: Vec<String>) -> Vec<(f64,f64)> {
    fplist
    .into_iter()
    .map(|sfp| {
        let components: Vec<&str> = sfp.split(':').collect();
        let freq = components[0].parse::<f64>().expect("failed to parse freq");
        let mut phase = 0.0;
        if components.len() > 1 {
            phase = components[1].parse::<f64>().expect("failed to parse phase");
        }
        (freq, phase)
    })
    .collect()
}

