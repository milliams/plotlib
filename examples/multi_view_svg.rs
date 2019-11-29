use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::{BarChart, Function, Histogram, HistogramBins, Scatter};
use plotlib::style::{BoxStyle, LineStyle, PointMarker, PointStyle};
use plotlib::view::{
    CategoricalView, ContinuousView, ContinuousViewAxisIntersectionStyle,
    ContinuousViewAxisLabelStyle, ContinuousViewAxisLineStyle, ContinuousViewStyle,
    ContinuousViewTickStyle, View,
};

fn main() {
    let f1 = Function::new(|x| x, -10., 10.).style(LineStyle::new().colour("burlywood"));
    let f2 = Function::new(|x| x.powi(2), -3.16, 3.16)
        .style(LineStyle::new().colour("darkolivegreen").width(2.));
    let f3 = Function::new(|x| x.sqrt(), 0., 10.).style(LineStyle::new().colour("brown").width(1.));

    let mut v1 = ContinuousView::new()
        .add(f1)
        .add(f2)
        .add(f3)
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

    let b1 = BarChart::new(5.3).label("1");
    let b2 = BarChart::new(2.6)
        .label("2")
        .style(BoxStyle::new().fill("darkolivegreen"));

    let v2 = CategoricalView::new().add(b1).add(b2).x_label("Experiment");

    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
        .style(BoxStyle::new().fill("burlywood"));

    let v3 = ContinuousView::new().add(h);
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = Scatter::from_slice(&data).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let s2 = Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(PointStyle::new().colour("darkseagreen"));

    let v4 = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v1)
        .dimensions(800, 800)
        .add_plot(&v2, 1, 0)
        .add_plot(&v3, 0, 1)
        .add_plot(&v4, 1, 1)
        .save("multi_view.svg")
        .expect("saving svg");
}
