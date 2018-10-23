// plotting / graphing utility functions
use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::page::Page;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Chart, Axis, Marker, Dataset, Block, Borders};
use tui::style::{Style, Color};
use tui::layout::Rect;
use std::io::{Stdout, stdout};

// draws linear time plot
pub fn drawplot(data: &Vec<(f64,f64)>) {
    // We create our scatter plot from the data
    let s1 = Scatter::from_vec(data);

    let x0 = data.first().unwrap().0.floor();
    let x1 = data.last().unwrap().0.ceil();

    // The 'view' describes what set of data is drawn
    let v = View::new()
        .add(&s1)
        .x_range(x0, x1)
        .y_range(-2.0, 2.0);

    println!("{}", Page::single(&v).to_text());
}

// draws plot in a unit circle
pub fn drawcircle(data: &Vec<(f64,f64)>) {
    // plot scale from min/max values
    let r = data.iter().fold(0.0, |acc: f64,xy|{
        acc.max(xy.0).max(xy.1)
    }).ceil();

    // We create our scatter plot from the data
    let s1 = Scatter::from_vec(data);

    // The 'view' describes what set of data is drawn
    let v = View::new()
        .add(&s1)
        .x_range(-r,r)
        .y_range(-r,r);

    println!("{}", Page::single(&v).to_text());
}

// initializes and returns tui terminal
pub fn get_tui() -> Terminal<TermionBackend<Stdout>> {
    Terminal::new(TermionBackend::with_stdout(stdout())).unwrap()
}

pub fn draw_circle_graph(mut term: &mut Terminal<TermionBackend<Stdout>>, data: &[(f64,f64)]) {
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
    let r = data.iter().fold(0.0, |acc: f64,xy|{
        acc.max(xy.1)
    }).ceil();

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
