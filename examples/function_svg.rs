extern crate plotlib;

use plotlib::{repr, view};

fn main() {
    let f1 = repr::Function::new(|x| x * 5., 0., 10.)
        .style(repr::LineStyle::new().colour("burlywood"));
    let f2 = repr::Function::new(|x| x.powi(2), 0., 10.).style(
        repr::LineStyle::new()
            .colour("darkolivegreen")
            .width(2.),
    );
    let f3 = repr::Function::new(|x| x.sqrt() * 20., 0., 10.)
        .style(repr::LineStyle::new().colour("brown").width(1.));
    let v = view::ContinuousView::new()
        .add(Box::new(f1))
        .add(Box::new(f2))
        .add(Box::new(f3));
    plotlib::page::Page::single(&v)
        .save("function.svg")
        .expect("saving svg");
}
