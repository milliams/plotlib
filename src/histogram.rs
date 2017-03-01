//! A module for Histograms
//!
//! TODO:
//!  - frequency or density option
//!    - Variable bins implies frequency
//!    - What should be the default?

#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>, // will have N_bins + 1 entries
    pub bin_counts: Vec<u32>, // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
}

impl Histogram {
    pub fn from_vec(v: &[f64]) -> Histogram {

        let max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
        let min = v.iter().fold(1. / 0., |a, &b| f64::min(a, b));

        let num_bins = 30; // Number of bins

        let mut bins = vec![0; num_bins];

        let range = max - min;

        let bin_width = (max - min) / num_bins as f64; // width of bin in real units

        let mut bounds: Vec<f64> =
            (0..num_bins).map(|n| (n as f64 / num_bins as f64) * range + min).collect();
        bounds.push(max);
        let bounds = bounds;

        for &val in v.iter() {
            let mut bin = ((val - min) / bin_width) as usize;
            if bin == num_bins && val == max {
                //We are right on the top-most bound
                bin = num_bins - 1;
            }
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
}
