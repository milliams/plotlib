use plotlib::barchart::BarChart;
use plotlib::barchart::Style as BarChartStyle;
use plotlib::grid::Grid;
use plotlib::line::Line;
use plotlib::line::Style as LineStyle;
use plotlib::page::Page;
// XXX only supports rust 1.31, however we cannot import the BarChart trait and BarChart struct at
// the same time
use plotlib::style::BarChart as _;
use plotlib::style::Line as _;
use plotlib::view::{CategoricalView, ContinuousView, View};

fn main() {
    render_line_chart("line_with_grid.svg");
    render_barchart("barchart_with_grid.svg");
}

fn render_line_chart<S>(filename: S)
where
    S: AsRef<str>,
{
    let l1 = Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(LineStyle::new().colour("burlywood"));
    let mut v = ContinuousView::new().add(&l1);
    v.add_grid(Grid::new(3, 8));
    Page::single(&v)
        .save(filename.as_ref())
        .expect("saving svg");
}

fn render_barchart<S>(filename: S)
where
    S: AsRef<str>,
{
    let b1 = BarChart::new(5.3).label("1");
    let b2 = BarChart::new(2.6)
        .label("2")
        .style(BarChartStyle::new().fill("darkolivegreen"));
    let mut v = CategoricalView::new()
        .add(&b1)
        .add(&b2)
        .x_label("Experiment");
    v.add_grid(Grid::new(3, 8));
    Page::single(&v)
        .save(filename.as_ref())
        .expect("saving svg");
}
