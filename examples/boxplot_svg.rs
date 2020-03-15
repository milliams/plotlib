use plotlib::page::Page;
use plotlib::repr::BoxPlot;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;

fn main() {
    let b1 = BoxPlot::from_slice(&[1.0, 4.0, 2.0, 3.5, 6.4, 2.5, 7.5, 1.8, 9.6]).label("1");
    let b2 = BoxPlot::from_slice(&[3.0, 4.3, 2.0, 3.5, 6.9, 4.5, 7.5, 1.8, 10.6])
        .label("2")
        .style(&BoxStyle::new().fill("darkolivegreen"));

    let v = CategoricalView::new()
        .add(b1)
        .add(b2)
        .x_label("Experiment")
        .y_label("y");

    Page::single(&v)
        .dimensions(400, 300)
        .save("boxplot.svg")
        .expect("saving svg");
}
