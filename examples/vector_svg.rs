use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::Vector;
use plotlib::style::LineStyle;
use plotlib::view::{
    ContinuousView, ContinuousViewAxisIntersectionStyle, ContinuousViewAxisLabelStyle,
    ContinuousViewAxisLineStyle, ContinuousViewStyle, ContinuousViewTickStyle, View,
};

fn main() {
    let v1 = Vector::new(((-1., -1.), (3., 1.))).style(LineStyle::new().colour("burlywood"));
    let v2 = Vector::new(((-4., -1.5), (-7., 7.))).style(LineStyle::new().colour("red"));
    let v3 = Vector::new(((1., -1.), (7., -4.))).style(LineStyle::new().colour("black"));
    let v4 = Vector::new(((-1., 1.), (-3., -3.))).style(LineStyle::new().colour("black"));

    let mut v1 = ContinuousView::new()
        .add(v1)
        .add(v2)
        .add(v3)
        .add(v4)
        .x_range(-10., 10.)
        .y_range(-10., 10.)
        .x_label("x")
        .y_label("y")
        .style(
            ContinuousViewStyle::new()
                .axis(
                    ContinuousViewAxisIntersectionStyle::OriginCenter,
                    ContinuousViewAxisLineStyle::Arrow,
                )
                .tick(ContinuousViewTickStyle::OnLine, 7.0, 12)
                .label(ContinuousViewAxisLabelStyle::Border, 16),
        );

    v1.add_grid(Grid::new(20, 20));

    Page::single(&v1).save("vector.svg").expect("saving svg");
}
