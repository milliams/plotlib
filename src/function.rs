/*!

Plot arbitrary functions

# Examples

```
# use plotlib::function::Function;
# use plotlib::view::ContinuousView;
// y=x^2 between 0 and 10
let f = Function::new(|x| x*x, 0., 10.);
let v = ContinuousView::new().add(&f);
```
*/

use std::f64;

use svg;
use nalgebra;

use axis;
use representation::PlanarRepresentation;
use svg_render;
use style;

#[derive(Debug, Default)]
pub struct Style {
    colour: Option<String>,
    width: Option<f32>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            colour: None,
            width: None,
        }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }

        if let Some(ref v) = other.width {
            self.width = Some(v.clone())
        }

        match other.width {
            Some(v) => self.width = Some(v),
            None => {}
        }
    }
}

impl style::Line for Style {
    fn colour<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.colour = Some(value.into());
        self
    }

    fn get_colour(&self) -> &Option<String> {
        &self.colour
    }

    fn width<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f32>,
    {
        self.width = Some(value.into());
        self
    }

    fn get_width(&self) -> &Option<f32> {
        &self.width
    }
}

pub struct Function {
    pub data: Vec<(f64, f64)>,
    style: Style,
}

impl Function {
    pub fn new<F>(f: F, lower: f64, upper: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let sampling = (upper - lower) / 200.;
        let samples = (0..)
            .map(|x| lower + (x as f64 * sampling))
            .take_while(|&x| x <= upper);
        let values = samples.map(|s| (s, f(s))).collect();
        Function {
            data: values,
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
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

impl PlanarRepresentation for Function {
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
        transform: nalgebra::Affine2<f64>,
    ) -> svg::node::element::Group {
        svg_render::draw_face_line(
            &self.data,
            transform,
            &self.style,
        )
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        transform: nalgebra::Affine2<f64>,
        face_width: u32,
        face_height: u32,
    ) -> String {
        "".into()
    }
}
