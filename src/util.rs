// utility module

use std::f32::consts::PI;
use super::Sample;

// generates sinewave
// arguments:
// f - frequency to generate
// t - time in seconds
// ss - stepsize in seconds
pub fn sinewave(f: f32, p: f32, t: f32, sr: usize, a: f32) -> Sample {
    // precalculate 2πf
    let fc = 2.0 * PI * f;

    // precalculate phase in radians
    let rad = p * PI / 180.0;

    // sine generator
    // f(t) = amplitude
    let mut data = vec![];
    let dt = 1.0 / sr as f32;
    let mut t0 = 0.0;
    while t0 < t {
        let i = (fc * t0 + rad).sin() * a;
        data.push(i);
        t0 += dt;
    }

    Sample{ data: data, rate: sr }
}

// generates sinewaves from list of (freq,phase) pairs
pub fn sinewaves(t: f32, sr: usize, frequencies: &[(f32,f32)]) -> Sample {
    // mix and generate samples
    Sample{
        data: frequencies.into_iter()
        .fold(vec![],|acc,fp| {
            println!("{} hz, {} phase", fp.0, fp.1);
            if acc.len() > 0 {
                acc.into_iter()
                    .zip(sinewave(fp.0,fp.1,t,sr,1.0).data.into_iter())
                    .map(|(t0,t1)|{
                        t0 + t1
                    })
                    .collect()
            }
            else {
                sinewave(fp.0,fp.1,t,sr,1.0).data
            }
        }),
        rate: sr
    }
}

// parses list of "freq:phase" strings to vector of (f32,f32)
pub fn parse_freq_phase_pairs(fplist: Vec<String>) -> Vec<(f32,f32)> {
    fplist
    .into_iter()
    .map(|sfp| {
        let components: Vec<&str> = sfp.split(':').collect();
        let freq = components[0].parse::<f32>().expect("failed to parse freq");
        let mut phase = 0.0;
        if components.len() > 1 {
            phase = components[1].parse::<f32>().expect("failed to parse phase");
        }
        (freq, phase)
    })
    .collect()
}

// adjusts peaks location + normalizes peaks
pub fn adjust_peaks(phasors: &[super::Phasor], peaks: &[usize]) -> Vec<usize> {
    let peaks_amp: Vec<(f32, usize)> = peaks.iter()
        .map(|&p0| {
            let p0_amp = phasors[p0].complex.to_polar().0;
            let mut p1 = p0 + 1;
            while phasors.len() > p1 && phasors[p1].complex.to_polar().0 == p0_amp {
                p1 += 1;
            }
            // return floor(center)
            (p0_amp, p0 + (p1 as f32 - p0 as f32).floor() as usize)
        })
    .collect();

    // get max amplitude
    let p_max: f32 = peaks_amp.iter().fold(0.0, |acc, p| acc.max(p.0));

    let mut peaks_normalized: Vec<usize> = peaks_amp.iter()
        .filter(|&p|{ p.0 > p_max * 0.3333333 })
        .map(|&p| p.1)
        .collect();
    peaks_normalized.dedup();

    peaks_normalized
}

