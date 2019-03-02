extern crate plotlib;

use plotlib::{repr, view};

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = repr::Histogram::from_slice(&data, repr::Bins::Count(10))
        .style(repr::BoxStyle::new().fill("burlywood"));
    let v = plotlib::view::ContinuousView::new().add(Box::new(h));
    plotlib::page::Page::single(&v)
        .save("histogram.svg")
        .expect("saving svg");
}
