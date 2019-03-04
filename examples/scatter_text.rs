use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::{Marker, Point};
use plotlib::view::ContinuousView;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];

    let s1 = Scatter::from_slice(&data);
    let s2 =
        Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)]).style(Style::new().marker(Marker::Square));

    let v = ContinuousView::new()
        .add(&s1)
        .add(&s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    println!("{}", Page::single(&v).to_text().unwrap());
}
