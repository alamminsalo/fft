use plotlib::scatter::Scatter;
use plotlib::view::View;
use plotlib::page::Page;

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
