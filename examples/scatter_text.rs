extern crate plotlib;

fn main() {
    let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    let s = plotlib::scatter::Scatter::from_vec(&data);
    let v = plotlib::view::View::new()
        .add(&s)
        .x_range(-5., 10.)
        .y_range(-2., 6.);
    println!("{}", plotlib::plot::Plot::single(&v).to_text());
}
