/*!

Box plot

# Examples

```
# use plotlib::boxplot::Box;
# use plotlib::view::View;
let b1 = Box::from_slice(&[0., 2., 3., 4.]);
let b2 = Box::from_vec(vec![0., 2., 3., 4.]);
//let v = View::new().add(&b);
```
*/

use std::f64;
use std;

use svg;

use axis;
use representation::Representation;
use svg_render;
use style;

#[derive(Debug, Default)]
pub struct Style {
    colour: Option<String>,
}

impl Style {
    pub fn new() -> Self {
        Style { colour: None }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.colour {
            self.colour = Some(v.clone())
        }
    }
}

impl style::Box for Style {
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
}

enum BoxData<'a> {
    Owned(Vec<f64>),
    Ref(&'a [f64]),
}

pub struct Box<'a> {
    data: BoxData<'a>,
    style: Style,
}

impl<'a> Box<'a> {
    pub fn from_slice(v: &'a [(f64)]) -> Self {
        Box {
            data: BoxData::Ref(v),
            style: Style::new(),
        }
    }

    pub fn from_vec(v: Vec<f64>) -> Self {
        Box {
            data: BoxData::Owned(v),
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

    fn get_data(&'a self) -> &'a [f64] {
        match self.data {
            BoxData::Owned(ref v) => v,
            BoxData::Ref(v) => v,
        }
    }

    fn range(&self) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &v in self.get_data() {
            min = min.min(v);
            max = max.max(v);
        }
        (min, max)
    }

    fn mean(&self) -> f64 {
        mean(self.get_data())
    }

    fn quartiles(&self) -> (f64, f64, f64) {
        quartiles(self.get_data())
    }
}

fn mean(s: &[f64]) -> f64 {
    s.iter().map(|v| v / s.len() as f64).sum()
}

fn median(s: &[f64]) -> f64 {
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match s.len() % 2 {
        0 => (s[(s.len() / 2) - 1] / 2.) + (s[(s.len() / 2)] / 2.),
        _ => s[s.len() / 2],
    }
}

fn quartiles(s: &[f64]) -> (f64, f64, f64) {
    if s.len() == 1 {
        return (s[0], s[0], s[0])
    }
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (a, b) = if s.len() % 2 == 0 {
        s.split_at(s.len() / 2)
    } else {
        (&s[..(s.len() / 2)], &s[((s.len() / 2) + 1)..])
    };
    (median(a), median(&s), median(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        // TODO should error: mean(&[]);
        assert_eq!(mean(&[1.]), 1.);
        assert_eq!(mean(&[1., 2.]), 1.5);
        assert_eq!(mean(&[1., 2., 3.]), 2.);
    }

    #[test]
    fn test_median() {
        // TODO should error: median(&[]);
        assert_eq!(median(&[1.]), 1.);
        assert_eq!(median(&[1., 2.]), 1.5);
        assert_eq!(median(&[1., 2., 4.]), 2.);
        assert_eq!(median(&[1., 2., 3., 7.]), 2.5);
    }

    #[test]
    fn test_quartiles() {
        // TODO should error: quartiles(&[]);
        assert_eq!(quartiles(&[1.]), (1., 1., 1.));
        assert_eq!(quartiles(&[1., 2.]), (1., 1.5, 2.));
        assert_eq!(quartiles(&[1., 2., 4.]), (1., 2., 4.));
        assert_eq!(quartiles(&[1., 2., 3., 4.]), (1.5, 2.5, 3.5));
    }
}