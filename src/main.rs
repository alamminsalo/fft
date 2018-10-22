extern crate fft;
extern crate argparse;

use fft::util;
use argparse::{ArgumentParser, Store, StoreTrue, StoreOption, List};

fn parseFreqPhasePairs(fplist: Vec<String>) -> Vec<(f64,f64)> {
    fplist
    .into_iter()
    .map(|sfp| {
        let mut freq = 0.0;
        let mut phase = 0.0;
        let components: Vec<&str> = sfp.split(':').collect();
        freq = components[0].parse::<f64>().expect("failed to parse freq");
        if components.len() > 1 {
            phase = components[1].parse::<f64>().expect("failed to parse phase");
        }
        (freq, phase)
    })
    .collect()
}

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
        sample.0 = util::generate_sinewaves(gen_t,gen_sf, &parseFreqPhasePairs(gen_frequencies));

        if plot_sample {
            util::drawplot(&sample.0.iter().enumerate().map(|(idx,&x)|{
                (idx as f64 / sample.1, x)
            }).collect());
        }

        sample.1 = gen_sf;
        println!("Generated sinewaves: sampling time of {} seconds, sampling frequency {} hz", gen_t, gen_sf);
    }

    if sample.0.len() > 0 {
        // run analysis
        let ft_data = fft::analyze((&sample.0[..],gen_sf), ft_min, ft_max, ft_ss, plot_circle);

        // finally, draw FT plot
        util::drawplot(&ft_data);
    }
}
