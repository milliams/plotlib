extern crate plotlib;

fn main() {
    let f = plotlib::function::Function::new(|x| x, 0., 10.)
        .style(plotlib::function::Style::new().colour("burlywood"));
    let v = plotlib::view::View::new().add(&f);
    plotlib::page::Page::single(&v).save("function.svg");
}
