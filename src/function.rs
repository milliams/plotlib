use std::f64;

use svg;

use axis;
use representation::Representation;

use svg_render;

#[derive(Debug)]
pub struct Style {
    colour: Option<String>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            colour: None,
        }
    }

    pub fn overlay(&mut self, other: Self) {
        match other.colour {
            Some(v) => self.colour = Some(v),
            None => {}
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

pub struct Function {
    pub data: Vec<(f64, f64)>,
    style: Style,
}

impl Function {
    pub fn new<F>(f: F, lower: f64, upper: f64) -> Self
        where F: Fn(f64) -> f64 {
        let sampling = (upper - lower) / 200.;
        let samples = (0..).map(|x| lower + (x as f64 * sampling)).take_while(|&x| x <= upper);
        let values = samples.map(|s| (s, f(s))).collect();
        Function {
            data: values,
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

impl Representation for Function {
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
        svg_render::draw_face_line(self, x_axis, y_axis, face_width, face_height, &self.style)
    }

    fn to_text(&self,
               x_axis: &axis::Axis,
               y_axis: &axis::Axis,
               face_width: u32,
               face_height: u32)
               -> String {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let f = Function::new(|x| x*x, 0., 10.);
    }
}
