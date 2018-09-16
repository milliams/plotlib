extern crate plotlib;

use plotlib::style::BarChart;

fn main() {
    let b1 = plotlib::barchart::BarChart::new(5.3).label("1");
    let b2 = plotlib::barchart::BarChart::new(2.6).label("2");
    //.style(plotlib::boxplot::Style::new().fill("darkolivegreen"));
    let v = plotlib::view::CategoricalView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment");
    plotlib::page::Page::single(&v)
        .save("barchart.svg")
        .expect("saving svg");
}
