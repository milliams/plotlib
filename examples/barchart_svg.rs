extern crate plotlib;

use plotlib::{repr, view, page::Page};

fn main() {
    let b1 = repr::BarChart::new(5.3).label("1");
    let b2 = repr::BarChart::new(2.6)
        .label("2")
        .style(repr::BoxStyle::new().fill("darkolivegreen"));
    let v = view::CategoricalView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment");
    Page::single(&v)
        .save("barchart.svg")
        .expect("saving svg");
}
