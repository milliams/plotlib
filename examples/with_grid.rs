extern crate plotlib;

use plotlib::grid::Grid;
use plotlib::style::Line;
use plotlib::view::View;

fn main() {
    let l1 = plotlib::line::Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(plotlib::line::Style::new().colour("burlywood"));
    let grid = Grid::new(3, 8);
    let mut v = plotlib::view::ContinuousView::new().add(&l1);
    v.add_grid(grid);
    plotlib::page::Page::single(&v)
        .save("line.svg")
        .expect("saving svg");
}
