use itertools::{Itertools, Product};
use num_iter::{range, range_inclusive, Range};
use num_traits::{cast, PrimInt, Zero};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cord<T>(pub T, pub T);

pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

// Trait that allows easily adding generic bounds on cord's datatype.
pub trait CordData: PrimInt {}
impl<T: PrimInt> CordData for T {}

struct MooreNeighborhoodIterator<Datatype: CordData> {
    iterator: Product<Range<Datatype>, Range<Datatype>>,
    cord: Cord<Datatype>,
    radius: usize,
}

impl<Datatype: CordData> Iterator for MooreNeighborhoodIterator<Datatype> {
    type Item = Cord<Datatype>;

    fn next(&mut self) -> Option<Self::Item> {
        // Goes from left to right and from top to bottom generating neighbor cords.
        // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1
        while let Some((i, j)) = self.iterator.next() {
            let x = self.cord.0.checked_sub(
                &cast(self.radius).expect("Can't cast the radius to cord's datatype."),
            );
            let y = self.cord.1.checked_sub(
                &cast(self.radius).expect("Can't cast the radius to cord's datatype."),
            );
            let (x, y) = match (x, y) {
                (Some(a), Some(b)) => (a.checked_add(&i), b.checked_add(&j)),
                _ => panic!("\
The moore neighborhood goes below the datatype's limits. \
Brach type_bounded_cord implemented fix for this with artificial limits based on datatype but decided too much of footgun.\
Better to panic than silently ignore neighbors that can't be represented with datatype."),
            };

            let (x,y) = match (x,y) {
                (Some(a), Some(b)) => (a,b),
                _ => panic!("\
The moore neighborhood goes above the datatype's limits.  \
Branch type_bounded_cord implemented fix for this with artificial limits based on datatype but decided too much of footgun.\
Better to panic than silently ignore neighbors that can't be represented with datatype."),
            };

            // Don't add self to neighbor list.
            if x == self.cord.0 && y == self.cord.1 {
                continue;
            }

            return Some(Cord(x, y));
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sidelength = self.radius + self.radius + 1;
        let area = sidelength * sidelength;
        // Area minus the cell the neighborhood is for
        (area - 1, Some(area - 1))
    }
}

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
    pub fn moore_neighborhood(&self, radius: usize) -> impl Iterator<Item = Cord<Datatype>> {
        let dim_max = cast::<usize, Datatype>(radius + radius + 1)
            .expect("Can't convert 2*radius + 1 into cord's datatype. Does it fit?");
        let iterator = range(Zero::zero(), dim_max).cartesian_product(range(Zero::zero(), dim_max));
        MooreNeighborhoodIterator {
            iterator,
            cord: *self,
            radius,
        }
    }

    /// Radius is manhattan distance of furthest neighbors.
    /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
    pub fn neumann_neighborhood(&self, radius: usize) -> impl Iterator<Item = Cord<Datatype>> + '_ {
        let neighbors = self.moore_neighborhood(radius);
        neighbors.filter(move |&x| {
            x.manhattan_distance(&self)
                <= cast(radius).expect("Can't convert radius to cord's datatype.")
        })
    }

    /// Returns a vector of every cordinate with an x or y value between self and other inclusive.
    pub fn interpolate(&self, other: &Self) -> impl Iterator<Item = Cord<Datatype>> {
        range_inclusive(self.0.min(other.0), self.0.max(other.0))
            .cartesian_product(range_inclusive(self.1.min(other.1), self.1.max(other.1)))
            .map_into()
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
    fn moore_neighborhood_test() {
        let cord = Cord(-8, 4);
        let out = cord.moore_neighborhood(1);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(
            out_vec,
            vec![
                Cord(-9, 3),
                Cord(-9, 4),
                Cord(-9, 5),
                Cord(-8, 3),
                Cord(-8, 5),
                Cord(-7, 3),
                Cord(-7, 4),
                Cord(-7, 5)
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
                Cord(-10, 2),Cord(-10, 3),Cord(-10, 4),Cord(-10, 5),Cord(-10, 6),
                Cord(-9, 2), Cord(-9, 3), Cord(-9, 4), Cord(-9, 5), Cord(-9, 6),
                Cord(-8, 2), Cord(-8, 3),              Cord(-8, 5), Cord(-8, 6),
                Cord(-7, 2), Cord(-7, 3), Cord(-7, 4), Cord(-7, 5), Cord(-7, 6),
                Cord(-6, 2), Cord(-6, 3), Cord(-6, 4), Cord(-6, 5), Cord(-6, 6)
            ]
        );

        let cord = Cord(0, 0);
        let out = cord.moore_neighborhood(3);
        let out_size = out.size_hint();
        let out_vec = out.collect::<Vec<_>>();
        assert_eq!(out_vec.len(), out_size.0);
        assert_eq!(out_vec.len(), out_size.1.unwrap());
        #[rustfmt::skip]
        assert_eq!(
            out_vec,
            vec![
                Cord(-3, -3),Cord(-3, -2),Cord(-3, -1),Cord(-3, 0),Cord(-3, 1),Cord(-3, 2),Cord(-3, 3),
                Cord(-2, -3),Cord(-2, -2),Cord(-2, -1),Cord(-2, 0),Cord(-2, 1),Cord(-2, 2),Cord(-2, 3),
                Cord(-1, -3),Cord(-1, -2),Cord(-1, -1),Cord(-1, 0),Cord(-1, 1),Cord(-1, 2),Cord(-1, 3),
                Cord(0, -3), Cord(0, -2), Cord(0, -1),             Cord(0, 1), Cord(0, 2), Cord(0, 3),
                Cord(1, -3), Cord(1, -2), Cord(1, -1), Cord(1, 0), Cord(1, 1), Cord(1, 2), Cord(1, 3),
                Cord(2, -3), Cord(2, -2), Cord(2, -1), Cord(2, 0), Cord(2, 1), Cord(2, 2), Cord(2, 3),
                Cord(3, -3), Cord(3, -2), Cord(3, -1), Cord(3, 0), Cord(3, 1), Cord(3, 2), Cord(3, 3)
            ]
        );
    }

    #[test]
    fn neumann_neighborhood_test() {
        let cord = Cord(-8, 4);
        let out = cord.neumann_neighborhood(1);
        assert_eq!(
            out.collect::<Vec<_>>(),
            vec![Cord(-9, 4), Cord(-8, 3), Cord(-8, 5), Cord(-7, 4)]
        );

        let out: Vec<_> = cord.neumann_neighborhood(2).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                          Cord(-10, 4),
                             Cord(-9, 3), Cord(-9, 4), Cord(-9, 5),
                Cord(-8, 2), Cord(-8, 3),              Cord(-8, 5), Cord(-8, 6),
                             Cord(-7, 3), Cord(-7, 4), Cord(-7, 5),
                                          Cord(-6, 4)
            ]
        );

        let cord = Cord(0, 0);
        let out: Vec<_> = cord.neumann_neighborhood(3).collect();
        #[rustfmt::skip]
        assert_eq!(
            out,
            vec![
                                                       Cord(-3, 0),
                                          Cord(-2, -1),Cord(-2, 0), Cord(-2, 1),
                             Cord(-1, -2),Cord(-1, -1),Cord(-1, 0), Cord(-1, 1), Cord(-1, 2),
                Cord(0, -3), Cord(0, -2), Cord(0, -1),              Cord(0, 1),  Cord(0, 2), Cord(0, 3),
                             Cord(1, -2), Cord(1, -1), Cord(1, 0),  Cord(1, 1), Cord(1, 2),
                                          Cord(2, -1), Cord(2, 0),  Cord(2, 1),
                                                       Cord(3, 0)
            ]
        );
    }

    #[test]
    fn interpolate_test() {
        let cord1 = Cord(498, 4);
        let cord2 = Cord(498, 6);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(out, vec![Cord(498, 4), Cord(498, 5), Cord(498, 6)]);

        let cord1 = Cord(498, 6);
        let cord2 = Cord(496, 6);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
        assert_eq!(out, vec![Cord(496, 6), Cord(497, 6), Cord(498, 6)]);

        let cord1 = Cord(498, 6);
        let cord2 = Cord(496, 7);
        let out: Vec<_> = cord1.interpolate(&cord2).collect();
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
