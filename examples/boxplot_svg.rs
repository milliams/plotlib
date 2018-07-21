extern crate plotlib;

use plotlib::style::BoxPlot;

fn main() {
    let b1 = plotlib::boxplot::BoxPlot::from_slice(&[1.0, 4.0, 2.0, 3.5, 6.4, 2.5, 7.5, 1.8, 9.6])
        .label("1");
    let b2 = plotlib::boxplot::BoxPlot::from_slice(&[3.0, 4.3, 2.0, 3.5, 6.9, 4.5, 7.5, 1.8, 10.6])
        .label("2")
        .style(plotlib::boxplot::Style::new().fill("darkolivegreen"));
    let v = plotlib::view::DiscreteView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment")
        .y_label("y");
    plotlib::page::Page::single(&v).dimensions(400, 300).save("boxplot.svg");
}
