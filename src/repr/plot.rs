//! Plot line charts

//! # Examples

//! ```
//! # use plotlib::repr::Plot;
//! # use plotlib::view::ContinuousView;
//! // y=x^2 between 0 and 10
//! let l = Plot::new(vec![(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)]);
//! let v = ContinuousView::new().add(l);
//! ```

use std::f64;

use svg;
use svg::node;
use svg::Node;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::*;
use crate::svg_render;
use crate::text_render;
use crate::svg_render::draw_marker;

/// Representation of any plot with points in the XY plane, visualized as points and/or with lines
/// in-between.
#[derive(Debug, Clone)]
pub struct Plot {
    pub data: Vec<(f64, f64)>,
    /// None if no lines should be displayed
    pub line_style: Option<LineStyle>,
    /// None if no points should be displayed
    pub point_style: Option<PointStyle>,
    pub legend: Option<String>,
}

impl Plot {
    pub fn new(data: Vec<(f64, f64)>) -> Self {
        Plot {
            data,
            line_style: None,
            point_style: None,
            legend: None,
        }
    }

    pub fn from_function<F: Fn(f64) -> f64>(f: F, lower: f64, upper: f64) -> Self {
        let sampling = (upper - lower) / 200.;
        let samples = (0..)
            .map(|x| lower + (f64::from(x) * sampling))
            .take_while(|&x| x <= upper);
        let values = samples.map(|s| (s, f(s))).collect();
        Plot {
            data: values,
            line_style: None,
            point_style: None,
            legend: None,
        }
    }

    pub fn line_style(mut self, other: LineStyle) -> Self {
        if let Some(ref mut self_style) = self.line_style {
            self_style.overlay(&other);
        } else {
            self.line_style = Some(other);
        }
        self
    }
    pub fn point_style(mut self, other: PointStyle) -> Self {
        if let Some(ref mut self_style) = self.point_style {
            self_style.overlay(&other);
        } else {
            self.point_style = Some(other);
        }
        self
    }
    pub fn legend(mut self, legend: String) -> Self {
        self.legend = Some(legend);
        self
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

impl ContinuousRepresentation for Plot {
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
        let mut group = node::element::Group::new();
        if let Some(ref line_style) = self.line_style {
            group.append(svg_render::draw_face_line(
                &self.data,
                x_axis,
                y_axis,
                face_width,
                face_height,
                line_style,
            ))
        }
        if let Some(ref point_style) = self.point_style {
            group.append(svg_render::draw_face_points(
                &self.data,
                x_axis,
                y_axis,
                face_width,
                face_height,
                point_style,
            ))
        }
        group
    }
    fn legend_svg(&self) -> Option<svg::node::element::Group> {
        // TODO: add points
        // TODO: can we use common functionality with svg_render?
        self.legend.as_ref().map(|legend| {
            let legend = legend.clone();

            let mut group = node::element::Group::new();
            const FONT_SIZE: f32 = 12.0;

            // Draw legend text
            let legend_text = node::element::Text::new()
                .set("x", 0)
                .set("y", 0)
                .set("text-anchor", "start")
                .set("font-size", FONT_SIZE)
                .add(node::Text::new(legend));
            group.append(legend_text);

            if let Some(ref style) = self.line_style {
                let line = node::element::Line::new()
                    .set("x1", -23)
                    .set("y1", -FONT_SIZE / 2. + 2.)
                    .set("x2", -3)
                    .set("y2", -FONT_SIZE / 2. + 2.)
                    .set("stroke-width", style.get_width())
                    .set("stroke", style.get_colour());
                group.append(line);
            }

            if let Some(ref style) = self.point_style {
                let mark = draw_marker(-13., (-FONT_SIZE / 2. + 2.) as f64, style);
                group.append(mark);
            }

            group
        })
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        let face_lines = if let Some(line_style) = &self.line_style {
            unimplemented!("Text rendering does not yet support line plots")
        } else {
            text_render::empty_face(face_width, face_height)
        };
        let face_points = if let Some(point_style) = &self.point_style {
            text_render::render_face_points(
                &self.data,
                x_axis,
                y_axis,
                face_width,
                face_height,
                &point_style,
            )
        } else {
            text_render::empty_face(face_width, face_height)
        };
        text_render::overlay(&face_lines, &face_points, 0, 0)
    }
}
