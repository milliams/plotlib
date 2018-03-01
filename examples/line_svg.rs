extern crate plotlib;

use plotlib::style::Line;

fn main() {
    let l1 = plotlib::line::Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(plotlib::line::Style::new().colour("burlywood"));
    let v = plotlib::view::View::new().add(&l1);
    plotlib::page::Page::single(&v).save("line.svg");
}
