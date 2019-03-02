extern crate plotlib;

use plotlib::repr::{Line, LineStyle};

fn main() {
    let l1 = Line::new(vec![(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(LineStyle::new().colour("burlywood"));
    let v = plotlib::view::ContinuousView::new().add(Box::new(l1));
    plotlib::page::Page::single(&v)
        .save("line.svg")
        .expect("saving svg");
}
