extern crate plotlib;

fn main() {
    let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = plotlib::histogram::Histogram::from_vec(&data, 10);
    plotlib::text_render::draw_histogram(&h);
}
