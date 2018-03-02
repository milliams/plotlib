/*!

Box plot

# Examples

```
# use plotlib::boxplot::Box;
# use plotlib::view::DiscreteView;
let b1 = Box::from_slice(&[0., 2., 3., 4.]);
let b2 = Box::from_vec(vec![0., 2., 3., 4.]);
let v = DiscreteView::new().add(&b1);
```
*/

use std::f64;

use svg;

use axis;
use representation::DiscreteRepresentation;
use svg_render;
use style;
use utils;

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

impl style::BoxPlot for Style {
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
    label: String,
    style: Style,
}

impl<'a> Box<'a> {
    pub fn from_slice(v: &'a [(f64)]) -> Self {
        Box {
            data: BoxData::Ref(v),
            style: Style::new(),
            label: String::new(),
        }
    }

    pub fn from_vec(v: Vec<f64>) -> Self {
        Box {
            data: BoxData::Owned(v),
            style: Style::new(),
            label: String::new(),
        }
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = label.into();
        self
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    fn get_data(&'a self) -> &'a [f64] {
        match self.data {
            BoxData::Owned(ref v) => v,
            BoxData::Ref(v) => v,
        }
    }

    fn range(&self) -> (f64, f64) {
        match self.data {
            BoxData::Owned(ref v) => utils::range(v),
            BoxData::Ref(v) => utils::range(v),
        }
    }
}

impl<'a> DiscreteRepresentation for Box<'a> {
    /// The maximum range. Used for auto-scaling axis
    fn range(&self) -> (f64, f64) {
        self.range()
    }

    /// The ticks that this representation covers. Used to collect all ticks for display
    fn ticks(&self) -> Vec<String> {
        vec![self.label.clone()]
    }

    fn to_svg(
        &self,
        x_axis: &axis::DiscreteAxis,
        y_axis: &axis::Axis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg::node::element::Group::new()
    }

    fn to_text(
        &self,
        x_axis: &axis::DiscreteAxis,
        y_axis: &axis::Axis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        "".into()
    }
}
