use plotlib::page::Page;
use plotlib::repr::Function;
use plotlib::style::LineStyle;
use plotlib::view::{
    ContinuousView, ContinuousViewAxisIntersectionStyle, ContinuousViewAxisLabelStyle,
    ContinuousViewAxisLineStyle, ContinuousViewStyle, ContinuousViewTickStyle,
};

fn main() {
    let f1 = Function::new(|x| x, -10., 10.).style(LineStyle::new().colour("burlywood"));
    let f2 = Function::new(|x| x.powi(2), -3.16, 3.16)
        .style(LineStyle::new().colour("darkolivegreen").width(2.));
    let f3 = Function::new(|x| x.sqrt(), 0., 10.).style(LineStyle::new().colour("brown").width(1.));

    let v = ContinuousView::new()
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

    Page::single(&v).save("function.svg").expect("saving svg");
}
