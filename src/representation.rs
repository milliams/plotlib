use svg;
use axis;

pub trait Representation {
    /// The maximum range in each dimension. Used for auto-scaling axes
    fn range(&self, dim: u32) -> (f64, f64);

    fn to_svg(
        &self,
        x_axis: &axis::Axis,
        y_axis: &axis::Axis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group;

    fn to_text(
        &self,
        x_axis: &axis::Axis,
        y_axis: &axis::Axis,
        face_width: u32,
        face_height: u32,
    ) -> String;
}
