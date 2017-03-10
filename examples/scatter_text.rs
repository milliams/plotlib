extern crate plotlib;

fn main() {
    let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    let s = plotlib::scatter::Scatter::from_vec(&data);
    plotlib::text_render::draw_scatter(&s);
}
