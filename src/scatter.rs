use std::f64;

use axis;

#[derive(Debug)]
pub struct Scatter {
    pub data: Vec<(f64, f64)>,
    pub x_axis: axis::Axis,
    pub y_axis: axis::Axis,
}

impl Scatter {
    pub fn from_vec(v: &[(f64, f64)]) -> Scatter {

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut data:  Vec<(f64, f64)> = vec![];
        for &(x, y) in v {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
            data.push((x, y));
        }

        let x_axis = axis::Axis::new(x_min, x_max);
        let y_axis = axis::Axis::new(y_min, y_max);

        Scatter {
            data: data,
            x_axis: x_axis,
            y_axis: y_axis,
        }
    }
}

