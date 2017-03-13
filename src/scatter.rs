use std::f64;

use svg;

use axis;
use svg_render;
use text_render;
use representation::Representation;

#[derive(Debug)]
pub enum Marker {
    Circle,
    Square,
}

#[derive(Debug)]
pub struct Style {
    pub marker: Marker,
}

impl Style {
    fn defaults() -> Self {
        Style {
            marker: Marker::Circle,
        }
    }
}

/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Scatter {
    pub data: Vec<(f64, f64)>,
    pub x_axis: axis::Axis,
    pub y_axis: axis::Axis,
    pub style: Style,
}

impl Scatter {
    pub fn from_vec(v: &[(f64, f64)]) -> Self {

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut data: Vec<(f64, f64)> = vec![];
        for &(x, y) in v {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
            data.push((x, y));
        }

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        x_min = x_min - (x_range / 20.0);
        x_max = x_max + (x_range / 20.0);
        y_min = y_min - (y_range / 20.0);
        y_max = y_max + (y_range / 20.0);

        let x_axis = axis::Axis::new(x_min, x_max);
        let y_axis = axis::Axis::new(y_min, y_max);

        Scatter {
            data: data,
            x_axis: x_axis,
            y_axis: y_axis,
            style: Style::defaults(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Representation for Scatter {
    fn x_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(x, _) in self.data.iter() {
            min = min.min(x);
            max = max.max(x);
        }
        (min, max)
    }

    fn y_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(_, y) in self.data.iter() {
            min = min.min(y);
            max = max.max(y);
        }
        (min, max)
    }

    fn to_svg(&self, x_axis: &axis::Axis, y_axis: &axis::Axis, face_width: f64, face_height: f64) -> svg::node::element::Group {
        svg_render::draw_face_points(self, &x_axis, &y_axis, face_width, face_height)
    }

    fn to_text(&self, x_axis: &axis::Axis, y_axis: &axis::Axis, face_width: u32, face_height: u32) -> String {
        text_render::render_face_points(self, &x_axis, &y_axis, face_width, face_height).join("\n")
    }
}
