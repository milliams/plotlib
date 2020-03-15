use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;

fn main() {
    let b1 = BarChart::new(5.3).label("1");
    let b2 = BarChart::new(2.6)
        .label("2")
        .style(&BoxStyle::new().fill("darkolivegreen"));

    let v = CategoricalView::new().add(b1).add(b2).x_label("Experiment");

    Page::single(&v).save("barchart.svg").expect("saving svg");
}
