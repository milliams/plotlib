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
        Style { marker: Marker::Circle }
    }
}

/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Scatter {
    pub data: Vec<(f64, f64)>,
    pub style: Style,
}

impl Scatter {
    pub fn from_vec(v: &[(f64, f64)]) -> Self {
        let mut data: Vec<(f64, f64)> = vec![];
        for &(x, y) in v {
            data.push((x, y));
        }

        Scatter {
            data: data,
            style: Style::defaults(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

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
}

impl Representation for Scatter {
    fn range(&self, dim: u32) -> (f64, f64) {
        match dim {
            0 => self.x_range(),
            1 => self.y_range(),
            _ => panic!("Axis out of range")
        }
    }

    fn to_svg(&self,
              x_axis: &axis::Axis,
              y_axis: &axis::Axis,
              face_width: f64,
              face_height: f64)
              -> svg::node::element::Group {
        svg_render::draw_face_points(self, &x_axis, &y_axis, face_width, face_height)
    }

    fn to_text(&self,
               x_axis: &axis::Axis,
               y_axis: &axis::Axis,
               face_width: u32,
               face_height: u32)
               -> String {
        text_render::render_face_points(self, &x_axis, &y_axis, face_width, face_height)
    }
}
