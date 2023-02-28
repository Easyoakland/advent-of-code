#![allow(dead_code)]
use itertools::Itertools;
use num::{range, range_inclusive, One, PrimInt, Zero};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cord<T>(pub T, pub T);

pub fn abs_diff<T: Sub<T, Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

pub trait CordData: PrimInt + Zero {}

impl<T: PrimInt + Zero> CordData for T {}

#[allow(dead_code)]
impl<Datatype> Cord<Datatype>
where
    Datatype: CordData,
{
    pub fn op1(self, f: fn(Datatype) -> Datatype) -> Self {
        Cord(f(self.0), f(self.1))
    }
    pub fn op2<T: Into<O> + From<Datatype>, O: Into<Datatype>>(
        self,
        rhs: Self,
        f: fn(T, T) -> O,
    ) -> Self {
        Cord(
            f(self.0.into(), rhs.0.into()).into(),
            f(self.1.into(), rhs.1.into()).into(),
        )
    }
    pub fn op2_refutable<T: Into<Option<Datatype>>>(
        self,
        rhs: Self,
        f: fn(Datatype, Datatype) -> T,
    ) -> Option<Self> {
        let x = f(self.0, rhs.0).into()?;
        let y = f(self.1, rhs.1).into()?;
        Some(Cord(x, y))
    }

    pub fn manhattan_distance(self, other: &Self) -> Datatype {
        // Try one way and if it doesn't give valid value try other.
        let temp = self.op2(*other, abs_diff::<Datatype>);
        temp.0 + temp.1
    }

    /// Radius is manhattan distance from center to edge.
    /// Moore neighborhood is a square formed by the extents of the Neumann neighborhood.
    pub fn moore_neighborhood(&self, radius: Datatype) -> Vec<Cord<Datatype>> {
        let mut neighbors = Vec::new();
        // Goes from left to right and from top to bottom generating neighbor cords.
        // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1
        for (i, j) in range(Zero::zero(), radius + radius + One::one())
            .cartesian_product(range(Zero::zero(), radius + radius + One::one()))
        {
            let x = self.0.checked_sub(&radius);
            let y = self.1.checked_sub(&radius);
            let (x, y) = match (x, y) {
                (Some(a), Some(b)) => (a.add(i), b.add(j)),
                _ => continue,
            };

            // If neither is negative can safely convert to unsigned.
            let x = x;
            let y = y;

            // Don't add self to neighbor list.
            if x == self.0 && y == self.1 {
                continue;
            }

            neighbors.push(Cord(x, y));
        }

        neighbors
    }

    /// Radius is manhattan distance of furthest neighbors.
    /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
    pub fn neumann_neighborhood(&self, radius: Datatype) -> Vec<Cord<Datatype>> {
        let neighbors = self.moore_neighborhood(radius);
        neighbors
            .into_iter()
            .filter(|&x| x.manhattan_distance(self) <= radius)
            .collect()
    }

    /// Returns a vector of every cordinate with an x or y value between self and other inclusive.
    pub fn interpolate(&self, other: &Self) -> Vec<Cord<Datatype>> {
        let mut out = Vec::new();
        for (x, y) in range_inclusive(self.0.min(other.0), self.0.max(other.0))
            .cartesian_product(range_inclusive(self.1.min(other.1), self.1.max(other.1)))
        {
            out.push(Cord(x, y));
        }
        out
    }
}

impl<T: CordData> Add for Cord<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op2(rhs, T::add)
    }
}

impl<T: CordData> AddAssign<Self> for Cord<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: CordData> Sub<Self> for Cord<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.op2(rhs, T::sub)
    }
}

impl<T: CordData> SubAssign<Self> for Cord<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: CordData> From<(T, T)> for Cord<T> {
    fn from(value: (T, T)) -> Self {
        Cord(value.0, value.1)
    }
}

impl<T> From<Cord<T>> for (T, T) {
    fn from(value: Cord<T>) -> Self {
        (value.0, value.1)
    }
}

pub fn offset_to_cord<T>(offset: T, width: T) -> Cord<T>
where
    T: std::ops::Div<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    let y = offset / width;
    let x = offset - width * y;
    Cord(x, y)
}

#[cfg(test)]
mod tests {
    use super::Cord;

    #[test]
    fn manhattan_distance_test() {
        let cord1 = Cord(-2isize, 4);
        let cord2 = Cord(498, 6);
        let out = cord1.manhattan_distance(&cord2);
        assert_eq!(out, 500 + 2);
    }

    #[test]
    fn interpolate_test() {
        let cord1 = Cord(498, 4);
        let cord2 = Cord(498, 6);
        let out = cord1.interpolate(&cord2);
        assert_eq!(out, vec![Cord(498, 4), Cord(498, 5), Cord(498, 6)]);

        let cord1 = Cord(498, 6);
        let cord2 = Cord(496, 6);
        let out = cord1.interpolate(&cord2);
        assert_eq!(out, vec![Cord(496, 6), Cord(497, 6), Cord(498, 6)]);

        let cord1 = Cord(498, 6);
        let cord2 = Cord(496, 7);
        let out = cord1.interpolate(&cord2);
        assert_eq!(
            out,
            vec![
                Cord(496, 6),
                Cord(496, 7),
                Cord(497, 6),
                Cord(497, 7),
                Cord(498, 6),
                Cord(498, 7)
            ]
        );
    }
}
