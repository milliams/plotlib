use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::PointStyle;
use crate::svg_render;
use crate::text_render;

/// The scatter *representation*.
/// It knows its data as well how to style itself
#[derive(Debug)]
pub struct Scatter {
    pub data: Vec<(f64, f64)>,
    style: PointStyle,
}

impl Scatter {
    pub fn from_slice(v: &[(f64, f64)]) -> Self {
        let mut data: Vec<(f64, f64)> = vec![];
        for &(x, y) in v {
            data.push((x, y));
        }

        Scatter {
            data,
            style: PointStyle::new(),
        }
    }

    pub fn style(mut self, style: &PointStyle) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &PointStyle {
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

impl ContinuousRepresentation for Scatter {
    fn range(&self, dim: u32) -> (f64, f64) {
        match dim {
            0 => self.x_range(),
            1 => self.y_range(),
            _ => panic!("Axis out of range"),
        }
    }

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg_render::draw_face_points(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }
    fn legend_svg(&self) -> Option<svg::node::element::Group> {
        // TODO implement
        None
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        text_render::render_face_points(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }
}
