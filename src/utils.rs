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
}
