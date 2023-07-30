use crate::iters::NDCartesianProduct;
use derive_more::{Deref, DerefMut};
use num_iter::range_inclusive;
use num_traits::{cast, PrimInt, Zero};
use std::{
    array,
    fmt::Debug,
    iter::Sum,
    num::NonZeroUsize,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

// Trait that allows easily adding generic bounds on cord's datatype.
pub trait CordData: PrimInt + Sum + 'static {}
impl<T: PrimInt + Sum + 'static> CordData for T {}

/// Newtype on n dimensional arrays representing coordinates in a grid-like space.
///
/// Capable of things like neighborhood calculation, cordinate addition, interpolation, etc...
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct NDCord<T, const DIM: usize>(pub [T; DIM]);

impl<T: Default + Copy, const DIM: usize> Default for NDCord<T, DIM> {
    fn default() -> Self {
        Self([Default::default(); DIM])
    }
}

impl<T, const DIM: usize> Add for NDCord<T, DIM>
where
    T: CordData,
{
    type Output = NDCord<T, DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply(rhs, T::add)
    }
}

impl<T, const DIM: usize> AddAssign for NDCord<T, DIM>
where
    T: CordData,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T, const DIM: usize> Sub for NDCord<T, DIM>
where
    T: CordData,
{
    type Output = NDCord<T, DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply(rhs, T::sub)
    }
}

impl<T, const DIM: usize> SubAssign for NDCord<T, DIM>
where
    T: CordData,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T, const DIM: usize> Mul<T> for NDCord<T, DIM>
where
    T: CordData,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        array::from_fn(|i| self[i] * rhs).into()
    }
}

impl<T, const DIM: usize> MulAssign<T> for NDCord<T, DIM>
where
    T: CordData,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T, const DIM: usize> Div<T> for NDCord<T, DIM>
where
    T: CordData,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        array::from_fn(|i| self[i] / rhs).into()
    }
}

impl<T, const DIM: usize> DivAssign<T> for NDCord<T, DIM>
where
    T: CordData,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

/// Iterator over the moore neighborhood centered at some cord.
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct MooreNeighborhoodIterator<I, T: CordData, const DIM: usize> {
    iterator: I,
    cord: NDCord<T, DIM>,
    radius: usize,
}

impl<I: Iterator<Item = NDCord<T, DIM>>, T: CordData, const DIM: usize> Iterator
    for MooreNeighborhoodIterator<I, T, DIM>
{
    type Item = NDCord<T, DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1.
        while let Some(cord_offset) = self.iterator.next() {
            let smallest_neighbor = NDCord(self.cord.0.map(|x| {
                x - cast(self.radius).expect("Can't cast the radius to cord's datatype.")
            }));
            let new_cord = smallest_neighbor + cord_offset;

            // Don't add self to neighbor list.
            if new_cord == self.cord {
                continue;
            }

            return Some(new_cord);
        }
        None
    }

    // FIXME for nd. Currently only valid for 2d.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let sidelength = self.radius + self.radius + 1;
        let area = sidelength * sidelength;
        // Area minus the cell the neighborhood is for
        (area - 1, Some(area - 1))
    }
}

impl<T, const DIM: usize> NDCord<T, DIM>
where
    T: CordData,
{
    pub fn apply<O>(self, other: Self, func: impl Fn(T, T) -> O) -> NDCord<O, DIM> {
        let mut other = other.0.into_iter();
        NDCord(
            self.0
                .map(|x| func(x, other.next().expect("Same length arrays"))),
        )
    }
    pub fn manhattan_distance(self, other: &Self) -> T {
        let diff_per_axis = self.apply(*other, abs_diff::<T>);
        diff_per_axis.0.into_iter().sum()
    }

    /// Radius is manhattan distance from center to edge.
    /// Moore neighborhood is a square formed by the extents of the Neumann neighborhood.
    pub fn moore_neighborhood(
        &self,
        radius: usize,
    ) -> MooreNeighborhoodIterator<impl Iterator<Item = NDCord<T, DIM>> + Clone, T, DIM> {
        let dim_max = cast::<usize, T>(radius + radius)
            .expect("Can't convert radius + radius into cord's datatype.");

        let iterator =
            NDCartesianProduct::new(array::from_fn(|_| range_inclusive(Zero::zero(), dim_max)))
                .map(NDCord);

        MooreNeighborhoodIterator {
            iterator,
            cord: *self,
            radius,
        }
    }

    /// Radius is manhattan distance of furthest neighbors.
    /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
    pub fn neumann_neighborhood(&self, radius: usize) -> impl Iterator<Item = NDCord<T, DIM>> + '_ {
        let neighbors = self.moore_neighborhood(radius);
        neighbors.filter(move |&x| {
            x.manhattan_distance(self)
                <= cast(radius).expect("Can't convert radius to cord's datatype.")
        })
    }

    /// Return an iterator over all points (inclusive) between `self` and `other`. Order is lexicographical.
    pub fn interpolate(&self, other: &Self) -> impl Iterator<Item = NDCord<T, DIM>> {
        // Use min and max so range doesn't silently emit no values (high..low is length 0 range)
        let ranges = array::from_fn(|i| {
            range_inclusive(self.0[i].min(other.0[i]), self.0[i].max(other.0[i]))
        });
        NDCartesianProduct::new(ranges).map(NDCord)
    }

    /// Finds the largest value in each dimension and smallest value in each dimension as the pair `(min, max)`.
    pub fn extents(&self, other: &Self) -> (Self, Self) {
        let smallest = array::from_fn(|axis| self[axis].min(other[axis]));
        let largest = array::from_fn(|axis| self[axis].max(other[axis]));
        (smallest.into(), largest.into())
    }

    /// Finds the overall extents for many [`NDCord`] with [`NDCord::extents`]. Handles empty iterator with [`None`].
    /// # Return
    /// `(min_per_axis, max_per_axis)`
    pub fn extents_iter(mut it: impl Iterator<Item = Self>) -> Option<(Self, Self)> {
        let first = it.next()?;
        Some(it.fold((first, first), |(min, max), x| {
            (x.extents(&min).0, x.extents(&max).1)
        }))
    }

    /// Find the cordinate that coresponds to a given offset where maximum width of each axis is given.
    /// Fills values of coordinates from greatest index to least.
    /// ```
    /// # use advent_lib::cord::NDCord;
    /// # use core::num::NonZeroUsize;
    /// // x x
    /// // x x
    /// // x x
    /// let widths = [2, 3].map(|x| NonZeroUsize::new(x).unwrap());
    /// assert_eq!(NDCord::from_offset(0, widths), NDCord([0, 0]));
    /// assert_eq!(NDCord::from_offset(1, widths), NDCord([1, 0]));
    /// assert_eq!(NDCord::from_offset(2, widths), NDCord([0, 1]));
    /// assert_eq!(NDCord::from_offset(3, widths), NDCord([1, 1]));
    /// assert_eq!(NDCord::from_offset(4, widths), NDCord([0, 2]));
    /// ```
    pub fn from_offset(mut offset: usize, widths: [NonZeroUsize; DIM]) -> NDCord<T, DIM>
    where
        T: From<usize>,
    {
        let mut out = [0; DIM];
        for axis in (0..DIM).rev() {
            let next_lowest_axis_width = axis.checked_sub(1);
            out[axis] = match next_lowest_axis_width {
                Some(x) => offset / widths[x],
                None => offset,
            };
            if next_lowest_axis_width.is_some() {
                offset -= out[axis] * usize::from(widths[axis - 1]);
            }
        }
        out.map(Into::into).into()
    }
}

impl<T: CordData, const DIM: usize> From<[T; DIM]> for NDCord<T, DIM> {
    fn from(value: [T; DIM]) -> Self {
        NDCord(value)
    }
}

impl<T, const DIM: usize> From<NDCord<T, DIM>> for [T; DIM] {
    fn from(value: NDCord<T, DIM>) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        let cord1 = NDCord([-2isize, 4]);
        let cord2 = NDCord([498, 6]);
        let out = cord1.manhattan_distance(&cord2);
        assert_eq!(out, 500 + 2);
    }

    #[test]
    fn moore_neighborhood_test() {
        let cord = NDCord([-8, 4]);
        let out = cord.moore_neighborhood(1);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(
            out_vec,
            vec![
                NDCord([-9, 3]),
                NDCord([-9, 4]),
                NDCord([-9, 5]),
                NDCord([-8, 3]),
                NDCord([-8, 5]),
                NDCord([-7, 3]),
                NDCord([-7, 4]),
                NDCord([-7, 5])
            ]
        );
        assert_eq!(out_vec.len(), out_size.0);
        assert_eq!(out_vec.len(), out_size.1.unwrap());
        let out = cord.moore_neighborhood(2);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(out_vec.len(), out_size.0);
        assert_eq!(out_vec.len(), out_size.1.unwrap());
        #[rustfmt::skip]
        assert_eq!(
            out_vec,
            vec![
                NDCord([-10, 2]),NDCord([-10, 3]),NDCord([-10, 4]),NDCord([-10, 5]),NDCord([-10, 6]),
                NDCord([-9, 2]), NDCord([-9, 3]), NDCord([-9, 4]), NDCord([-9, 5]), NDCord([-9, 6]),
                NDCord([-8, 2]), NDCord([-8, 3]),              NDCord([-8, 5]), NDCord([-8, 6]),
                NDCord([-7, 2]), NDCord([-7, 3]), NDCord([-7, 4]), NDCord([-7, 5]), NDCord([-7, 6]),
                NDCord([-6, 2]), NDCord([-6, 3]), NDCord([-6, 4]), NDCord([-6, 5]), NDCord([-6, 6])
            ]
        );

        let cord = NDCord([0, 0]);
        let out = cord.moore_neighborhood(3);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(out_vec.len(), out_size.0);
        assert_eq!(out_vec.len(), out_size.1.unwrap());
        #[rustfmt::skip]
        assert_eq!(
            out_vec,
            vec![
                NDCord([-3, -3]),NDCord([-3, -2]),NDCord([-3, -1]),NDCord([-3, 0]),NDCord([-3, 1]),NDCord([-3, 2]),NDCord([-3, 3]),
                NDCord([-2, -3]),NDCord([-2, -2]),NDCord([-2, -1]),NDCord([-2, 0]),NDCord([-2, 1]),NDCord([-2, 2]),NDCord([-2, 3]),
                NDCord([-1, -3]),NDCord([-1, -2]),NDCord([-1, -1]),NDCord([-1, 0]),NDCord([-1, 1]),NDCord([-1, 2]),NDCord([-1, 3]),
                NDCord([0, -3]), NDCord([0, -2]), NDCord([0, -1]),             NDCord([0, 1]), NDCord([0, 2]), NDCord([0, 3]),
                NDCord([1, -3]), NDCord([1, -2]), NDCord([1, -1]), NDCord([1, 0]), NDCord([1, 1]), NDCord([1, 2]), NDCord([1, 3]),
                NDCord([2, -3]), NDCord([2, -2]), NDCord([2, -1]), NDCord([2, 0]), NDCord([2, 1]), NDCord([2, 2]), NDCord([2, 3]),
                NDCord([3, -3]), NDCord([3, -2]), NDCord([3, -1]), NDCord([3, 0]), NDCord([3, 1]), NDCord([3, 2]), NDCord([3, 3])
            ]
        );
    }

    #[test]
    fn neumann_neighborhood_test() {
        let cord = NDCord([-8, 4]);
        let out = cord.neumann_neighborhood(1);
        assert_eq!(
            out.collect::<Vec<_>>(),
            vec![
                NDCord([-9, 4]),
                NDCord([-8, 3]),
                NDCord([-8, 5]),
                NDCord([-7, 4])
            ]
        );

        let out: Vec<_> = cord.neumann_neighborhood(2).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                          NDCord([-10, 4]),
                             NDCord([-9, 3]), NDCord([-9, 4]), NDCord([-9, 5]),
                NDCord([-8, 2]), NDCord([-8, 3]),              NDCord([-8, 5]), NDCord([-8, 6]),
                             NDCord([-7, 3]), NDCord([-7, 4]), NDCord([-7, 5]),
                                          NDCord([-6, 4])
            ]
        );

        let cord = NDCord([0, 0]);
        let out: Vec<_> = cord.neumann_neighborhood(3).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                                       NDCord([-3, 0]),
                                          NDCord([-2, -1]),NDCord([-2, 0]), NDCord([-2, 1]),
                             NDCord([-1, -2]),NDCord([-1, -1]),NDCord([-1, 0]), NDCord([-1, 1]), NDCord([-1, 2]),
                NDCord([0, -3]), NDCord([0, -2]), NDCord([0, -1]),              NDCord([0, 1]),  NDCord([0, 2]), NDCord([0, 3]),
                             NDCord([1, -2]), NDCord([1, -1]), NDCord([1, 0]),  NDCord([1, 1]), NDCord([1, 2]),
                                          NDCord([2, -1]), NDCord([2, 0]),  NDCord([2, 1]),
                                                       NDCord([3, 0])
            ]
        );
    }

    #[test]
    fn interpolate_test() {
        let cord1 = NDCord([498, 4]);
        let cord2 = NDCord([498, 6]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(
            out,
            vec![NDCord([498, 4]), NDCord([498, 5]), NDCord([498, 6])]
        );

        let cord1 = NDCord([498, 6]);
        let cord2 = NDCord([496, 6]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(
            out,
            vec![NDCord([496, 6]), NDCord([497, 6]), NDCord([498, 6])]
        );

        let cord1 = NDCord([498, 6]);
        let cord2 = NDCord([496, 7]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(
            out,
            vec![
                NDCord([496, 6]),
                NDCord([496, 7]),
                NDCord([497, 6]),
                NDCord([497, 7]),
                NDCord([498, 6]),
                NDCord([498, 7])
            ]
        );
    }

    #[test]
    fn offset_to_cord_test() {
        {
            // x x
            // x x
            // x x
            let widths = [2, 3].map(|x| NonZeroUsize::new(x).unwrap());
            assert_eq!(NDCord::from_offset(0, widths), NDCord([0, 0]));
            assert_eq!(NDCord::from_offset(1, widths), NDCord([1, 0]));
            assert_eq!(NDCord::from_offset(2, widths), NDCord([0, 1]));
            assert_eq!(NDCord::from_offset(3, widths), NDCord([1, 1]));
            assert_eq!(NDCord::from_offset(4, widths), NDCord([0, 2]));
        }
        {
            // z = 0
            // x
            // x
            // z = 1
            //  x
            //  x
            // z = 2
            //   x
            //   x
            let widths = [1, 2, 3].map(|x| NonZeroUsize::new(x).unwrap());
            assert_eq!(NDCord::from_offset(0, widths), NDCord([0, 0, 0]));
            assert_eq!(NDCord::from_offset(1, widths), NDCord([0, 1, 0]));
            assert_eq!(NDCord::from_offset(2, widths), NDCord([0, 0, 1]));
            assert_eq!(NDCord::from_offset(3, widths), NDCord([0, 1, 1]));
        }
    }
}
