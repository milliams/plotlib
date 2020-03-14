use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::view::ContinuousView;

fn main() {
    let l1 = Plot::new(vec![(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)]).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new().add(l1);
    Page::single(&v).save("line.svg").expect("saving svg");
}
