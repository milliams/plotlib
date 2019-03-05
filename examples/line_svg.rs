use plotlib::line::Line;
use plotlib::page::Page;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;

fn main() {
    let l1 = Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(LineStyle::new().colour("burlywood"));
    let v = ContinuousView::new().add(&l1);
    Page::single(&v).save("line.svg").expect("saving svg");
}
