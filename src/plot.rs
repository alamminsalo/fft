// plotting / graphing utility functions
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Chart, Axis, Marker, Dataset, Block, Borders};
use tui::style::{Style, Color};
use tui::layout::Rect;
use std::io::{Stdout, stdout};

// initializes and returns tui terminal
pub fn get_tui() -> Terminal<TermionBackend<Stdout>> {
    Terminal::new(TermionBackend::with_stdout(stdout())).unwrap()
}

pub fn draw_circle_graph(mut term: &mut Terminal<TermionBackend<Stdout>>, data: &[(f64,f64)]) {
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
                  .data(data)])
        .render(&mut term, &Rect::new(0,0,w,h));
}

pub fn draw_plot_1(mut term: &mut Terminal<TermionBackend<Stdout>>, data: &[(f64,f64)], min: f64, max: f64) {
    // plot scale from min/max values
    let r = data.iter().fold(0.0, |acc: f64,xy|{
        acc.max(xy.1)
    }).ceil();

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
                  .data(data)])
        .render(&mut term, &Rect::new(x,y,w,h));
}

pub fn draw_plot_2(mut term: &mut Terminal<TermionBackend<Stdout>>, data: &[(f64,f64)], min: f64, max: f64) {
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
                  .data(data)])
        .render(&mut term, &Rect::new(0,y,w,h));
}

pub fn draw_plot_2_peaks(mut term: &mut Terminal<TermionBackend<Stdout>>, data: &[(f64,f64)], min: f64, max: f64) {
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
                  .data(data)])
        .render(&mut term, &Rect::new(0,y,w,h));
}
