use plotlib::line::{Line, Style};
use plotlib::page::Page;
// XXX only supports rust 1.31, however we cannot import the BarChart trait and BarChart struct at
// the same time
use plotlib::style::Line as _;
use plotlib::view::ContinuousView;

fn main() {
    let l1 = Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(Style::new().colour("burlywood"));
    let v = ContinuousView::new().add(&l1);
    Page::single(&v).save("line.svg").expect("saving svg");
}
