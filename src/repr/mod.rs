/*!
*Representations* are the interface between the data coming from the user and the rendered output.

Each type that implements `Representation` or `CategoricalRepr` knows how to read in data
and convert that into a concrete element to be incorporated into a larger plot.

For example the `scatter::Scatter` representation can be created from a list of coordinates.
When `to_svg()` is called on it, it will create the SVG elements showing the points from within
the range that was requested by the caller.

These points may then be layered with other SVG elements from other representations into a
`view::View`.
*/

use crate::axis;
use crate::svg;

mod line;
mod function;
mod scatter;
mod boxplot;
mod histogram;
mod barchart;
mod style;

pub use line::*;
pub use function::*;
pub use scatter::*;
pub use boxplot::*;
pub use histogram::*;
pub use barchart::*;
pub use style::*;

/**
A representation of data that is continuous in two dimensions.
*/
pub trait ContinuousRepr {
    /// The maximum range in each dimension. Used for auto-scaling axes.
    fn range(&self, dim: u32) -> (f64, f64);

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group;

    /// Returns None if no legend has been specified for this representation
    fn legend_svg(&self) -> Option<svg::node::element::Group>;

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String;
}

/**
A representation of data that is categorical in the x-axis but continuous in the y-axis.
*/
pub trait CategoricalRepr {
    /// The maximum range in the y-axis. Used for auto-scaling the axis.
    fn range(&self) -> (f64, f64);

    /// The ticks that this representation covers. Used to collect all ticks for display.
    fn ticks(&self) -> Vec<String>;

    fn to_svg(
        &self,
        x_axis: &axis::CategoricalAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group;

    fn to_text(
        &self,
        x_axis: &axis::CategoricalAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String;
}
