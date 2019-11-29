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

/// Defines were the two axis should intersect
#[derive(Clone, Copy, Debug)]
pub enum ContinuousViewAxisIntersectionStyle {
    /// Intersect in the (0,0) point
    OriginCenter,
    /// Interset in the (min,min) point
    CornerCenter,
}

/// Defines the style of the axis line
#[derive(Clone, Copy, Debug)]
pub enum ContinuousViewAxisLineStyle {
    /// Draws a line with an arrow tip on the maximum side (svg only)
    Arrow,
    /// Draws a simple line
    Line,
}

/// Defines were the axis label should be placed
#[derive(Clone, Copy, Debug)]
pub enum ContinuousViewAxisLabelStyle {
    /// Places the label next to the axis on the maximum side
    NextToAxis,
    /// Places the label the label  on the left and bottom parts of the view
    Border,
}

/// Defines how the tick intersects the axis
#[derive(Clone, Copy, Debug)]
pub enum ContinuousViewTickStyle {
    /// The tick intersects the axis from below
    UnderLine,
    /// The tick intersects the axis on its medium point
    OnLine,
    /// The tick intersects the axis from the top
    OverLine,
}

/// Defines the style of a Continuous view
pub struct ContinuousViewStyle {
    axis_intersection: ContinuousViewAxisIntersectionStyle,
    axis_line: ContinuousViewAxisLineStyle,
    axis_label: ContinuousViewAxisLabelStyle,
    label_size: u32,
    tick_position: ContinuousViewTickStyle,
    tick_size: f32,
    tick_font_size: u32,
}

impl ContinuousViewStyle {
    /// Creates a new style
    ///
    /// The axis intersection will be of type [CornerCenter](enum.ContinuousViewAxisIntersectionStyle.html#variant.CornerCenter);
    /// The axis line will be of type [Line](enum.ContinuousViewAxisLineStyle.html#variant.Line);
    /// The axis label will be of type [Border](enum.ContinuousViewAxisLabelStyle.html#variant.Border) and have a font size of 12
    /// The axis tick will be of type [UnderLine](enum.ContinuousViewTickStyle.html#variant.UnderLine) have a size of 10 and a label with font size of 12
    pub fn new() -> Self {
        ContinuousViewStyle::default()
    }

    /// Sets the axis intersection and line style respectively
    pub fn axis(
        mut self,
        intersection: ContinuousViewAxisIntersectionStyle,
        line: ContinuousViewAxisLineStyle,
    ) -> Self {
        self.axis_intersection = intersection;
        self.axis_line = line;
        self
    }

    /// Sets the axis tick intersection, size and label size respectively
    pub fn tick(mut self, position: ContinuousViewTickStyle, size: f32, font_size: u32) -> Self {
        self.tick_position = position;
        self.tick_size = size;
        self.tick_font_size = font_size;
        self
    }

    /// Sets the plot label position and font size respectively
    pub fn label(mut self, position: ContinuousViewAxisLabelStyle, size: u32) -> Self {
        self.axis_label = position;
        self.label_size = size;
        self
    }

    /// Gets the axis intersection and line style respectively
    pub fn get_axis(
        &self,
    ) -> (
        ContinuousViewAxisIntersectionStyle,
        ContinuousViewAxisLineStyle,
    ) {
        (self.axis_intersection, self.axis_line)
    }

    /// Gets the axis tick intersection, size and label size respectively
    pub fn get_tick(&self) -> (ContinuousViewTickStyle, f32, u32) {
        (self.tick_position, self.tick_size, self.tick_font_size)
    }

    /// Gets the plot label position and font size respectively
    pub fn get_label(&self) -> (ContinuousViewAxisLabelStyle, u32) {
        (self.axis_label, self.label_size)
    }
}

// Define defaults for the styling options
impl Default for ContinuousViewAxisIntersectionStyle {
    fn default() -> Self {
        ContinuousViewAxisIntersectionStyle::CornerCenter
    }
}

impl Default for ContinuousViewAxisLineStyle {
    fn default() -> Self {
        ContinuousViewAxisLineStyle::Line
    }
}

impl Default for ContinuousViewAxisLabelStyle {
    fn default() -> Self {
        ContinuousViewAxisLabelStyle::Border
    }
}

impl Default for ContinuousViewTickStyle {
    fn default() -> Self {
        ContinuousViewTickStyle::UnderLine
    }
}

impl Default for ContinuousViewStyle {
    fn default() -> Self {
        ContinuousViewStyle {
            axis_intersection: Default::default(),
            axis_line: Default::default(),
            axis_label: Default::default(),
            tick_position: Default::default(),
            label_size: 12,
            tick_size: 10.0,
            tick_font_size: 12,
        }
    }
}

/// Standard 1-dimensional view with a continuous x-axis
pub struct ContinuousView {
    representations: Vec<Box<dyn ContinuousRepresentation>>,
    x_range: Option<axis::Range>,
    y_range: Option<axis::Range>,
    x_max_ticks: usize,
    y_max_ticks: usize,
    x_label: Option<String>,
    y_label: Option<String>,
    grid: Option<Grid>,
    style: ContinuousViewStyle,
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
            style: ContinuousViewStyle::default(),
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

    // Set the style for the view
    pub fn style(mut self, style: ContinuousViewStyle) -> Self {
        self.style = style;
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

        // Get the label styling options
        let label = self.style.get_label();

        // if the label is set to be placed on the border we store the font size to calculate the view offset
        let label_size = match label.0 {
            ContinuousViewAxisLabelStyle::Border => label.1 as f64,
            ContinuousViewAxisLabelStyle::NextToAxis => 0.,
        };

        // Calculate the x were the y axis will be placed from the left to the right, the y were the x axis will be placed from bottom to top
        // the start of the x axis, the start of the y axis
        // and the origin behavior
        let (x, y, min_x, min_y, x_zero, y_zero) = match self.style.get_axis().0 {
            ContinuousViewAxisIntersectionStyle::OriginCenter => {
                // Get the Y where the axis origin intersects
                let amplitude = y_axis.max() - y_axis.min();
                let change = (face_height - label_size) / amplitude;
                let y = y_axis.min().abs() * change;

                // Get the X where the axis origin intersects
                let amplitude = x_axis.max() - x_axis.min();
                let change = (face_width - label_size) / amplitude;
                let x = x_axis.min().abs() * change;

                (
                    // Add the label size to prevent collisions between the axis and the labels
                    x + label_size,
                    face_height - (y + label_size),
                    label_size,
                    // in svg the coords are from the top left corner so we subtract our value from the height and get our y
                    face_height - label_size,
                    // In origin center we don't want to show the ticks nor the x axis zero
                    false,
                    false,
                )
            }
            ContinuousViewAxisIntersectionStyle::CornerCenter => {
                let tick = self.style.get_tick();

                // calculate the space that the ticks and labels occupy from the borders (left and bottom)
                // OnLine => half the tick size + the tick label size and the label size
                // UnderLine => the tick size + the tick label size and the label size
                // OverLine => label size
                let (tick_margin_y, tick_margin_x) = match tick.0 {
                    ContinuousViewTickStyle::OnLine => (
                        tick.2 as f64 + tick.1 as f64 / 2.0 + label_size,
                        // Get the largest number, count the digits and multiply by the tick label size
                        tick.1 as f64 / 2.0
                            + y_axis
                                .ticks()
                                .iter()
                                .cloned()
                                .fold(0. / 0., f64::max)
                                .to_string()
                                .chars()
                                .count() as f64
                                * tick.2 as f64
                            + label_size / 2.,
                    ),
                    ContinuousViewTickStyle::OverLine => (label_size, label_size),
                    ContinuousViewTickStyle::UnderLine => (
                        tick.2 as f64 + tick.1 as f64 + label_size,
                        // Get the largest number, count the digits and multiply by the tick label size
                        tick.1 as f64
                            + y_axis
                                .ticks()
                                .iter()
                                .cloned()
                                .fold(0. / 0., f64::max)
                                .to_string()
                                .chars()
                                .count() as f64
                                * tick.2 as f64
                            + label_size / 2.,
                    ),
                };

                (
                    tick_margin_x,
                    // Once again we need to subtract our Y from our height to get it into the correct coordinates
                    face_height - tick_margin_y,
                    tick_margin_x,
                    // Once again we need to subtract our Y from our height to get it into the correct coordinates
                    face_height - tick_margin_y,
                    true,
                    true,
                )
            }
        };

        // Add in the axes
        view_group.append(svg_render::draw_x_axis(
            &x_axis,
            min_x,
            y,
            face_width,
            face_height,
            x_zero,
            self.style.get_axis().1,
            self.style.get_label(),
            self.style.get_tick(),
        ));

        view_group.append(svg_render::draw_y_axis(
            &y_axis,
            x,
            min_y,
            face_height,
            y_zero,
            self.style.get_axis().1,
            self.style.get_label(),
            self.style.get_tick(),
        ));

        if let Some(grid) = &self.grid {
            view_group.append(svg_render::draw_grid(
                GridType::Both(grid),
                min_x,
                0.,
                face_width - min_x,
                min_y,
            ));
        }

        let (legend_x, mut legend_y) = (face_width, 10.);

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(&x_axis, &y_axis, min_x, min_y, face_width - min_x, min_y);
            view_group.append(repr_group);

            if let Some(legend_group) = repr.legend_svg() {
                view_group.append(legend_group.set(
                    "transform",
                    format!("translate({}, {})", legend_x, legend_y),
                ));
                legend_y += 18.;
            }
        }

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

        let (y, x) = (
            12. + 10. + 12.,
            10. + y_axis
                .ticks()
                .iter()
                .cloned()
                .fold(0. / 0., f64::max)
                .to_string()
                .chars()
                .count() as f64
                * 12.,
        );

        // Add in the axes
        view_group.append(svg_render::draw_categorical_x_axis(
            &x_axis,
            x,
            face_height - y,
            face_width,
            face_height,
        ));
        view_group.append(svg_render::draw_y_axis(
            &y_axis,
            x,
            face_height - y,
            face_height,
            true,
            ContinuousViewAxisLineStyle::default(),
            (ContinuousViewAxisLabelStyle::default(), 12),
            (ContinuousViewTickStyle::default(), 10., 12),
        ));

        if let Some(grid) = &self.grid {
            view_group.append(svg_render::draw_grid(
                GridType::HorizontalOnly(grid),
                x,
                y,
                face_width - x,
                face_height - y,
            ));
        }

        // Then, based on those ranges, draw each repr as an SVG
        for repr in &self.representations {
            let repr_group = repr.to_svg(
                &x_axis,
                &y_axis,
                face_height - y,
                x,
                face_width - x,
                face_height - y,
            );
            view_group.append(repr_group);
        }

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
