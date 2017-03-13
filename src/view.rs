use std::f64;

use svg;
use svg::Node;

use representation::Representation;
use axis;
use svg_render;

/// Standard 1-dimensional view with a continuous x-axis
pub struct View<'a> {
    pub representations: Vec<&'a Representation>,
    //x_axis: axis::Axis,
    //y_axis: axis::Axis,
}

impl<'a> View<'a> {
    pub fn new() -> View<'a> {
        View {
            representations: vec![],
            //x_axis: axis::Axis::new(0., 1.),
            //y_axis: axis::Axis::new(0., 1.),
        }
    }

    pub fn add(mut self, repr: &'a Representation) -> Self {
        /*let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        
        for &(x, y) in repr.data().iter() {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        x_min = x_min - (x_range / 20.0);
        x_max = x_max + (x_range / 20.0);
        y_min = y_min - (y_range / 20.0);
        y_max = y_max + (y_range / 20.0);*/

        self.representations.push(repr);

        self
    }

    //fn 

    pub fn to_svg(&self) -> svg::node::element::Group {
        let face_width = 500.;
        let face_height = 350.;

        let mut view_group = svg::node::element::Group::new();

        // TODO this axis wrangling will need to be done more cleverly
        // For each repr, get the x and y range and work out a default view which encopasses them all
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for repr in self.representations.iter() {
            let (this_x_min, this_x_max) = repr.x_range();
            let (this_y_min, this_y_max) = repr.y_range();
            x_min = x_min.min(this_x_min);
            x_max = x_max.max(this_x_max);
            y_min = y_min.min(this_y_min);
            y_max = y_max.max(this_y_max);
        }

        let x_axis = axis::Axis::new(x_min, x_max);
        let y_axis = axis::Axis::new(y_min, y_max);

        // Then, based on those ranges, draw each repr as an SVG
        for repr in self.representations.iter() {
            let repr_group = repr.to_svg(&x_axis, &y_axis, face_width, face_height);
            view_group.append(repr_group);
            
        }
        // Add in the axes
        view_group.append(svg_render::draw_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));
        view_group
    }
}
