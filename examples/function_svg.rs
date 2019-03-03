fn main() {
    let f1 = plotlib::function::Function::new(|x| x * 5., 0., 10.)
        .style(plotlib::style::LineStyle::new().colour("burlywood"));
    let f2 = plotlib::function::Function::new(|x| x.powi(2), 0., 10.).style(
        plotlib::style::LineStyle::new()
            .colour("darkolivegreen")
            .width(2.),
    );
    let f3 = plotlib::function::Function::new(|x| x.sqrt() * 20., 0., 10.)
        .style(plotlib::style::LineStyle::new().colour("brown").width(1.));
    let v = plotlib::view::ContinuousView::new()
        .add(&f1)
        .add(&f2)
        .add(&f3);
    plotlib::page::Page::single(&v)
        .save("function.svg")
        .expect("saving svg");
}
