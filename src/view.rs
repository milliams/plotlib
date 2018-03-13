/*!
*Views* are plotlib's way of combining multiple representations into a single plot.
It is analogous to a *subplot* in other plotting libraries.

In essence, a view is a collection of representations along with some metadata describing the
extent to plot and information about the axes. It knows how to render itself.

Currently a view refers only to planar plots (i.e. not map projections and polar plots).
*/

use std;
use std::f64;

use svg;
use svg::Node;

use representation::{DiscreteRepresentation, ContinuousRepresentation, PlanarRepresentation};
use axis;
use svg_render;
use text_render;

pub trait View {
    fn to_svg(&self, face_width: f64, face_height: f64) -> svg::node::element::Group;
    fn to_text(&self, face_width: u32, face_height: u32) -> String;
}

/// Standard 1-dimensional view with a continuous x-axis
#[derive(Default)]
pub struct ContinuousView<'a> {
    representations: Vec<&'a PlanarRepresentation>,
    x_range: Option<axis::Range>,
    y_range: Option<axis::Range>,
    x_label: Option<String>,
    y_label: Option<String>,
}

impl<'a> ContinuousView<'a> {
    /**
    Create an empty view
    */
    pub fn new() -> ContinuousView<'a> {
        ContinuousView {
            representations: vec![],
            x_range: None,
            y_range: None,
            x_label: None,
            y_label: None,
        }
    }

    /**
    Add a representation to the view
    */
    pub fn add(mut self, repr: &'a PlanarRepresentation) -> Self {
        self.representations.push(repr);
        self
    }

    /**
    Set the x range for the view
    */
    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some(axis::Range::new(min, max));
        self
    }

    /**
    Set the y range for the view
    */
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some(axis::Range::new(min, max));
        self
    }

    /**
    Set the label for the x-axis
    */
    pub fn x_label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.x_label = Some(value.into());
        self
    }

    /**
    Set the label for the y-axis
    */
    pub fn y_label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.y_label = Some(value.into());
        self
    }

    fn default_x_range(&self) -> axis::Range {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        for repr in &self.representations {
            let (this_x_min, this_x_max) = repr.range(0);
            x_min = x_min.min(this_x_min);
            x_max = x_max.max(this_x_max);
        }
        axis::Range::new(x_min, x_max)
    }

    fn default_y_range(&self) -> axis::Range {
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for repr in &self.representations {
            let (this_y_min, this_y_max) = repr.range(1);
            y_min = y_min.min(this_y_min);
            y_max = y_max.max(this_y_max);
        }
        axis::Range::new(y_min, y_max)
    }

    fn create_axes(&self) -> (axis::ContinuousAxis, axis::ContinuousAxis) {
        let default_x_range = self.default_x_range();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_range);

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);

        let default_x_label = "".to_string();
        let x_label: String = self.x_label.clone().unwrap_or(default_x_label);

        let default_y_label = "".to_string();
        let y_label: String = self.y_label.clone().unwrap_or(default_y_label);

        let x_axis = axis::ContinuousAxis::new(x_range.lower, x_range.upper).label(x_label);
        let y_axis = axis::ContinuousAxis::new(y_range.lower, y_range.upper).label(y_label);

        (x_axis, y_axis)
    }
}

use nalgebra::{Affine2};

use representation::AxisTransform;

impl<'a> View for ContinuousView<'a> {
    /**
    Create an SVG rendering of the view
    */
    fn to_svg(&self, face_width: f64, face_height: f64) -> svg::node::element::Group {
        let mut view_group = svg::node::element::Group::new();

        let (x_axis, y_axis) = self.create_axes();

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(&[AxisTransform::Continuous(Affine2::identity()), AxisTransform::Continuous(Affine2::identity())]);
            view_group.append(repr_group);
        }

        // Add in the axes
        view_group.append(svg_render::draw_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));
        view_group
    }

    /**
    Create a text rendering of the view
    */
    fn to_text(&self, face_width: u32, face_height: u32) -> String {
        let (x_axis, y_axis) = self.create_axes();

        let (y_axis_string, longest_y_label_width) =
            text_render::render_y_axis_strings(&y_axis, face_height);

        let (x_axis_string, start_offset) = text_render::render_x_axis_strings(&x_axis, face_width);

        let left_gutter_width = std::cmp::max(
            longest_y_label_width as i32 + 3,
            start_offset.wrapping_neg(),
        ) as u32;

        let view_width = face_width + 1 + left_gutter_width + 1;
        let view_height = face_height + 4;

        let blank: Vec<String> = (0..view_height)
            .map(|_| (0..view_width).map(|_| ' ').collect())
            .collect();
        let mut view_string = blank.join("\n");

        for repr in &self.representations {
            // TODO
            /*let face_string = repr.to_text(&x_axis, &y_axis, face_width, face_height);
            view_string =
                text_render::overlay(&view_string, &face_string, left_gutter_width as i32 + 1, 0);*/
        }

        let view_string = text_render::overlay(
            &view_string,
            &y_axis_string,
            left_gutter_width as i32 - 2 - longest_y_label_width,
            0,
        );
        let view_string = text_render::overlay(
            &view_string,
            &x_axis_string,
            left_gutter_width as i32,
            face_height as i32,
        );

        view_string
    }
}

/// A view with discrete entries along the x-axis and continuous values along the y-axis
#[derive(Default)]
pub struct DiscreteView<'a> {
    representations: Vec<&'a DiscreteRepresentation>,
    x_range: Option<Vec<String>>,
    y_range: Option<axis::Range>,
    x_label: Option<String>,
    y_label: Option<String>,
}

impl<'a> DiscreteView<'a> {
    /**
    Create an empty view
    */
    pub fn new() -> DiscreteView<'a> {
        DiscreteView {
            representations: vec![],
            x_range: None,
            y_range: None,
            x_label: None,
            y_label: None,
        }
    }

    /**
    Add a representation to the view
    */
    pub fn add(mut self, repr: &'a DiscreteRepresentation) -> Self {
        self.representations.push(repr);
        self
    }

    /**
    Set the x range for the view
    */
    pub fn x_ticks(mut self, ticks: &[String]) -> Self {
        self.x_range = Some(ticks.into());
        self
    }

    /**
    Set the y range for the view
    */
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some(axis::Range::new(min, max));
        self
    }


    /**
    Set the label for the x-axis
    */
    pub fn x_label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.x_label = Some(value.into());
        self
    }


    /**
    Set the label for the y-axis
    */
    pub fn y_label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.y_label = Some(value.into());
        self
    }

    fn default_x_ticks(&self) -> Vec<String> {
        let mut v = vec![];
        for repr in &self.representations {
            for l in repr.ticks() {
                if !v.contains(&l) {
                    v.push(l.clone());
                }
            }
        }
        v
    }

    fn default_y_range(&self) -> axis::Range {
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for repr in &self.representations {
            let (this_y_min, this_y_max) = repr.range();
            y_min = y_min.min(this_y_min);
            y_max = y_max.max(this_y_max);
        }
        let range = y_max - y_min;
        axis::Range::new(y_min - range / 10., y_max + range / 10.)
    }

    fn create_axes(&self) -> (axis::DiscreteAxis, axis::ContinuousAxis) {
        let default_x_ticks = self.default_x_ticks();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_ticks);

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);

        let default_x_label = "".to_string();
        let x_label: String = self.x_label.clone().unwrap_or(default_x_label);

        let default_y_label = "".to_string();
        let y_label: String = self.y_label.clone().unwrap_or(default_y_label);

        let x_axis = axis::DiscreteAxis::new(x_range).label(x_label);
        let y_axis = axis::ContinuousAxis::new(y_range.lower, y_range.upper).label(y_label);

        (x_axis, y_axis)
    }
}

impl<'a> View for DiscreteView<'a> {
    fn to_svg(&self, face_width: f64, face_height: f64) -> svg::node::element::Group {
        let mut view_group = svg::node::element::Group::new();

        let (x_axis, y_axis) = self.create_axes();

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(&x_axis, &y_axis, face_width, face_height);
            view_group.append(repr_group);
        }

        // Add in the axes
        view_group.append(svg_render::draw_discrete_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));
        view_group
    }

    fn to_text(&self, face_width: u32, face_height: u32) -> String {
        "".into()
    }
}
