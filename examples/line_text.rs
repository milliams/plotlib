use plotlib::repr::Line;
use plotlib::page::Page;
use plotlib::view::ContinuousView;

fn main() {
    let l1 = Line::new(&[(-1.4, 2.5), (2.3, 0.2), (5.1, 1.1), (7.2, -0.3)]);
    let v = ContinuousView::new().add(&l1);
    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}
