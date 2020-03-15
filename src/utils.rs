use std::f64;
use std::iter::{Skip, Zip};
use std::slice::Iter;

pub trait PairWise<T> {
    fn pairwise(&self) -> Zip<Iter<T>, Skip<Iter<T>>>;
}

impl<T> PairWise<T> for [T] {
    fn pairwise(&self) -> Zip<Iter<T>, Skip<Iter<T>>> {
        self.iter().zip(self.iter().skip(1))
    }
}

fn _mean(s: &[f64]) -> f64 {
    s.iter().map(|v| v / s.len() as f64).sum()
}

pub fn median(s: &[f64]) -> f64 {
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match s.len() % 2 {
        0 => (s[(s.len() / 2) - 1] / 2.) + (s[(s.len() / 2)] / 2.),
        _ => s[s.len() / 2],
    }
}

pub fn quartiles(s: &[f64]) -> (f64, f64, f64) {
    if s.len() == 1 {
        return (s[0], s[0], s[0]);
    }
    let mut s = s.to_owned();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (a, b) = if s.len() % 2 == 0 {
        s.split_at(s.len() / 2)
    } else {
        (&s[..(s.len() / 2)], &s[((s.len() / 2) + 1)..])
    };
    (median(a), median(&s), median(b))
}

pub fn range(s: &[f64]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for &v in s {
        min = min.min(v);
        max = max.max(v);
    }
    (min, max)
}

/**
Floor or ceiling the min or max to zero to avoid them both having the same value
*/
pub fn pad_range_to_zero(min: f64, max: f64) -> (f64, f64) {
    if (min - max).abs() < std::f64::EPSILON {
        (if min > 0. {0.} else {min}, if max < 0. {0.} else {max})
    } else {
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(a.pairwise().nth(0).unwrap(), (&1, &2));
        assert_eq!(a.pairwise().last().unwrap(), (&4, &5));
        assert_eq!(a.pairwise().len(), a.len() - 1);

        let a = [1, 2];
        assert_eq!(a.pairwise().nth(0).unwrap(), (&1, &2));
        assert_eq!(a.pairwise().last().unwrap(), (&1, &2));
        assert_eq!(a.pairwise().len(), a.len() - 1);

        let a = [1];
        assert!(a.pairwise().nth(0).is_none());

        let b: Vec<f64> = vec![0.0, 0.1, 0.2];
        assert_eq!(b.pairwise().nth(0).unwrap(), (&0.0, &0.1));
    }

    #[test]
    fn test_mean() {
        // TODO should error: mean(&[]);
        assert_eq!(_mean(&[1.]), 1.);
        assert_eq!(_mean(&[1., 2.]), 1.5);
        assert_eq!(_mean(&[1., 2., 3.]), 2.);
    }

    #[test]
    fn test_median() {
        // TODO should error: median(&[]);
        assert_eq!(median(&[1.]), 1.);
        assert_eq!(median(&[1., 2.]), 1.5);
        assert_eq!(median(&[1., 2., 4.]), 2.);
        assert_eq!(median(&[1., 2., 3., 7.]), 2.5);
    }

    #[test]
    fn test_quartiles() {
        // TODO should error: quartiles(&[]);
        assert_eq!(quartiles(&[1.]), (1., 1., 1.));
        assert_eq!(quartiles(&[1., 2.]), (1., 1.5, 2.));
        assert_eq!(quartiles(&[1., 2., 4.]), (1., 2., 4.));
        assert_eq!(quartiles(&[1., 2., 3., 4.]), (1.5, 2.5, 3.5));
    }

    #[test]
    fn test_pad_range_to_zero() {
        assert_eq!(pad_range_to_zero(2.0, 2.0), (0.0, 2.0));
        assert_eq!(pad_range_to_zero(-2.0, 2.0), (-2.0, 2.0));
        assert_eq!(pad_range_to_zero(-2.0, -2.0), (-2.0, 0.0));
    }
}
