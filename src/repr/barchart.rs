/*!

Bar chart

# Examples

```
# use plotlib::repr::BarChart;
# use plotlib::view::CategoricalView;
let b1 = BarChart::new(5.2).label("b1");
let b2 = BarChart::new(1.6).label("b2");
let v = CategoricalView::new().add(b1).add(b2);
```
*/

use std::f64;

use svg;

use crate::axis;
use crate::repr::CategoricalRepresentation;
use crate::style::BoxStyle;
use crate::svg_render;

pub struct BarChart {
    value: f64,
    label: String,
    style: BoxStyle,
}

impl BarChart {
    pub fn new(v: f64) -> Self {
        BarChart {
            value: v,
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

    fn get_value(&self) -> f64 {
        self.value
    }
}

impl CategoricalRepresentation for BarChart {
    /// The maximum range. Used for auto-scaling axis
    fn range(&self) -> (f64, f64) {
        (0.0, self.value)
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
        svg_render::draw_face_barchart(
            self.get_value(),
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
