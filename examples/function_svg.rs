use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;

fn main() {
    let f1 =
        Plot::from_function(|x| x * 5., 0., 10.).line_style(LineStyle::new().colour("burlywood"));
    let f2 = Plot::from_function(|x| x.powi(2), 0., 10.)
        .line_style(LineStyle::new().colour("darkolivegreen").width(2.));
    let f3 = Plot::from_function(|x| x.sqrt() * 20., 0., 10.)
        .line_style(LineStyle::new().colour("brown").width(1.));

    let v = ContinuousView::new().add(f1).add(f2).add(f3);

    Page::single(&v).save("function.svg").expect("saving svg");
}
