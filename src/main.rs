extern crate fft;

use fft::util;

fn main() {
    // sampled time in seconds
    let t = 3.0;
    // sampling frequency hz
    let sf = 44_100.0;
    println!("sampling time of {} seconds, sampling frequency {} hz", t, sf);

    // generate sinewave samples
    let sine0 = util::generate_sinewave(30.0,t,sf);
    // drawplot(&sine0);
    let sine1 = util::generate_sinewave(40.00,t,sf);
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

    // run analysis
    let ft_min = 25.0;  // lower bound, hz
    let ft_max = 45.0;  // upper bound, hz
    let ft_ss = 0.25;    // stepsize, hz
    let ft_data = fft::analyze(&data, ft_min, ft_max, ft_ss);

    // finally, draw FT plot
    util::drawplot(&ft_data);
}
