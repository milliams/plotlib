use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let data = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Circle));
    let s2 = Plot::new(vec![(-1.4, 2.5), (7.2, -0.3)])
        .point_style(PointStyle::new().marker(PointMarker::Square));

    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}
