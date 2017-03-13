use svg;

use axis;

pub trait Representation {
    fn x_range(&self) -> (f64, f64); // TODO this must be generalised!
    fn y_range(&self) -> (f64, f64); // TODO this must be generalised!

    fn to_svg(&self, x_axis: &axis::Axis, y_axis: &axis::Axis, face_width: f64, face_height: f64) -> svg::node::element::Group;
}
