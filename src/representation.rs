/*!
*Representations* are the interface between the data coming from the user and the rendered output.

Each type that implements `Representation` or `DiscreteRepresentation` knows how to read in data
and convert that into a concrete element to be incorporated into a larger plot.

For example the `scatter::Scatter` representation can be created from a list of coordinates.
When `to_svg()` is called on it, it will create the SVG elements showing the points from within
the range that was requested by the caller.

These points may then be layered with other SVG elements from other representations into a
`view::View`.

Ideas:

We want to move from specific discrete/continuous structs to a generic interface.
In general a rendering function (like to_svg, to_text) should pass:
- the data to be plotted,
- a transformation for each axis and
- a limit for each axis

Probably transformations will be a transformation matrix for continuous axes
and some sort of lookup/mapping to matrices for discrete axes.

Some representations are explicitly 2D (box plot), some are explicitly 3D (surface plot).

2D representations could in principle be embedded into a 3D plot if the position in the 3rd dimension is specified.
This should probably de done explicitly by creating a special repr for the purpose.

The other time we want a 2D drawing in a 3D plot is for things like a surface plot having a contour projection onto the sides of the 3D box.
This might be best handled as a feature of the 3D surface plot rather than a separate 2D contour plot.

This comes back to a strong distinction between 2D and 3D plots.
*/

use nalgebra::{Affine2};

use svg;
use axis;

/**

*/
pub trait PlanarRepresentation {
    fn range(&self, dim: u32) -> (f64, f64);

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        transform: Affine2<f64>,
    ) -> svg::node::element::Group;

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        transform: Affine2<f64>,
        face_width: u32,
        face_height: u32,
    ) -> String;
}

/**
A representation of data that is continuous in two dimensions.
*/
pub trait ContinuousRepresentation {
    /// The maximum range in each dimension. Used for auto-scaling axes.
    fn range(&self, dim: u32) -> (f64, f64);

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group;

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String;
}

/**
A representation of data that is discrete in the x-axis but continuous in the y-axis.
*/
pub trait DiscreteRepresentation {
    /// The maximum range in the y-axis. Used for auto-scaling the axis.
    fn range(&self) -> (f64, f64);

    /// The ticks that this representation covers. Used to collect all ticks for display.
    fn ticks(&self) -> Vec<String>;

    fn to_svg(
        &self,
        x_axis: &axis::DiscreteAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group;

    fn to_text(
        &self,
        x_axis: &axis::DiscreteAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String;
}
