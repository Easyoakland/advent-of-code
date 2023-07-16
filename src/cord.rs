use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use num_iter::range_inclusive;
use num_traits::{cast, PrimInt, Zero};
use std::{
    array,
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

// Trait that allows easily adding generic bounds on cord's datatype.
pub trait CordData: PrimInt + Sum + 'static + Debug {}
impl<T: PrimInt + Sum + 'static + Debug> CordData for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct Cord<T, const DIM: usize>(pub [T; DIM]);

impl<T: Default + Copy, const DIM: usize> Default for Cord<T, DIM> {
    fn default() -> Self {
        Self([Default::default(); DIM])
    }
}

impl<T, const DIM: usize> Add for Cord<T, DIM>
where
    T: CordData,
{
    type Output = Cord<T, DIM>;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply(rhs, T::add)
    }
}

impl<T, const DIM: usize> AddAssign for Cord<T, DIM>
where
    T: CordData,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T, const DIM: usize> Sub for Cord<T, DIM>
where
    T: CordData,
{
    type Output = Cord<T, DIM>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply(rhs, T::sub)
    }
}

impl<T, const DIM: usize> SubAssign for Cord<T, DIM>
where
    T: CordData,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

pub struct MooreNeighborhoodIterator<T: CordData, const DIM: usize> {
    iterator: Box<dyn Iterator<Item = Cord<T, DIM>>>,
    cord: Cord<T, DIM>,
    radius: usize,
}

impl<Datatype: CordData, const DIM: usize> Iterator for MooreNeighborhoodIterator<Datatype, DIM> {
    type Item = Cord<Datatype, DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        // Goes from left to right and from top to bottom generating neighbor cords.
        // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1
        while let Some(cord_offset) = self.iterator.next() {
            let new_cord = Cord(self.cord.0.map(|x| {
                x - cast(self.radius).expect("Can't cast the radius to cord's datatype.")
            })) + cord_offset;

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

impl<Datatype, const DIM: usize> Cord<Datatype, DIM>
where
    Datatype: CordData + 'static,
{
    pub fn apply<O>(self, other: Self, func: impl Fn(Datatype, Datatype) -> O) -> Cord<O, DIM> {
        let mut other = other.0.into_iter();
        Cord(
            self.0
                .map(|x| func(x, other.next().expect("Same length arrays"))),
        )
    }
    pub fn manhattan_distance(self, other: &Self) -> Datatype {
        let diff_per_axis = self.apply(*other, abs_diff::<Datatype>);
        diff_per_axis.0.into_iter().sum()
    }

    /// Radius is manhattan distance from center to edge.
    /// Moore neighborhood is a square formed by the extents of the Neumann neighborhood.
    pub fn moore_neighborhood(&self, radius: usize) -> MooreNeighborhoodIterator<Datatype, DIM> {
        let dim_max = cast::<usize, Datatype>(radius + radius)
            .expect("Can't convert 2*radius + 1 into cord's datatype.");

        // TODO fix below to use NDCartesianProduct
        let iterator: Box<dyn Iterator<Item = Cord<Datatype, DIM>>> = {
            let n_dimension = DIM;

            // Setup 0th dimension.
            let iterator = Box::new(range_inclusive(Zero::zero(), dim_max).map(|x| {
                let mut array = [Zero::zero(); DIM];
                array[0] = x;
                array
            }));
            // Setup further dimensions.
            let mut iter: Box<dyn Iterator<Item = [Datatype; DIM]>> = iterator;
            for i in 1..n_dimension {
                iter = Box::new(
                    iter.cartesian_product(range_inclusive(Zero::zero(), dim_max))
                        .map(move |mut x| {
                            x.0[i] = x.1;
                            x.0
                        }),
                );
            }
            Box::new(iter.map(Cord))
        };

        MooreNeighborhoodIterator {
            iterator,
            cord: *self,
            radius,
        }
    }

    /// Radius is manhattan distance of furthest neighbors.
    /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
    pub fn neumann_neighborhood(
        &self,
        radius: usize,
    ) -> impl Iterator<Item = Cord<Datatype, DIM>> + '_ {
        let neighbors = self.moore_neighborhood(radius);
        neighbors.filter(move |&x| {
            x.manhattan_distance(&self)
                <= cast(radius).expect("Can't convert radius to cord's datatype.")
        })
    }

    /// Returns a vector of every cordinate with an x or y value between self and other inclusive.
    pub fn interpolate(&self, other: &Self) -> impl Iterator<Item = Cord<Datatype, DIM>> {
        // Use min and max so range doesn't silently emit no values (high..low is length 0 range)
        let ranges = array::from_fn(|i| {
            range_inclusive(self.0[i].min(other.0[i]), self.0[i].max(other.0[i]))
        });
        NDCartesianProduct::new(ranges).map(Cord)
    }
}

impl<T: CordData, const DIM: usize> From<[T; DIM]> for Cord<T, DIM> {
    fn from(value: [T; DIM]) -> Self {
        Cord(value)
    }
}

impl<T, const DIM: usize> From<Cord<T, DIM>> for [T; DIM] {
    fn from(value: Cord<T, DIM>) -> Self {
        value.0
    }
}

/* pub fn offset_to_cord<T, const DIM: usize>(offset: T, width: T) -> Cord<T, DIM>
where
    T: std::ops::Div<Output = T> + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    let y = offset / width;
    let x = offset - width * y;
    Cord([x, y])
} */

/// Determines next value of products in lexicographic order
fn next_product_iter<T, const N: usize, I>(
    mut current: [T; N],
    next_val_per_idx: &mut [I; N],
    reset_per_idx: &[I; N],
) -> Option<[T; N]>
where
    I: Iterator<Item = T> + Clone,
{
    // Start at least significant digit first.
    for i in (0..N).rev() {
        // If still new values for idx get next and return.
        if let Some(next) = next_val_per_idx[i].next() {
            current[i] = next;
            return Some(current);
        }
        // If still more to check reset it and try next
        else if i > 0 {
            next_val_per_idx[i] = reset_per_idx[i].clone();
            current[i] = next_val_per_idx[i].next().expect("Already reset iterator");
        }
    }
    // If no more to check and all are at max then there is no more.
    return None;
}

/// N dimensional product in lexicographical order
struct NDCartesianProduct<I, const N: usize>
where
    I: Iterator,
{
    original_iters: [I; N],
    next_val_iters: [I; N],
    current: [I::Item; N],
}

impl<I, const N: usize> NDCartesianProduct<I, N>
where
    I: Iterator + Clone,
    I::Item: Debug,
{
    fn new(mut values_per_axis: [I; N]) -> Self {
        let original_iters = values_per_axis.clone();
        // The length of current is N and so is values per axis. This unwrap should thus never fail unless an empty iterator is used.
        // The values_per_axis are purposefully stepped here so that the lower bound is not repeated.
        let current = array::from_fn(|i| {
            values_per_axis[i]
                .next()
                .expect("All values per axis should have at least 1 valid value.")
        });

        // Reset the least significant idx (0) so the first element is not skipped
        match (values_per_axis.last_mut(), original_iters.last()) {
            (Some(x), Some(y)) => *x = y.clone(),
            _ => (),
        }

        NDCartesianProduct {
            original_iters,
            next_val_iters: values_per_axis,
            current,
        }
    }
}

impl<I: Iterator + Clone, const N: usize> Iterator for NDCartesianProduct<I, N>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        self.current = next_product_iter(
            self.current.clone(),
            &mut self.next_val_iters,
            &self.original_iters,
        )?;
        Some(self.current.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        let cord1 = Cord([-2isize, 4]);
        let cord2 = Cord([498, 6]);
        let out = cord1.manhattan_distance(&cord2);
        assert_eq!(out, 500 + 2);
    }

    #[test]
    fn moore_neighborhood_test() {
        let cord = Cord([-8, 4]);
        let out = cord.moore_neighborhood(1);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(
            out_vec,
            vec![
                Cord([-9, 3]),
                Cord([-9, 4]),
                Cord([-9, 5]),
                Cord([-8, 3]),
                Cord([-8, 5]),
                Cord([-7, 3]),
                Cord([-7, 4]),
                Cord([-7, 5])
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
                Cord([-10, 2]),Cord([-10, 3]),Cord([-10, 4]),Cord([-10, 5]),Cord([-10, 6]),
                Cord([-9, 2]), Cord([-9, 3]), Cord([-9, 4]), Cord([-9, 5]), Cord([-9, 6]),
                Cord([-8, 2]), Cord([-8, 3]),              Cord([-8, 5]), Cord([-8, 6]),
                Cord([-7, 2]), Cord([-7, 3]), Cord([-7, 4]), Cord([-7, 5]), Cord([-7, 6]),
                Cord([-6, 2]), Cord([-6, 3]), Cord([-6, 4]), Cord([-6, 5]), Cord([-6, 6])
            ]
        );

        let cord = Cord([0, 0]);
        let out = cord.moore_neighborhood(3);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(out_vec.len(), out_size.0);
        assert_eq!(out_vec.len(), out_size.1.unwrap());
        #[rustfmt::skip]
        assert_eq!(
            out_vec,
            vec![
                Cord([-3, -3]),Cord([-3, -2]),Cord([-3, -1]),Cord([-3, 0]),Cord([-3, 1]),Cord([-3, 2]),Cord([-3, 3]),
                Cord([-2, -3]),Cord([-2, -2]),Cord([-2, -1]),Cord([-2, 0]),Cord([-2, 1]),Cord([-2, 2]),Cord([-2, 3]),
                Cord([-1, -3]),Cord([-1, -2]),Cord([-1, -1]),Cord([-1, 0]),Cord([-1, 1]),Cord([-1, 2]),Cord([-1, 3]),
                Cord([0, -3]), Cord([0, -2]), Cord([0, -1]),             Cord([0, 1]), Cord([0, 2]), Cord([0, 3]),
                Cord([1, -3]), Cord([1, -2]), Cord([1, -1]), Cord([1, 0]), Cord([1, 1]), Cord([1, 2]), Cord([1, 3]),
                Cord([2, -3]), Cord([2, -2]), Cord([2, -1]), Cord([2, 0]), Cord([2, 1]), Cord([2, 2]), Cord([2, 3]),
                Cord([3, -3]), Cord([3, -2]), Cord([3, -1]), Cord([3, 0]), Cord([3, 1]), Cord([3, 2]), Cord([3, 3])
            ]
        );
    }

    #[test]
    fn neumann_neighborhood_test() {
        let cord = Cord([-8, 4]);
        let out = cord.neumann_neighborhood(1);
        assert_eq!(
            out.collect::<Vec<_>>(),
            vec![Cord([-9, 4]), Cord([-8, 3]), Cord([-8, 5]), Cord([-7, 4])]
        );

        let out: Vec<_> = cord.neumann_neighborhood(2).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                          Cord([-10, 4]),
                             Cord([-9, 3]), Cord([-9, 4]), Cord([-9, 5]),
                Cord([-8, 2]), Cord([-8, 3]),              Cord([-8, 5]), Cord([-8, 6]),
                             Cord([-7, 3]), Cord([-7, 4]), Cord([-7, 5]),
                                          Cord([-6, 4])
            ]
        );

        let cord = Cord([0, 0]);
        let out: Vec<_> = cord.neumann_neighborhood(3).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                                       Cord([-3, 0]),
                                          Cord([-2, -1]),Cord([-2, 0]), Cord([-2, 1]),
                             Cord([-1, -2]),Cord([-1, -1]),Cord([-1, 0]), Cord([-1, 1]), Cord([-1, 2]),
                Cord([0, -3]), Cord([0, -2]), Cord([0, -1]),              Cord([0, 1]),  Cord([0, 2]), Cord([0, 3]),
                             Cord([1, -2]), Cord([1, -1]), Cord([1, 0]),  Cord([1, 1]), Cord([1, 2]),
                                          Cord([2, -1]), Cord([2, 0]),  Cord([2, 1]),
                                                       Cord([3, 0])
            ]
        );
    }

    #[test]
    fn interpolate_test() {
        let cord1 = Cord([498, 4]);
        let cord2 = Cord([498, 6]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(out, vec![Cord([498, 4]), Cord([498, 5]), Cord([498, 6])]);

        let cord1 = Cord([498, 6]);
        let cord2 = Cord([496, 6]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(out, vec![Cord([496, 6]), Cord([497, 6]), Cord([498, 6])]);

        let cord1 = Cord([498, 6]);
        let cord2 = Cord([496, 7]);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(
            out,
            vec![
                Cord([496, 6]),
                Cord([496, 7]),
                Cord([497, 6]),
                Cord([497, 7]),
                Cord([498, 6]),
                Cord([498, 7])
            ]
        );
    }
}
