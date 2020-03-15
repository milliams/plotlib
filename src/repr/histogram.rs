/*!

A module for Histograms

# Examples

```
# use plotlib::repr::Histogram;
// Create some dummy data
let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];

// and create a histogram out of it
let h = Histogram::from_slice(&data, plotlib::repr::HistogramBins::Count(30));
```

TODO:

- frequency or density option
    - Variable bins implies frequency
    - What should be the default?
*/

use std;

use svg;

use crate::axis;
use crate::repr::ContinuousRepresentation;
use crate::style::BoxStyle;
use crate::svg_render;
use crate::text_render;
use crate::utils::PairWise;

#[derive(Debug)]
enum HistogramType {
    Count,
    Density,
}

#[derive(Debug)]
pub enum HistogramBins {
    Count(usize),
    Bounds(Vec<f64>),
}

/**
A one-dimensional histogram with equal binning.
*/
#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>,    // will have N_bins + 1 entries
    pub bin_counts: Vec<f64>,    // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
    style: BoxStyle,
    h_type: HistogramType,
}

impl Histogram {
    pub fn from_slice(v: &[f64], bins: HistogramBins) -> Histogram {
        let mut max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
        let mut min = v.iter().fold(1. / 0., |a, &b| f64::min(a, b));

        if (min - max).abs() < std::f64::EPSILON {
            min -= 0.5;
            max += 0.5;
        }

        let (num_bins, bounds) = match bins {
            HistogramBins::Count(num_bins) => {
                let range = max - min;
                let mut bounds: Vec<f64> = (0..num_bins)
                    .map(|n| (n as f64 / num_bins as f64) * range + min)
                    .collect();
                bounds.push(max);
                (num_bins, bounds)
            }
            HistogramBins::Bounds(bounds) => (bounds.len(), bounds),
        };

        let mut bins = vec![0; num_bins];

        let bin_width = (max - min) / num_bins as f64; // width of bin in real units

        for &val in v.iter() {
            let bin = bounds
                .pairwise()
                .enumerate()
                .skip_while(|&(_, (&l, &u))| !(val >= l && val <= u))
                .map(|(i, (_, _))| i)
                .next()
                .unwrap();
            bins[bin] += 1;
        }
        let density_per_bin = bins.iter().map(|&x| f64::from(x) / bin_width).collect();

        Histogram {
            bin_bounds: bounds,
            bin_counts: bins.iter().map(|&x| f64::from(x)).collect(),
            bin_densities: density_per_bin,
            style: BoxStyle::new(),
            h_type: HistogramType::Count,
        }
    }

    pub fn num_bins(&self) -> usize {
        self.bin_counts.len()
    }

    fn x_range(&self) -> (f64, f64) {
        (
            *self.bin_bounds.first().unwrap(),
            *self.bin_bounds.last().unwrap(),
        )
    }

    fn y_range(&self) -> (f64, f64) {
        let max = self
            .get_values()
            .iter()
            .fold(-1. / 0., |a, &b| f64::max(a, b));
        (0., max)
    }

    pub fn style(mut self, style: &BoxStyle) -> Self {
        self.style.overlay(style);
        self
    }

    /**
    Set the histogram to display as normalised densities
    */
    pub fn density(mut self) -> Self {
        self.h_type = HistogramType::Density;
        self
    }

    pub fn get_style(&self) -> &BoxStyle {
        &self.style
    }

    pub fn get_values(&self) -> &[f64] {
        match self.h_type {
            HistogramType::Count => &self.bin_counts,
            HistogramType::Density => &self.bin_densities,
        }
    }
}

impl ContinuousRepresentation for Histogram {
    fn range(&self, dim: u32) -> (f64, f64) {
        match dim {
            0 => self.x_range(),
            1 => self.y_range(),
            _ => panic!("Axis out of range"),
        }
    }

    fn to_svg(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: f64,
        face_height: f64,
    ) -> svg::node::element::Group {
        svg_render::draw_face_bars(self, x_axis, y_axis, face_width, face_height, &self.style)
    }
    fn legend_svg(&self) -> Option<svg::node::element::Group> {
        // TODO implement
        None
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        face_width: u32,
        face_height: u32,
    ) -> String {
        text_render::render_face_bars(self, x_axis, y_axis, face_width, face_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_from_slice() {
        assert_eq!(
            Histogram::from_slice(&[], HistogramBins::Count(3)).get_values(),
            [0., 0., 0.]
        );
        assert_eq!(
            Histogram::from_slice(&[0.], HistogramBins::Count(3)).get_values(),
            [0., 1., 0.]
        );
        assert_eq!(
            Histogram::from_slice(&[0., 3.], HistogramBins::Count(3)).get_values(),
            [1., 0., 1.]
        );
        assert_eq!(
            Histogram::from_slice(&[0., 1., 2., 3.], HistogramBins::Count(3)).get_values(),
            [2., 1., 1.]
        );
    }

    #[test]
    fn test_histogram_define_bin_bounds() {
        assert_eq!(
            Histogram::from_slice(&[0., 1.], HistogramBins::Count(3)).bin_bounds,
            [0., 1. / 3., 2. / 3., 1.]
        );
        assert_eq!(
            Histogram::from_slice(&[], HistogramBins::Bounds([0., 1., 1.5, 2., 5.6].to_vec()))
                .bin_bounds,
            [0., 1., 1.5, 2., 5.6]
        );
    }
}
