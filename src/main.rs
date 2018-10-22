// cli interface for lib

extern crate fft;
extern crate argparse;
extern crate plotlib;
extern crate tui;
extern crate termion;

mod util;
mod plot;

use argparse::{ArgumentParser, Store, StoreTrue, StoreOption, List};

fn main() {
    // sine sample generation args
    let mut gen_t = 2.0; // time secs
    let mut gen_sf = 44_100.0; // sampling frequency
    let mut gen_frequencies: Vec<String> = vec![]; // frequencies to generate

    let mut plot_sample = false;
    let mut plot_circle = false;

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
        ap.refer(&mut plot_sample)
            .add_option(&["--plot-sample"], StoreTrue,
            "draws given sample as a chart");
        ap.refer(&mut plot_circle)
            .add_option(&["--plot-circle"], StoreTrue,
            "draws winding graphs");
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

    let mut sample: (Vec<f64>,f64) = (vec![], gen_sf);
    if sample.0.len() == 0 && gen_frequencies.len() > 0 {
        sample.0 = util::generate_sinewaves(gen_t,gen_sf, &util::parse_freq_phase_pairs(gen_frequencies));

        if plot_sample {
            plot::drawplot(&sample.0.iter().enumerate().map(|(idx,&x)|{
                (idx as f64 / sample.1, x)
            }).collect());
        }

        sample.1 = gen_sf;
        println!("Generated sinewaves: sampling time of {} seconds, sampling frequency {} hz", gen_t, gen_sf);
    }

    if sample.0.len() > 0 {
        // run analysis
        let mut ft_data: Vec<(f64,f64)> = vec![];
        let mut f = ft_min;
        let mut term = plot::get_tui();
        term.hide_cursor().unwrap();
        term.clear().unwrap();
        while f <= ft_max {
            if plot_circle {
                plot::draw_circle_graph(&mut term, &fft::graph_circle(&sample.0[..],gen_sf,f));
            }
            ft_data.push((f, fft::analyze_freq((&sample.0[..], gen_sf),f)));
            plot::draw_plot(&mut term, &ft_data, ft_min, ft_max);
            term.draw().unwrap();
            f += ft_ss;
        }
        term.show_cursor().unwrap();
        term.clear().unwrap();

        // all done
    }
}
