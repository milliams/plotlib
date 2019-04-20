/*!

Box plot

# Examples

```
# use plotlib::repr::BoxPlot;
# use plotlib::view::CategoricalView;
let b1 = BoxPlot::from_slice(&[0., 2., 3., 4.]);
let b2 = BoxPlot::from_vec(vec![0., 2., 3., 4.]);
let v = CategoricalView::new().add(b1);
```
*/

use std::f64;

use svg;

use crate::axis;
use crate::repr::CategoricalRepresentation;
use crate::style::BoxStyle;
use crate::svg_render;
use crate::utils;

enum BoxData<'a> {
    Owned(Vec<f64>),
    Ref(&'a [f64]),
}

pub struct BoxPlot<'a> {
    data: BoxData<'a>,
    label: String,
    style: BoxStyle,
}

impl<'a> BoxPlot<'a> {
    pub fn from_slice(v: &'a [(f64)]) -> Self {
        BoxPlot {
            data: BoxData::Ref(v),
            style: BoxStyle::new(),
            label: String::new(),
        }
    }

    pub fn from_vec(v: Vec<f64>) -> Self {
        BoxPlot {
            data: BoxData::Owned(v),
            style: BoxStyle::new(),
            label: String::new(),
        }
    }

    pub fn style(mut self, style: &BoxStyle) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &BoxStyle {
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

impl<'a> CategoricalRepresentation for BoxPlot<'a> {
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
        x_axis: &axis::CategoricalAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg_render::draw_face_boxplot(
            self.get_data(),
            &self.label,
            x_axis,
            y_axis,
            face_width,
            face_height,
            &self.style,
        )
    }

    fn to_text(
        &self,
        _x_axis: &axis::CategoricalAxis,
        _y_axis: &axis::ContinuousAxis,
        _face_width: u32,
        _face_height: u32,
    ) -> String {
        "".into()
    }
}
