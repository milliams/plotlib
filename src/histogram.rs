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

use axis;
use utils::PairWise;

#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>, // will have N_bins + 1 entries
    pub bin_counts: Vec<u32>, // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
    pub x_axis: axis::Axis,
    pub y_axis: axis::Axis,
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

            let bin = bounds.pairwise().enumerate().skip_while(|&(_, (&l, &u))| !(val >= l && val <= u)).map(|(i, (_,_))| i).next().unwrap();
            bins[bin] += 1;
        }
        let density_per_bin = bins.iter().map(|&x| x as f64 / bin_width).collect();

        let x_min = *bounds.first().expect("ERROR: There are no ticks for the x-axis");
        let x_max = *bounds.last().expect("ERROR: There are no ticks for the x-axis");
        let x_axis = axis::Axis::new(x_min, x_max);

        let largest_bin_count = *bins.iter().max().expect("ERROR: There are no bins");
        let y_axis = axis::Axis::new(0.0, largest_bin_count as f64);

        Histogram {
            bin_bounds: bounds,
            bin_counts: bins,
            bin_densities: density_per_bin,
            x_axis: x_axis,
            y_axis: y_axis,
        }
    }

    pub fn num_bins(&self) -> usize {
        self.bin_counts.len()
    }
}
