use std::ops::{Add, AddAssign, Sub};
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cord(pub usize, pub usize);

#[allow(dead_code)]
impl Cord {
    pub fn op1(self, f: fn(usize) -> usize) -> Self {
        Cord(f(self.0), f(self.1))
    }
    pub fn op2<T: Into<usize>>(self, rhs: Self, f: fn(usize, usize) -> T) -> Self {
        Cord(f(self.0, rhs.0).into(), f(self.1, rhs.1).into())
    }
    pub fn op2_refutable<T: Into<Option<usize>>>(
        self,
        rhs: Self,
        f: fn(usize, usize) -> T,
    ) -> Option<Self> {
        let x = f(self.0, rhs.0).into()?;
        let y = f(self.1, rhs.1).into()?;
        Some(Cord(x, y))
    }
    pub fn manhattan_distance(self, other: &Self) -> usize {
        let temp = self.op2(*other, usize::abs_diff);
        temp.0 + temp.1
    }

    /// Radius is manhattan distance from center to edge.
    /// Moore neighborhood is a square formed by the extents of the Neumann neighborhood.
    pub fn moore_neighborhood(
        &self,
        radius: usize,
        grid_width: usize,
        grid_height: usize,
    ) -> Vec<Cord> {
        let mut neighbors = Vec::new();
        // Goes from left to right and from top to bottom generating neighbor cords.
        // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1
        for j in 0..2 * radius + 1 {
            for i in 0..2 * radius + 1 {
                let x: i64 = self.0 as i64 - radius as i64 + i as i64;
                let y: i64 = self.1 as i64 - radius as i64 + j as i64;
                // Don't make neighbors with negative cords.
                if x < 0 || y < 0 {
                    continue;
                }
                // If neither is negative can safely convert to unsigned.
                let x: usize = x.try_into().unwrap();
                let y: usize = y.try_into().unwrap();

                // Don't make neighbors with cords beyond the bounds of the board.
                if x > grid_width - 1 || y > grid_height - 1 {
                    continue;
                }

                // Don't add self to neighbor list.
                if x == self.0 && y == self.1 {
                    continue;
                }

                neighbors.push(Cord(x, y));
            }
        }

        neighbors
    }

    /// Radius is manhattan distance of furthest neighbors.
    /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
    pub fn neumann_neighborhood(
        &self,
        radius: usize,
        grid_width: usize,
        grid_height: usize,
    ) -> Vec<Cord> {
        let neighbors = self.moore_neighborhood(radius, grid_width, grid_height);
        neighbors
            .into_iter()
            .filter(|&x| x.manhattan_distance(self) <= radius)
            .collect()
    }
}

impl Add<Self> for Cord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op2(rhs, usize::add)
    }
}

impl AddAssign<Self> for Cord {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Self> for Cord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.op2(rhs, usize::sub)
    }
}

impl From<(usize, usize)> for Cord {
    fn from(value: (usize, usize)) -> Self {
        Cord(value.0, value.1)
    }
}

impl From<Cord> for (usize, usize) {
    fn from(value: Cord) -> Self {
        (value.0, value.1)
    }
}

pub fn offset_to_cord(offset: usize, width: usize) -> Cord {
    let y = offset / width;
    let x = offset - width * y;
    Cord(x, y)
}
