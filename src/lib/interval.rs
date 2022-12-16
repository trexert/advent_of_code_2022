use itertools::Itertools;
use num::PrimInt;

use std::fmt::Debug;

macro_rules! i {
    ($min:expr, $max:expr) => {
        Interval::new($min, $max)
    };
}

macro_rules! is {
    ($min:expr, $max:expr) => {
        IntervalSet {
            intervals: vec![Interval::new($min, $max)],
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Interval<T: PrimInt + Debug> {
    min: T,
    max: T,
}

impl<T: PrimInt + Debug> Interval<T> {
    fn new(min: T, max: T) -> Self {
        assert!(min <= max);
        Interval { min, max }
    }

    fn union(self, other: Self) -> Option<Self> {
        let mini = self.min(other);
        let maxi = self.max(other);

        // With integer intervals, there is no gap between neighbouring numbers, so join them to a single interval in a union.
        if maxi.min <= mini.max + T::one() {
            Some(i!(mini.min, mini.max.max(maxi.max)))
        } else {
            None
        }
    }

    fn intersection(self, other: Self) -> Option<Self> {
        let mini = self.min(other);
        let maxi = self.max(other);

        if mini.max >= maxi.min {
            Some(i!(maxi.min, mini.max.min(maxi.max)))
        } else {
            None
        }
    }

    // Returns 0 - 2 intervals
    fn difference(self, other: Self) -> Vec<Self> {
        match (self.min < other.min, self.max > other.max) {
            (true, true) => vec![
                i!(self.min, other.min - T::one()),
                i!(other.max + T::one(), self.max),
            ],
            (true, false) => vec![i!(self.min, other.min - T::one())],
            (false, true) => vec![i!(other.max + T::one(), self.max)],
            (false, false) => vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntervalSet<T: PrimInt + Debug> {
    intervals: Vec<Interval<T>>,
}

impl<T: PrimInt + Debug> IntervalSet<T> {
    pub fn new() -> Self {
        IntervalSet { intervals: vec![] }
    }

    pub fn from_interval(min: T, max: T) -> Self {
        if min <= max {
            IntervalSet {
                intervals: vec![i!(min, max)],
            }
        } else {
            IntervalSet { intervals: vec![] }
        }
    }

    pub fn get_intervals(&self) -> Vec<(T, T)> {
        self.intervals
            .iter()
            .map(|interval| (interval.min, interval.max))
            .collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut intervals = self.intervals.clone();
        intervals.append(&mut other.intervals.clone());

        IntervalSet { intervals }.simplified()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut intervals = vec![];

        let mut temp_intervals = self.intervals.clone();
        temp_intervals.append(&mut other.intervals.clone());
        temp_intervals.sort();

        for (i1, i2) in temp_intervals.into_iter().tuple_windows() {
            if let Some(intersection) = i1.intersection(i2) {
                intervals.push(intersection);
            }
        }

        IntervalSet { intervals }.simplified()
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut intervals = vec![];

        for &self_interval in self.intervals.iter() {
            let mut current_intervals = vec![self_interval];
            let mut next_intervals = vec![];
            for &other_interval in other.intervals.iter() {
                for current_interval in current_intervals {
                    next_intervals.append(&mut current_interval.difference(other_interval))
                }
                current_intervals = next_intervals;
                next_intervals = vec![];
            }
            intervals.append(&mut current_intervals)
        }

        IntervalSet { intervals }
    }

    fn simplified(&self) -> Self {
        let mut intervals = vec![];

        if self.intervals.is_empty() {
            return self.clone();
        }

        let mut temp_intervals = self.intervals.clone();
        temp_intervals.sort();

        let mut interval_iter = temp_intervals.iter();

        let mut current_interval = *interval_iter.next().unwrap();

        for &interval in interval_iter {
            if let Some(union) = current_interval.union(interval) {
                current_interval = union;
            } else {
                intervals.push(current_interval);
                current_interval = interval;
            }
        }

        intervals.push(current_interval);

        IntervalSet { intervals }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_macro() {
        assert_eq!(i!(3, 6), Interval::new(3, 6));
        assert_eq!(
            is!(3, 6),
            IntervalSet {
                intervals: vec![i!(3, 6)]
            }
        );
    }

    #[test]
    fn union() {
        let i0 = is!(0, 3);
        let i1 = is!(3, 6);
        let i2 = is!(9, 12);
        let i3 = is!(8, 10);
        let i4 = is!(7, 7);

        assert_eq!(is!(0, 6), i0.union(&i1));
        assert_eq!(is!(8, 12), i2.union(&i3));
        assert_eq!(
            IntervalSet {
                intervals: vec![i!(0, 6), i!(8, 12)]
            },
            i2.union(&i3).union(&i0.union(&i1))
        );
        assert_eq!(is!(0, 12), i2.union(&i3).union(&i0.union(&i1)).union(&i4))
    }

    #[test]
    fn intersection() {
        let i0 = is!(0, 3);
        let i1 = is!(2, 5);
        let i2 = is!(-3, 5);

        assert_eq!(is!(2, 3), i0.intersection(&i1));
        assert_eq!(is!(0, 3), i0.intersection(&i2));
    }

    #[test]
    fn difference() {
        let i0 = is!(0, 2);
        let i1 = is!(4, 5);
        let i2 = is!(0, 5);

        assert_eq!(is!(0, 3), i2.difference(&i1));
        assert_eq!(IntervalSet::new(), i0.difference(&i2));
        assert_eq!(is!(3, 3), i2.difference(&i0.union(&i1)));
    }
}
