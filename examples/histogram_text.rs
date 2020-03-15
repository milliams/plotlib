use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::view::ContinuousView;

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = Histogram::from_slice(&data, HistogramBins::Count(10));

    let v = ContinuousView::new().add(h);

    println!("{}", Page::single(&v).dimensions(60, 15).to_text().unwrap());
}
