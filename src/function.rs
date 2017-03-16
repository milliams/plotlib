use svg;

use axis;
use representation::Representation;

pub struct Function {
    func: Fn(f64) -> f64,
}

impl Function {
    fn x_range(&self) -> (f64, f64) {
        (0., 1.)
    }

    fn y_range(&self) -> (f64, f64) {
        (0., 1.)
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
        svg::node::element::Group::new()
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
