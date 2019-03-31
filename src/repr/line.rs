/*!

Plot line charts

# Examples

```
# use plotlib::repr::Line;
# use plotlib::view::ContinuousView;
// y=x^2 between 0 and 10
let l = Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)]);
let v = ContinuousView::new().add(&l);
```
*/

use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::LineStyle;
use crate::svg_render;
use crate::text_render;

pub struct Line {
    pub data: Vec<(f64, f64)>,
    style: LineStyle,
}

impl Line {
    pub fn new(v: &[(f64, f64)]) -> Self {
        Line {
            data: v.into(),
            style: LineStyle::new(),
        }
    }

    pub fn style(mut self, style: &LineStyle) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &LineStyle {
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

impl ContinuousRepresentation for Line {
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
        svg_render::draw_face_line(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        text_render::render_face_line(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }
}
