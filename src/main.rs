// cli interface for lib

extern crate fft;
extern crate argparse;
extern crate plotlib;
extern crate tui;
extern crate termion;
extern crate num_complex;

mod plot;
mod generator;

use termion::input::TermRead;
use std::io::stdin;
use argparse::{ArgumentParser, Store, StoreOption, List};
use fft::util;

fn main() {
    // sine sample generation args
    let mut gen_t = 2.0; // time secs
    let mut gen_sf: usize = 44_100; // sampling frequency
    let mut gen_frequencies: Vec<String> = vec![]; // frequencies to generate

    // ft analysis args
    let mut ft_min = 1.0;  // lower bound, hz
    let mut ft_max = 100.0;  // upper bound, hz
    let mut ft_ss = 1.0;    // stepsize, hz
    let mut ft_res: Option<f64> = None;

    {
        // parse arguments
        let mut ap = ArgumentParser::new();
        ap.set_description("sample ft analysis tool");
        // gen args
        ap.refer(&mut gen_t)
            .add_option(&["--t"], Store,
            "sine generation length in seconds");
        ap.refer(&mut gen_sf)
            .add_option(&["--samplerate", "--rate"], Store,
            "sine generation sampling frequency in hz");
        ap.refer(&mut gen_frequencies)
            .add_option(&["--freqs"], List,
            "sine frequencies generated as list of hz");
        // ft args
        ap.refer(&mut ft_min)
            .add_option(&["--min"], Store,
            "FT analysis min, hz");
        ap.refer(&mut ft_max)
            .add_option(&["--max"], Store,
            "FT analysis max, hz");
        ap.refer(&mut ft_ss)
            .add_option(&["--ss", "--stepsize"], Store,
            "FT analysis step size, hz");
        ap.refer(&mut ft_res)
            .add_option(&["--res", "--resolution"], StoreOption,
            "FT analysis resolution, overrides stepsize if given");
        ap.parse_args_or_exit();
    }

    // Override stepsize if using point resolution
    if ft_res.is_some() {
        // calculate stepsize
        ft_ss = (ft_max - ft_min) / ft_res.unwrap();
    }

    // create sample
    let mut sample = fft::Sample{ data: vec![], rate: gen_sf };
    if gen_frequencies.len() > 0 {
        sample = util::generate_sinewaves(gen_t, sample.rate, &util::parse_freq_phase_pairs(gen_frequencies));
    }
    else {
        // get sample from input
    }

    // run analysis
    // plots realtime text graph
    if !sample.is_empty() {
        let mut ft_data: Vec<fft::Phasor> = vec![];
        let mut f = ft_min;
        let mut term = plot::get_tui();
        term.hide_cursor().unwrap();
        term.clear().unwrap();
        let mut peaks: Vec<usize> = vec![];

        // process graphs
        while f <= ft_max {
            plot::draw_waveform(&mut term, &sample, 0.0, gen_t);
            plot::draw_circle(&mut term, &fft::graph_circle(&sample,f));
            ft_data.push(fft::Phasor{ 
                frequency: f, 
                complex: fft::analyze_freq(&sample,f)
            });
            if ft_data.len() > 3 {
                let peak = fft::max(&ft_data[(ft_data.len() - 3)..]);
                if peak == 1 {
                    // peak local -> global
                    let peak = ft_data.len() - 3 + peak;
                    peaks.push(peak);
                }
            }
            plot::draw_frequency_graph(&mut term, &ft_data[..], ft_min, ft_max);

            // increment f
            f += ft_ss;

            if f > ft_max {
                // show peaks
                peaks = util::adjust_peaks(&ft_data, &peaks);
                plot::draw_peaks(&mut term,
                                 peaks.iter().map(|&p| &ft_data[p]).collect(),
                                 ft_min, ft_max);
            }
            term.draw().unwrap();
        }

        // stop for key events
        for c in stdin().keys() {
            match c.unwrap() {
                _ => break
            }
        }

        // clean up
        term.show_cursor().unwrap();
        term.clear().unwrap();
    }
}
