extern crate plotlib;

use plotlib::grid::Grid;
use plotlib::style::BarChart;
use plotlib::style::Line;
use plotlib::view::View;

fn main() {
    render_line_chart("line_with_grid.svg");
    render_barchart("barchart_with_grid.svg");
}

fn render_line_chart<S>(filename: S)
where
    S: AsRef<str>,
{
    let l1 = plotlib::line::Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(plotlib::line::Style::new().colour("burlywood"));
    let mut v = plotlib::view::ContinuousView::new().add(&l1);
    v.add_grid(Grid::new(3, 8));
    plotlib::page::Page::single(&v)
        .save(filename.as_ref())
        .expect("saving svg");
}

fn render_barchart<S>(filename: S)
where
    S: AsRef<str>,
{
    let b1 = plotlib::barchart::BarChart::new(5.3).label("1");
    let b2 = plotlib::barchart::BarChart::new(2.6)
        .label("2")
        .style(plotlib::barchart::Style::new().fill("darkolivegreen"));
    let mut v = plotlib::view::CategoricalView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment");
    v.add_grid(Grid::new(3, 8));
    plotlib::page::Page::single(&v)
        .save(filename.as_ref())
        .expect("saving svg");
}
