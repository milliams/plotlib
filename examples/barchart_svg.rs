use plotlib::barchart::{BarChart, Style};
use plotlib::page::Page;
// XXX only supports rust 1.31, however we cannot import the BarChart trait and BarChart struct at
// the same time
use plotlib::style::BarChart as _;
use plotlib::view::CategoricalView;

fn main() {
    let b1 = BarChart::new(5.3).label("1");
    let b2 = BarChart::new(2.6)
        .label("2")
        .style(Style::new().fill("darkolivegreen"));

    let v = CategoricalView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment");

    Page::single(&v).save("barchart.svg").expect("saving svg");
}
