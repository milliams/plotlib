/*!

A module for Histograms

# Examples

```
# use plotlib::histogram::Histogram;
// Create some dummy data
let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];

// and create a histogram out of it
let h = Histogram::from_slice(&data, 30);
```

TODO:

- frequency or density option
    - Variable bins implies frequency
    - What should be the default?
*/

use svg;
use nalgebra;

use axis;
use utils::PairWise;
use svg_render;
use text_render;
use representation::PlanarRepresentation;
use style;

#[derive(Debug, Default)]
pub struct Style {
    fill: Option<String>,
}

impl Style {
    pub fn new() -> Self {
        Style { fill: None }
    }

    pub fn overlay(&mut self, other: &Self) {
        if let Some(ref v) = other.fill {
            self.fill = Some(v.clone())
        }
    }
}

impl style::Bar for Style {
    fn fill<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.fill = Some(value.into());
        self
    }

    fn get_fill(&self) -> &Option<String> {
        &self.fill
    }
}

/**
A one-dimensional histogram with equal binning.
*/
#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>,    // will have N_bins + 1 entries
    pub bin_counts: Vec<u32>,    // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
    style: Style,
}

impl Histogram {
    pub fn from_slice(v: &[f64], num_bins: usize) -> Histogram {
        let mut max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
        let mut min = v.iter().fold(1. / 0., |a, &b| f64::min(a, b));

        if min == max {
            min = min - 0.5;
            max = max + 0.5;
        }

        let mut bins = vec![0; num_bins];

        let range = max - min;

        let bin_width = (max - min) / num_bins as f64; // width of bin in real units

        let mut bounds: Vec<f64> = (0..num_bins)
            .map(|n| (n as f64 / num_bins as f64) * range + min)
            .collect();
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

            let bin = bounds
                .pairwise()
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
            style: Style::new(),
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
        let max = *self.bin_counts.iter().max().unwrap();
        (0., max as f64)
    }

    pub fn style(mut self, style: &Style) -> Self {
        self.style.overlay(style);
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }
}

impl PlanarRepresentation for Histogram {
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
        transform: nalgebra::Affine2<f64>,
    ) -> svg::node::element::Group {
        svg_render::draw_face_bars(self, transform, &self.style)
    }

    fn to_text(
        &self,
        x_axis: &axis::ContinuousAxis,
        y_axis: &axis::ContinuousAxis,
        transform: nalgebra::Affine2<f64>,
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
        assert_eq!(Histogram::from_slice(&[], 3).bin_densities, [0., 0., 0.]);
        assert_eq!(Histogram::from_slice(&[0.], 3).bin_densities, [0., 3., 0.]);
        assert_eq!(
            Histogram::from_slice(&[0., 3.], 3).bin_densities,
            [1., 0., 1.]
        );
        assert_eq!(
            Histogram::from_slice(&[0., 1., 2., 3.], 3).bin_densities,
            [2., 1., 1.]
        );
    }
}
