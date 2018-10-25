// cli interface for lib

extern crate fft;
extern crate argparse;
extern crate plotlib;
extern crate tui;
extern crate termion;

mod util;
mod plot;

use termion::input::TermRead;
use std::io::stdin;
use argparse::{ArgumentParser, Store, StoreOption, List};

fn main() {
    // sine sample generation args
    let mut gen_t = 2.0; // time secs
    let mut gen_sf = 44_100.0; // sampling frequency
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
    let mut sample: (Vec<f64>,f64) = (vec![], gen_sf);
    if sample.0.len() == 0 && gen_frequencies.len() > 0 {
        sample.0 = util::generate_sinewaves(gen_t,gen_sf, &util::parse_freq_phase_pairs(gen_frequencies));
        sample.1 = gen_sf;
    }
    else {
        // get sample from input
    }

    // run analysis
    // plots realtime text graph
    if sample.0.len() > 0 {
        let mut ft_data: Vec<(f64,f64)> = vec![];
        let mut f = ft_min;
        let mut term = plot::get_tui();
        term.hide_cursor().unwrap();
        term.clear().unwrap();
        let waveform: Vec<(f64,f64)> = sample.0.iter().enumerate().map(|(idx,&x)|{
                (idx as f64 / sample.1, x)
            }).collect();

        let mut peaks: Vec<(f64,f64)> = vec![];

        // process graphs
        while f <= ft_max {
            plot::draw_plot_1(&mut term, &waveform[..],0.0,gen_t);
            plot::draw_circle_graph(&mut term, &fft::graph_circle(&sample.0[..],gen_sf,f));
            ft_data.push((f, fft::analyze_freq((&sample.0[..], gen_sf),f)));
            if ft_data.len() > 5 {
                let peak = fft::max(&ft_data[(ft_data.len() - 5)..]);
                if peak > 1 && peak < 3 {
                    peaks.push(ft_data[ft_data.len() - 5 + peak]);
                }
            }
            plot::draw_plot_2(&mut term, &ft_data, ft_min, ft_max);
            plot::draw_plot_2_peaks(&mut term, &peaks, ft_min, ft_max);
            term.draw().unwrap();

            f += ft_ss;
        }

        // key events
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
