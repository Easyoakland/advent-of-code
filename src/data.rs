use num::NumCast;

use crate::cord::Cord;
use std::{
    fmt::Debug,
    ops::{Deref, RangeInclusive, Sub},
};

#[derive(Debug)]
pub struct Pair<T> {
    pub sensor: Cord<T>,
    pub beacon: Cord<T>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct SortableRangeInclusive<T>(pub RangeInclusive<T>);

impl<T: Debug> Debug for SortableRangeInclusive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("S").field(&self.0).finish()
    }
}

impl<T> Deref for SortableRangeInclusive<T> {
    type Target = RangeInclusive<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<RangeInclusive<T>> for SortableRangeInclusive<T> {
    fn from(value: RangeInclusive<T>) -> Self {
        Self(value)
    }
}

impl<T> From<SortableRangeInclusive<T>> for RangeInclusive<T> {
    fn from(value: SortableRangeInclusive<T>) -> Self {
        value.0
    }
}

impl<T: PartialOrd + Ord + Clone> PartialOrd for SortableRangeInclusive<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialOrd + Ord + Clone> Ord for SortableRangeInclusive<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = match self.start().cmp(other.start()) {
            std::cmp::Ordering::Equal => match self.end().cmp(other.end()) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal, // If both bounds equal then they are equal.
                x => x, // If greater/less return that.
            },
            x => x, // If greater/less return that.
        };

        order
    }
}

impl<T: PartialOrd + Ord + Clone + Sub<Output = T> + NumCast> SortableRangeInclusive<T> {
    // Takes two ranges in sorted order. Returns the merged array and any remaining range if it exists.
    pub fn merge(self, other: Self) -> (Self, Option<Self>) {
        // Check for overlap
        if self.end() >= &(other.start().clone() - num::cast(1).unwrap()) {
            // If other ends after self meaning they meet in the middle.
            if self.end() <= other.end() {
                return (
                    SortableRangeInclusive(self.start().clone()..=other.end().clone()),
                    None,
                );
            }
            // If self completely encloses other.
            else {
                return (self, None);
            }
        }
        (self, Some(other))
    }
}

#[cfg(test)]
mod tests {
    use super::SortableRangeInclusive;

    #[test]
    fn merge_test() {
        assert_eq!(
            SortableRangeInclusive(1..=2).merge(SortableRangeInclusive(2..=3)),
            (SortableRangeInclusive(1..=3), None)
        );
        assert_eq!(
            SortableRangeInclusive(1..=2).merge(SortableRangeInclusive(1..=4)),
            (SortableRangeInclusive(1..=4), None)
        );
        assert_eq!(
            SortableRangeInclusive(1..=2).merge((3..=4).into()),
            ((1..=4).into(), None)
        );
        /* (-4..=8), S(9..=9) */
        assert_eq!(
            SortableRangeInclusive(-4..=8).merge((9..=11).into()),
            ((-4..=11).into(), None)
        )
    }

    #[test]
    fn range_sort_order() {
        dbg!((3..2).ge(2..3)); // first range is equivalent to empty so it should go first on lexicographic compare.
        dbg!((3..2).le(2..3));
        dbg!((3..=2).ge(2..=3));
        dbg!((3..=2).le(2..=3));
        let range_sort_key = |x: &std::ops::RangeInclusive<i32>,
                              y: &std::ops::RangeInclusive<i32>| {
            if x.clone().le(y.clone()) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        };
        let mut out = vec![1..=2, 2..=3, 1..=4];
        out.sort_by(|x, y| range_sort_key(x, y));
        dbg!(out);
        let mut out = vec![1..=6, 3..=2, 6..=6, 5..=7, 5..=7, 2..=3, 1..=4];
        out.sort_by(|x, y| range_sort_key(x, y));
        dbg!(&out, out[0].clone().cmp(out[1].clone()));

        let mut out = vec![
            SortableRangeInclusive(1..=2),
            SortableRangeInclusive(2..=3),
            SortableRangeInclusive(1..=5),
            SortableRangeInclusive(3..=4),
        ];
        out.sort();
        assert_eq!(
            out,
            vec![
                (1..=2).into(),
                (1..=5).into(),
                (2..=3).into(),
                (3..=4).into()
            ]
        );
        dbg!(out);
    }
}
