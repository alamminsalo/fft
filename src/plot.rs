// plotting / graphing utility functions
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Chart, Axis, Marker, Dataset, Block, Borders};
use tui::style::{Style, Color};
use tui::layout::Rect;
use tui::buffer::Buffer;
use std::io::{Stdout, stdout};
use num_complex::Complex;
use std::f32::consts::PI;
use fft;

type DTerm = Terminal<TermionBackend<Stdout>>;

// initializes and returns tui terminal
pub fn get_tui() -> DTerm {
    Terminal::new(TermionBackend::with_stdout(stdout())).unwrap()
}

// returns phasor freq, vector length
fn phasor_to_plot(data: &[fft::Phasor]) -> Vec<(f64,f64)> {
    data.iter().map(|p| (p.frequency as f64, p.complex.to_polar().0 as f64)).collect()
}

fn phasor_ref_to_plot(data: &[&fft::Phasor]) -> Vec<(f64,f64)> {
    data.iter().map(|&p| (p.frequency as f64, p.complex.to_polar().0 as f64)).collect()
}

fn complex_to_plot(data: &[Complex<f32>]) -> Vec<(f64,f64)> {
    data.iter().map(|&c| (c.re as f64, c.im as f64)).collect()
}

pub fn draw_circle(mut term: &mut DTerm, data: &[Complex<f32>]) {
    let data = complex_to_plot(data);
    // plot scale from min/max values
    let r = data.iter().fold(0.0, |acc: f64,xy|{
        acc.max(xy.0).max(xy.1)
    }).ceil();

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
                  .data(&data)])
        .render(&mut term, &Rect::new(0,0,w,h));
}

pub fn draw_waveform(mut term: &mut DTerm, sample: &fft::Sample, min: f32, max: f32) {
    // plot scale from min/max values
    let r = sample.max_amplitude().ceil() as f64;

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
                .bounds([min.floor() as f64,max.ceil() as f64])
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

pub fn draw_frequency_graph(mut term: &mut DTerm, data: &[fft::Phasor], min: f32, max: f32) {
    // plot scale from min/max values
    let r = 1.0;
    // let r = data.iter().fold(0.0, |acc: f32,xy|{
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
                .bounds([min.floor() as f64,max.ceil() as f64])
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
                  .data(&phasor_to_plot(data))])
        .render(&mut term, &Rect::new(0,y,w,h));
}


struct Label<'a> {
    text: &'a str,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label { text: "" }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, &Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
}

pub fn draw_peaks(mut term: &mut DTerm, data: Vec<&fft::Phasor>, min: f32, max: f32) {
    // plot scale from min/max values
    let r = 1.0;
    let size = &term.size().unwrap();
    let y = size.height / 2;
    let h = y;
    let w = size.width;

    Chart::<&str,&str>::default()
        .x_axis(Axis::default()
                .bounds([min.floor() as f64,max.ceil() as f64]))
        .y_axis(Axis::default()
                .bounds([-r,r]))
        .datasets(&[Dataset::default()
                  .marker(Marker::Dot)
                  .style(Style::default().fg(Color::Red))
                  .data(&phasor_ref_to_plot(&data[..]))])
        .render(&mut term, &Rect::new(0,y,w,h));

    // draw labels
    let l = max - min;
    data.iter().for_each(|&p|{
        let x = (w as f32 * (p.frequency - min) / l.max(0.1)) as u16;
        let (r,theta) = p.complex.to_polar();
        let degrees = theta * 180.0 / PI;
        Label::default()
            .text(&format!("{:.2}:{:.2}", p.frequency, degrees))
            .render(&mut term, 
                    &Rect::new(x,y + (h / 2) - (r * (h / 2) as f32) as u16,0,0));
    });
}
