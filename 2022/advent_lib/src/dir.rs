use crate::cord::NDCord;
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::{
    cmp::PartialEq,
    ops::{MulAssign, Neg},
};

pub type Velocity<T> = NDCord<T, 2>;

// Direction where the discriminant represents the number of clockwise turns from the right.
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    enum_iterator::Sequence,
)]
pub enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    pub fn rotate(self, rotation: &Rotation) -> Self {
        match rotation {
            Rotation::Right => Self::from_i8((self.to_i8().unwrap() + 1).rem_euclid(4)).unwrap(),
            Rotation::Left => Self::from_i8((self.to_i8().unwrap() - 1).rem_euclid(4)).unwrap(),
        }
    }

    /// Converts the direction to velocity with the assumption that Right is increasing x and Down is increasing y (and vice versa).
    pub fn to_velocity<T>(self) -> Velocity<T>
    where
        T: Zero + One + Neg<Output = T>,
    {
        match self {
            Dir::Right => [T::one(), T::zero()],
            Dir::Down => [T::zero(), T::one()],
            Dir::Left => [-T::one(), T::zero()],
            Dir::Up => [T::zero(), -T::one()],
        }
        .into()
    }

    /// Converts from a velocity to direction with the assumption that Right is increasing x and Down is increasing y (and vice versa).
    /// Additionally assumes that velocity has a magnitude of `1` or `0` in each dimension.
    pub fn from_velocity<T: Zero + One + Neg<Output = T> + PartialEq>(
        velocity: Velocity<T>,
    ) -> Self {
        match velocity.0 {
            x if x == [T::one(), T::zero()] => Dir::Right,
            x if x == [T::zero(), T::one()] => Dir::Down,
            x if x == [-T::one(), T::zero()] => Dir::Left,
            x if x == [T::zero(), -T::one()] => Dir::Up,
            _ => panic!("Invalid velocity"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Rotation {
    /// Counter-clockwise
    Left,
    /// Clockwise
    Right,
}

pub fn rotate<T: MulAssign + One + Neg<Output = T>>(dir: &mut Velocity<T>, rotation: &Rotation) {
    dir.swap(0, 1);
    match rotation {
        // 1,0 -> 0,1 -> -1,0 -> 0,-1
        Rotation::Right => dir[0] *= -T::one(),
        // 1,0 -> 0,-1 -> -1,0 -> 0,1
        Rotation::Left => dir[1] *= -T::one(),
    }
}
