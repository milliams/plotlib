/*!

Plot arbitrary vectors

# Examples

```
# use plotlib::repr::Vector;
# use plotlib::view::ContinuousView;
let f = Vector::new(&((0.,0.),(1.,1.)));
let v = ContinuousView::new().add(f);
```
*/

use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::LineStyle;
use crate::svg_render;

pub struct Vector {
    pub data: ((f64, f64), (f64, f64)),
    style: LineStyle,
}

impl Vector {
    pub fn new(data: ((f64, f64), (f64, f64))) -> Self {
        Vector {
            data,
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

        if (self.data.0).0 < (self.data.1).0 {
            min = min.min((self.data.0).0);
            max = max.max((self.data.1).0);
        } else {
            min = min.min((self.data.1).0);
            max = max.max((self.data.0).0);
        };

        (min, max)
    }

    fn y_range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        if (self.data.0).1 < (self.data.1).1 {
            min = min.min((self.data.0).1);
            max = max.max((self.data.1).1);
        } else {
            min = min.min((self.data.1).1);
            max = max.max((self.data.0).1);
        };

        (min, max)
    }
}

impl ContinuousRepresentation for Vector {
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
        x: f64,
        y: f64,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg_render::draw_face_vector(
            self.data,
            x_axis,
            y_axis,
            x,
            y,
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
        _x_axis: &axis::ContinuousAxis,
        _y_axis: &axis::ContinuousAxis,
        _face_width: u32,
        _face_height: u32,
    ) -> String {
        "".into()
    }
}
