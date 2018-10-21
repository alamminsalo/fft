extern crate fft;
extern crate argparse;

use fft::util;
use argparse::{ArgumentParser, Store, List};

fn main() {
    // sine sample generation args
    let mut gen_t = 2.0; // time secs
    let mut gen_sf = 44_100.0; // sampling frequency
    let mut gen_frequencies: Vec<f64> = vec![]; // frequencies to generate

    // ft analysis args
    let mut ft_min = 1.0;  // lower bound, hz
    let mut ft_max = 100.0;  // upper bound, hz
    let mut ft_ss = 1.0;    // stepsize, hz

    {
        // parse arguments
        let mut ap = ArgumentParser::new();
        ap.set_description("sample ft analysis tool");
        // gen args
        ap.refer(&mut gen_t)
            .add_option(&["--t"], Store,
            "sine generation length in seconds");
        ap.refer(&mut gen_sf)
            .add_option(&["--sf"], Store,
            "sine generation sampling frequency in hz");
        ap.refer(&mut gen_frequencies)
            .add_option(&["--freqs"], List,
            "sine frequencies generated as list of hz");
        // ft args
        ap.refer(&mut ft_min)
            .add_option(&["--min"], Store,
            "sine frequencies generated as list of hz");
        ap.refer(&mut ft_max)
            .add_option(&["--max"], Store,
            "sine frequencies generated as list of hz");
        ap.refer(&mut ft_ss)
            .add_option(&["--ss", "--stepsize"], Store,
            "sine frequencies generated as list of hz");
        ap.parse_args_or_exit();
    }

    let mut data = vec![];
    if data.len() == 0 && gen_frequencies.len() > 0 {
        data = util::generate_sinewaves(gen_t,gen_sf,&gen_frequencies);
        println!("Generated sinewaves: sampling time of {} seconds, sampling frequency {} hz", gen_t, gen_sf);
    }

    if data.len() > 0 {
        // run analysis
        let ft_data = fft::analyze(&data, ft_min, ft_max, ft_ss);

        // finally, draw FT plot
        util::drawplot(&ft_data);
    }
    else {
        println!("nothing to do");
        println!("try --help");
    }
}
