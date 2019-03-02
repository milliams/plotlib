extern crate plotlib;

use plotlib::{repr, view};

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = repr::Scatter::from_slice(&data);
    let s2 = repr::Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(repr::PointStyle::new().marker(repr::PointMarker::Square));
    let v = view::ContinuousView::new()
        .add(Box::new(s1))
        .add(Box::new(s2))
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");
    println!("{}", plotlib::page::Page::single(&v).to_text().unwrap());
}
