/*!

A module for Histograms

# Examples

```
# use plotlib::histogram::Histogram;
// Create some dummy data
let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];

// and create a histogram out of it
let h = Histogram::from_vec(&data, 30);
```

TODO:

- frequency or density option
    - Variable bins implies frequency
    - What should be the default?
*/

use svg;

use axis;
use utils::PairWise;
use svg_render;
use text_render;
use representation::Representation;

/**
A one-dimensional histogram with equal binning.
*/
#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>, // will have N_bins + 1 entries
    pub bin_counts: Vec<u32>, // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
}

impl Histogram {
    pub fn from_vec(v: &[f64], num_bins: u32) -> Histogram {

        let max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
        let min = v.iter().fold(1. / 0., |a, &b| f64::min(a, b));

        let num_bins = num_bins as usize;

        let mut bins = vec![0; num_bins];

        let range = max - min;

        let bin_width = (max - min) / num_bins as f64; // width of bin in real units

        let mut bounds: Vec<f64> =
            (0..num_bins).map(|n| (n as f64 / num_bins as f64) * range + min).collect();
        bounds.push(max);
        let bounds = bounds;

        for &val in v.iter() {
            /*
            let mut bin = ((val - min) / bin_width) as usize;
            if bin == num_bins && val == max {
                //We are right on the top-most bound
                bin = num_bins - 1;
            }
            */

            let bin = bounds.pairwise()
                .enumerate()
                .skip_while(|&(_, (&l, &u))| !(val >= l && val <= u))
                .map(|(i, (_, _))| i)
                .next()
                .unwrap();
            bins[bin] += 1;
        }
        let density_per_bin = bins.iter().map(|&x| x as f64 / bin_width).collect();

        Histogram {
            bin_bounds: bounds,
            bin_counts: bins,
            bin_densities: density_per_bin,
        }
    }

    pub fn num_bins(&self) -> usize {
        self.bin_counts.len()
    }

    fn x_range(&self) -> (f64, f64) {
        (*self.bin_bounds.first().unwrap(), *self.bin_bounds.last().unwrap())
    }

    fn y_range(&self) -> (f64, f64) {
        let max = *self.bin_counts.iter().max().unwrap();
        (0., max as f64)
    }
}

impl Representation for Histogram {
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
        svg_render::draw_face_bars(self, x_axis, y_axis, face_width, face_height)
    }

    fn to_text(&self,
               x_axis: &axis::Axis,
               y_axis: &axis::Axis,
               face_width: u32,
               face_height: u32)
               -> String {
        text_render::render_face_bars(self, x_axis, y_axis, face_width, face_height)
    }
}
