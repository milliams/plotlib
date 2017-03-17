use std::f64;

use svg;

use axis;
use svg_render;
use text_render;
use representation::Representation;

/**
The marker that should be used for the points of the scatter plot
*/
#[derive(Debug,Clone)]
pub enum Marker {
    Circle,
    Square,
    Cross,
}

/// `Style` follows the 'optional builder' pattern
/// Each field is a `Option` which start as `None`
/// Each can be set with setter methods and instances
/// of `Style` can be overlaid to set many at once.
/// Settings will be cloned in and out of it.
#[derive(Debug)]
pub struct Style {
    marker: Option<Marker>,
    colour: Option<String>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            marker: None,
            colour: None,
        }
    }

    pub fn overlay(&mut self, other: Self) {
        match other.marker {
            Some(v) => self.marker = Some(v),
            None => {}
        }
        match other.colour {
            Some(v) => self.colour = Some(v),
            None => {}
        }
    }

    pub fn marker<T>(mut self, value: T) -> Self
        where T: Into<Marker>
    {
        self.marker = Some(value.into());
        self
    }

    pub fn get_marker(&self) -> Marker {
        match self.marker.clone() {
            Some(v) => v,
            None => Marker::Circle,
        }
    }

    pub fn colour<T>(mut self, value: T) -> Self
        where T: Into<String>
    {
        self.colour = Some(value.into());
        self
    }

    pub fn get_colour(&self) -> String {
        match self.colour.clone() {
            Some(v) => v,
            None => "".into(),
        }
    }
}

/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Scatter {
    pub data: Vec<(f64, f64)>,
    style: Style,
}

impl Scatter {
    pub fn from_vec(v: &[(f64, f64)]) -> Self {
        let mut data: Vec<(f64, f64)> = vec![];
        for &(x, y) in v {
            data.push((x, y));
        }

        Scatter {
            data: data,
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    fn x_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(x, _) in &self.data {
            min = min.min(x);
            max = max.max(x);
        }
        (min, max)
    }

    fn y_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(_, y) in &self.data {
            min = min.min(y);
            max = max.max(y);
        }
        (min, max)
    }
}

impl Representation for Scatter {
    fn range(&self, dim: u32) -> (f64, f64) {
        match dim {
            0 => self.x_range(),
            1 => self.y_range(),
            _ => panic!("Axis out of range"),
        }
    }

    fn to_svg(&self,
              x_axis: &axis::Axis,
              y_axis: &axis::Axis,
              face_width: f64,
              face_height: f64)
              -> svg::node::element::Group {
        svg_render::draw_face_points(self, x_axis, y_axis, face_width, face_height, &self.style)
    }

    fn to_text(&self,
               x_axis: &axis::Axis,
               y_axis: &axis::Axis,
               face_width: u32,
               face_height: u32)
               -> String {
        text_render::render_face_points(self, x_axis, y_axis, face_width, face_height, &self.style)
    }
}
