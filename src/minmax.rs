use std::default::Default;
use std::fmt;
use std::iter::{FromIterator, IntoIterator};

use Commute;

/// A commutative data structure for tracking minimum and maximum values.
///
/// This also stores the number of samples.
#[derive(Clone)]
pub struct MinMax<T> {
    len: u64,
    min: Option<T>,
    max: Option<T>,
}

impl<T: PartialOrd + Clone> MinMax<T> {
    /// Create an empty state where min and max values do not exist.
    pub fn new() -> MinMax<T> {
        Default::default()
    }

    /// Add a sample to the data.
    pub fn add(&mut self, sample: T) {
        self.len += 1;
        if self.min.as_ref().map(|v| &sample < v).unwrap_or(true) {
            self.min = Some(sample.clone());
        }
        if self.max.as_ref().map(|v| &sample > v).unwrap_or(true) {
            self.max = Some(sample);
        }
    }

    /// Returns the minimum of the data set.
    ///
    /// `None` is returned if and only if the number of samples is `0`.
    pub fn min(&self) -> Option<&T> {
        self.min.as_ref()
    }

    /// Returns the maximum of the data set.
    ///
    /// `None` is returned if and only if the number of samples is `0`.
    pub fn max(&self) -> Option<&T> {
        self.max.as_ref()
    }

    /// Returns the number of data point.
    pub fn len(&self) -> usize {
        self.len as usize
    }
}

impl<T: PartialOrd> Commute for MinMax<T> {
    fn merge(&mut self, v: MinMax<T>) {
        self.len += v.len;
        if v.min < self.min { self.min = v.min; }
        if v.max > self.max { self.max = v.max; }
    }
}

impl<T: PartialOrd> Default for MinMax<T> {
    fn default() -> MinMax<T> {
        MinMax {
            len: 0,
            min: None,
            max: None,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for MinMax<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.min, &self.max) {
            (&Some(ref min), &Some(ref max)) => {
                write!(f, "[{:?}, {:?}]", min, max)
            }
            (&None, &None) => write!(f, "N/A"),
            _ => unreachable!(),
        }
    }
}

impl<T: PartialOrd + Clone> FromIterator<T> for MinMax<T> {
    fn from_iter<I: IntoIterator<Item=T>>(it: I) -> MinMax<T> {
        let mut v = MinMax::new();
        v.extend(it);
        v
    }
}

impl<T: PartialOrd + Clone> Extend<T> for MinMax<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}

#[cfg(test)]
mod test {
    use super::MinMax;

    #[test]
    fn minmax() {
        let minmax: MinMax<usize> =
            vec![1usize, 4, 2, 3, 10].into_iter().collect();
        assert_eq!(minmax.min(), Some(&1usize));
        assert_eq!(minmax.max(), Some(&10usize));
    }
}
