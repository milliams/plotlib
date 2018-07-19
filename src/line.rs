/*!

Plot line charts

# Examples

```
# use plotlib::line::Line;
# use plotlib::view::ContinuousView;
// y=x^2 between 0 and 10
let l = Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)]);
let v = ContinuousView::new().add(&l);
```
*/

use std::f64;

use svg;

use axis;
use errors::Result;
use representation::ContinuousRepresentation;
use style;
use svg_render;

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

pub struct Line {
    pub data: Vec<(f64, f64)>,
    style: Style,
}

impl Line {
    pub fn new(v: &[(f64, f64)]) -> Self {
        Line {
            data: v.into(),
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

    fn x_range(&self) -> Result<(f64, f64)> {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(x, _) in &self.data {
            min = min.min(x);
            max = max.max(x);
        }
        Ok((min, max))
    }

    fn y_range(&self) -> Result<(f64, f64)> {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &(_, y) in &self.data {
            min = min.min(y);
            max = max.max(y);
        }
        Ok((min, max))
    }
}

impl ContinuousRepresentation for Line {
    fn range(&self, dim: u32) -> Result<(f64, f64)> {
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
    ) -> Result<svg::node::element::Group> {
        Ok(svg_render::draw_face_line(
            &self.data,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        ))
    }

    fn to_text(
        &self,
        _x_axis: &axis::ContinuousAxis,
        _y_axis: &axis::ContinuousAxis,
        _face_width: u32,
        _face_height: u32,
    ) -> Result<String> {
        Ok("".into())
    }
}
