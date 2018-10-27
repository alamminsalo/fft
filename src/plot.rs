// plotting / graphing utility functions
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Chart, Axis, Marker, Dataset, Block, Borders};
use tui::style::{Style, Color};
use tui::layout::Rect;
use std::io::{Stdout, stdout};
use num_complex::Complex;
use fft;

type DTerm = Terminal<TermionBackend<Stdout>>;

// initializes and returns tui terminal
pub fn get_tui() -> DTerm {
    Terminal::new(TermionBackend::with_stdout(stdout())).unwrap()
}

// returns phasor freq, vector length
fn phasor_to_freqs(data: &[fft::Phasor]) -> Vec<(f64,f64)> {
    data.iter().map(|p| (p.frequency, p.complex.norm_sqr())).collect()
}

fn phasor_ref_to_freqs(data: &[&fft::Phasor]) -> Vec<(f64,f64)> {
    data.iter().map(|&p| (p.frequency, p.complex.norm_sqr())).collect()
}

fn complex_to_cartesian_tuples(data: &[Complex<f64>]) -> Vec<(f64,f64)> {
    data.iter().map(|&c| (c.re, c.im)).collect()
}

pub fn draw_circle(mut term: &mut DTerm, data: &[Complex<f64>]) {
    // plot scale from min/max values
    let r = 2.0;
    // let r = data.iter().fold(0.0, |acc: f64,xy|{
    //     acc.max(xy.0).max(xy.1)
    // }).ceil();

    let size = &term.size().unwrap();
    let h = size.height / 2;
    let w = (h * 2).min(size.width);

    Chart::<&str,&str>::default()
        .block(Block::default()
               .title("2d plane")
               .borders(Borders::ALL))
        .x_axis(Axis::default()
                .bounds([-r,r]))
        .y_axis(Axis::default()
                .bounds([-r,r]))
        .datasets(&[Dataset::default()
                  .marker(Marker::Braille)
                  .style(Style::default().fg(Color::White))
                  .data(&complex_to_cartesian_tuples(data))])
        .render(&mut term, &Rect::new(0,0,w,h));
}

pub fn draw_waveform(mut term: &mut DTerm, sample: &fft::Sample, min: f64, max: f64) {
    // plot scale from min/max values
    let r = sample.max_amplitude().ceil();

    let size = &term.size().unwrap();
    let x = size.height;
    let y = 0;
    let h = size.height / 2;
    let w = size.width - x;

    Chart::<&str,&str>::default()
        .block(Block::default()
               .title("Waveform")
               .borders(Borders::ALL))
        .x_axis(Axis::default()
                .bounds([min.floor(),max.ceil()])
                .labels(&[&min.to_string(), 
                        &(min + (max - min) / 4.0).to_string(),
                        &(min + (max - min) / 2.0).to_string(),
                        &(min + (max - min) / 1.3333333333333333333).to_string(),
                        &max.to_string()])
                )
        .y_axis(Axis::default()
                .bounds([-r,r]))
        .datasets(&[Dataset::default()
                  .marker(Marker::Braille)
                  .style(Style::default().fg(Color::White))
                  .data(&sample.with_time())])
        .render(&mut term, &Rect::new(x,y,w,h));
}

pub fn draw_frequency_graph(mut term: &mut DTerm, data: &[fft::Phasor], min: f64, max: f64) {
    // plot scale from min/max values
    let r = 1.0;
    // let r = data.iter().fold(0.0, |acc: f64,xy|{
    //     acc.max(xy.1)
    // }).ceil();

    let size = &term.size().unwrap();
    let y = size.height / 2;
    let h = y;
    let w = size.width;

    Chart::<&str,&str>::default()
        .block(Block::default()
               .title("FT analysis")
               .borders(Borders::ALL))
        .x_axis(Axis::default()
                .bounds([min.floor(),max.ceil()])
                .labels(&[&min.to_string(), 
                        &(min + (max - min) / 4.0).to_string(),
                        &(min + (max - min) / 2.0).to_string(),
                        &(min + (max - min) / 1.3333333333333333333).to_string(),
                        &max.to_string()])
                )
        .y_axis(Axis::default()
                .bounds([-r,r]))
        .datasets(&[Dataset::default()
                  .marker(Marker::Braille)
                  .style(Style::default().fg(Color::White))
                  .data(&phasor_to_freqs(data))])
        .render(&mut term, &Rect::new(0,y,w,h));
}

pub fn draw_peaks(mut term: &mut DTerm, data: Vec<&fft::Phasor>, min: f64, max: f64) {
    // plot scale from min/max values
    let r = 1.0;
    let size = &term.size().unwrap();
    let y = size.height / 2;
    let h = y;
    let w = size.width;

    Chart::<&str,&str>::default()
        .x_axis(Axis::default()
                .bounds([min.floor(),max.ceil()]))
        .y_axis(Axis::default()
                .bounds([-r,r]))
        .datasets(&[Dataset::default()
                  .marker(Marker::Dot)
                  .style(Style::default().fg(Color::Red))
                  .data(&phasor_ref_to_freqs(&data[..]))])
        .render(&mut term, &Rect::new(0,y,w,h));
}
