use plotlib::repr::{Histogram, HistogramBins};
use plotlib::page::Page;
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
        .style(BoxStyle::new().fill("burlywood"));

    let v = ContinuousView::new().add(h);

    Page::single(&v).save("histogram.svg").expect("saving svg");
}
