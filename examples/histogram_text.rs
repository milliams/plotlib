extern crate plotlib;

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = plotlib::histogram::Histogram::from_slice(&data, plotlib::histogram::Bins::Count(10));
    let v = plotlib::view::ContinuousView::new().add(&h);
    println!("{}", plotlib::page::Page::single(&v).dimensions(60, 15).to_text().unwrap());
}
