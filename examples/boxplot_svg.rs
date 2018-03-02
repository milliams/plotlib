extern crate plotlib;

use plotlib::style::BoxPlot;

fn main() {
    let b1 = plotlib::boxplot::Box::from_slice(&[1.0, 4.0, 2.0, 3.5, 6.4, 2.5, 7.5, 1.8, 9.6])
        .label("1");
    let v = plotlib::view::DiscreteView::new()
        .add(&b1)
        .x_label("Experiment");
    plotlib::page::Page::single(&v).save("boxplot.svg");
}
