/*!

Plot arbitrary functions

# Examples

```
# use plotlib::repr::Function;
# use plotlib::view::ContinuousView;
// y=x^2 between 0 and 10
let f = Function::new(|x| x*x, 0., 10.);
let v = ContinuousView::new().add(f);
```
*/

use std::f64;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::LineStyle;
use crate::svg_render;

pub struct Function {
    pub data: Vec<(f64, f64)>,
    style: LineStyle,
}

impl Function {
    pub fn new<F>(f: F, lower: f64, upper: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let sampling = (upper - lower) / 200.;
        let samples = (0..)
            .map(|x| lower + (f64::from(x) * sampling))
            .take_while(|&x| x <= upper);
        let values = samples.map(|s| (s, f(s))).collect();
        Function {
            data: values,
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

impl ContinuousRepresentation for Function {
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
