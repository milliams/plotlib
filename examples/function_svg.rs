use plotlib::function::{Function, Style};
use plotlib::page::Page;
use plotlib::style::Line;
use plotlib::view::ContinuousView;

fn main() {
    let f1 = Function::new(|x| x * 5., 0., 10.).style(Style::new().colour("burlywood"));
    let f2 = Function::new(|x| x.powi(2), 0., 10.)
        .style(Style::new().colour("darkolivegreen").width(2.));
    let f3 =
        Function::new(|x| x.sqrt() * 20., 0., 10.).style(Style::new().colour("brown").width(1.));

    let v = ContinuousView::new().add(&f1).add(&f2).add(&f3);

    Page::single(&v).save("function.svg").expect("saving svg");
}
