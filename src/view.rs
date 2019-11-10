/*!
*Views* are plotlib's way of combining multiple representations into a single plot.
It is analogous to a *subplot* in other plotting libraries.

In essence, a view is a collection of representations along with some metadata describing the
extent to plot and information about the axes. It knows how to render itself.
*/

use std;
use std::f64;

use failure::format_err;
use svg::Node;

use crate::axis;
use crate::errors::Result;
use crate::grid::{Grid, GridType};
use crate::repr::{CategoricalRepresentation, ContinuousRepresentation};
use crate::svg_render;
use crate::text_render;

pub trait View {
    fn to_svg(&self, face_width: f64, face_height: f64) -> Result<svg::node::element::Group>;
    fn to_text(&self, face_width: u32, face_height: u32) -> Result<String>;
    fn add_grid(&mut self, grid: Grid);
    fn grid(&self) -> &Option<Grid>;
}

/// Standard 1-dimensional view with a continuous x-axis
#[derive(Default)]
pub struct ContinuousView {
    representations: Vec<Box<dyn ContinuousRepresentation>>,
    x_range: Option<axis::Range>,
    y_range: Option<axis::Range>,
    x_max_ticks: usize,
    y_max_ticks: usize,
    x_label: Option<String>,
    y_label: Option<String>,
    grid: Option<Grid>,
}

impl ContinuousView {
    /// Create an empty view
    pub fn new() -> ContinuousView {
        ContinuousView {
            representations: vec![],
            x_range: None,
            y_range: None,
            x_max_ticks: 6,
            y_max_ticks: 6,
            x_label: None,
            y_label: None,
            grid: None,
        }
    }
    /// Set the maximum number of ticks along the x axis.
    pub fn x_max_ticks(mut self, val: usize) -> Self {
        self.x_max_ticks = val;
        self
    }
    /// Set the maximum number of ticks along the y axis.
    pub fn y_max_ticks(mut self, val: usize) -> Self {
        self.y_max_ticks = val;
        self
    }

    /// Add a representation to the view
    pub fn add<R: ContinuousRepresentation + 'static>(mut self, repr: R) -> Self {
        self.representations.push(Box::new(repr));
        self
    }

    /// Set the x range for the view
    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some(axis::Range::new(min, max));
        self
    }

    /// Set the y range for the view
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some(axis::Range::new(min, max));
        self
    }

    /// Set the label for the x-axis
    pub fn x_label<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.x_label = Some(value.into());
        self
    }

    /// Set the label for the y-axis
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

    fn create_axes(&self) -> Result<(axis::ContinuousAxis, axis::ContinuousAxis)> {
        let default_x_range = self.default_x_range();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_range);
        if !x_range.is_valid() {
            return Err(format_err!(
                "Invalid x_range: {} >= {}. Please specify the x_range manually.",
                x_range.lower,
                x_range.upper
            ));
        }

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);
        if !y_range.is_valid() {
            return Err(format_err!(
                "Invalid y_range: {} >= {}. Please specify the y_range manually.",
                y_range.lower,
                y_range.upper
            ));
        }

        let x_label: String = self.x_label.clone().unwrap_or_else(|| "".to_string());
        let y_label: String = self.y_label.clone().unwrap_or_else(|| "".to_string());

        let x_axis = axis::ContinuousAxis::new(x_range.lower, x_range.upper, self.x_max_ticks)
            .label(x_label);
        let y_axis = axis::ContinuousAxis::new(y_range.lower, y_range.upper, self.y_max_ticks)
            .label(y_label);

        Ok((x_axis, y_axis))
    }
}

impl View for ContinuousView {
    /**
    Create an SVG rendering of the view
    */
    fn to_svg(&self, face_width: f64, face_height: f64) -> Result<svg::node::element::Group> {
        let mut view_group = svg::node::element::Group::new();

        let (x_axis, y_axis) = self.create_axes()?;

        let (legend_x, mut legend_y) = (face_width - 100., -face_height);
        if let Some(grid) = &self.grid {
            view_group.append(svg_render::draw_grid(
                GridType::Both(grid),
                face_width,
                face_height,
            ));
        }

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(&x_axis, &y_axis, face_width, face_height);
            view_group.append(repr_group);

            if let Some(legend_group) = repr.legend_svg() {
                view_group.append(legend_group.set(
                    "transform",
                    format!("translate({}, {})", legend_x, legend_y),
                ));
                legend_y += 18.;
            }
        }

        // Add in the axes
        view_group.append(svg_render::draw_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));

        Ok(view_group)
    }

    /**
    Create a text rendering of the view
    */
    fn to_text(&self, face_width: u32, face_height: u32) -> Result<String> {
        let (x_axis, y_axis) = self.create_axes()?;

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
            let face_string = repr.to_text(&x_axis, &y_axis, face_width, face_height);
            view_string =
                text_render::overlay(&view_string, &face_string, left_gutter_width as i32 + 1, 0);
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

        Ok(view_string)
    }

    fn add_grid(&mut self, grid: Grid) {
        self.grid = Some(grid)
    }

    fn grid(&self) -> &Option<Grid> {
        &self.grid
    }
}

/// A view with categorical entries along the x-axis and continuous values along the y-axis
#[derive(Default)]
pub struct CategoricalView {
    representations: Vec<Box<dyn CategoricalRepresentation>>,
    x_range: Option<Vec<String>>,
    y_range: Option<axis::Range>,
    x_label: Option<String>,
    y_label: Option<String>,
    grid: Option<Grid>,
}

impl CategoricalView {
    /**
    Create an empty view
    */
    pub fn new() -> CategoricalView {
        CategoricalView {
            representations: vec![],
            x_range: None,
            y_range: None,
            x_label: None,
            y_label: None,
            grid: None,
        }
    }

    /**
    Add a representation to the view
    */
    pub fn add<R: CategoricalRepresentation + 'static>(mut self, repr: R) -> Self {
        self.representations.push(Box::new(repr));
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
        let buffer = (y_max - y_min) / 10.;
        let y_min = if y_min == 0.0 { y_min } else { y_min - buffer };
        let y_max = y_max + buffer;
        axis::Range::new(y_min, y_max)
    }

    fn create_axes(&self) -> Result<(axis::CategoricalAxis, axis::ContinuousAxis)> {
        let default_x_ticks = self.default_x_ticks();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_ticks);

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);

        if !y_range.is_valid() {
            return Err(format_err!("invalid y_range: {:?}", y_range));
        }

        let default_x_label = "".to_string();
        let x_label: String = self.x_label.clone().unwrap_or(default_x_label);

        let default_y_label = "".to_string();
        let y_label: String = self.y_label.clone().unwrap_or(default_y_label);

        let x_axis = axis::CategoricalAxis::new(x_range).label(x_label);
        let y_axis = axis::ContinuousAxis::new(y_range.lower, y_range.upper, 6).label(y_label);

        Ok((x_axis, y_axis))
    }
}

impl View for CategoricalView {
    fn to_svg(&self, face_width: f64, face_height: f64) -> Result<svg::node::element::Group> {
        let mut view_group = svg::node::element::Group::new();

        let (x_axis, y_axis) = self.create_axes()?;

        if let Some(grid) = &self.grid {
            view_group.append(svg_render::draw_grid(
                GridType::HorizontalOnly(grid),
                face_width,
                face_height,
            ));
        }

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(&x_axis, &y_axis, face_width, face_height);
            view_group.append(repr_group);
        }

        // Add in the axes
        view_group.append(svg_render::draw_categorical_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));

        Ok(view_group)
    }

    fn to_text(&self, _face_width: u32, _face_height: u32) -> Result<String> {
        Ok("".into())
    }

    fn add_grid(&mut self, grid: Grid) {
        self.grid = Some(grid);
    }

    fn grid(&self) -> &Option<Grid> {
        &self.grid
    }
}

/*pub struct AnyView<'a> {
    representations: Vec<&'a Representation>,
    axes: Vec<>,
    x_range: Option<axis::Range>,
    y_range: Option<axis::Range>,
    x_label: Option<String>,
    y_label: Option<String>,
}*/
