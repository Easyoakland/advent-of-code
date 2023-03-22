use crate::data::Voxel;
#[allow(unused_imports)]
use advent_lib::dbc;
use advent_lib::parse::{parse_from, read_file_static};
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod data {
    use core::panic;

    use advent_lib::cord::abs_diff;
    use derive_more::{Add, Sub};

    use super::*;

    type Datatype = isize;
    #[derive(Clone, Copy, Debug, Default, Add, Sub, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Voxel(Datatype, Datatype, Datatype);
    impl From<(Datatype, Datatype, Datatype)> for Voxel {
        fn from(value: (Datatype, Datatype, Datatype)) -> Self {
            Voxel(value.0, value.1, value.2)
        }
    }

    impl Voxel {
        pub fn op3<T: Into<O> + From<Datatype>, O: Into<Datatype>>(
            self,
            rhs: Self,
            f: fn(T, T) -> O,
        ) -> Self {
            Voxel(
                f(self.0.into(), rhs.0.into()).into(),
                f(self.1.into(), rhs.1.into()).into(),
                f(self.2.into(), rhs.2.into()).into(),
            )
        }

        pub fn manhattan_distance(self, other: &Self) -> Datatype {
            // Try one way and if it doesn't give valid value try other.
            let temp = self.op3(*other, abs_diff::<Datatype>);
            temp.0 + temp.1 + temp.2
        }

        /// Radius is manhattan distance from center to edge.
        /// Moore neighborhood is a square formed by the extents of the Neumann neighborhood.
        pub fn moore_neighborhood(&self, radius: isize) -> impl Iterator<Item = Voxel> + '_ {
            let dim_max = radius + radius + 1;
            (0..dim_max)
                .cartesian_product(0..dim_max)
                .cartesian_product(0..dim_max)
                .filter_map(move |((i, j), k)| {
                    // Goes from left to right and from top to bottom generating neighbor cords.
                    // Each radius increases number of cells in each dimension by 2 (each extent direction by 1) starting with 1 cell at radius = 1
                    {
                        let x = self.0.checked_sub(radius);
                        let y = self.1.checked_sub(radius);
                        let z = self.2.checked_sub(radius);
                        let (x, y, z) = match (x, y, z) {
                            (Some(a), Some(b), Some(c)) => (a + i, b + j, c + k),
                            _ => panic!("datatype can't hold neighborhood"),
                        };

                        // Don't add self to neighbor list.
                        if x == self.0 && y == self.1 && z == self.2 {
                            return None;
                        }

                        Some(Voxel(x, y, z))
                    }
                })
        }

        /// Radius is manhattan distance of furthest neighbors.
        /// Neumann neighborhood is all cells a manhattan distance of the radius or smaller.
        pub fn neumann_neighborhood(&self, radius: isize) -> impl Iterator<Item = Voxel> + '_ {
            let neighbors = self.moore_neighborhood(radius);
            neighbors.filter(move |x| x.manhattan_distance(&self) <= radius)
        }
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::{terminated, tuple},
        IResult,
    };

    use super::*;
    fn voxel(input: &str) -> IResult<&str, Voxel> {
        let (input, out) = tuple((
            (terminated(parse_from(digit1), tag(","))),
            (terminated(parse_from(digit1), tag(","))),
            parse_from(digit1),
        ))(input)?;
        Ok((input, out.into()))
    }
    pub fn parse_input(input: &str) -> IResult<&str, Vec<Voxel>> {
        all_consuming(terminated(separated_list1(line_ending, voxel), line_ending))(input)
    }
}

mod part1 {
    use super::*;
    use crate::parse::parse_input;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_file_static(file_name)?;
        let (_, mut voxels) = parse_input(input)?;
        let mut voxel_exposed = vec![6usize; voxels.len()];
        voxels.sort();
        // For every voxel
        for (i, voxel) in voxels.iter().enumerate() {
            eprintln!("{:?}", &voxel);
            // It is exposed on, 6 - the number of neighboring voxels that are in the blob, faces.
            for neighbor in voxel.neumann_neighborhood(1) {
                if let Ok(_idx) = voxels.binary_search(&neighbor) {
                    voxel_exposed[i] -= 1;
                    eprintln!("{:?}", voxels[_idx]);
                }
            }
            eprintln!("{:?}", voxel_exposed[i]);
        }
        Ok(voxel_exposed.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input = read_file_static("inputtest.txt")?;
        let (_, parsed) = parse::parse_input(input).unwrap();
        dbc!(parsed);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 64);
        Ok(())
    }

    #[test]
    fn test_part1_2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest2.txt")?, 18 + 18 - 4);
        Ok(())
    }

    #[test]
    fn test_part1_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest3.txt")?, 10);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert!(dbc!(part1::run("input.txt")?) < 12306);
        assert!(part1::run("input.txt")? < 3548);
        assert_eq!(part1::run("input.txt")?, 3542);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    // assert_eq!(part2::run("inputtest.txt")?, 1514285714288);
    // Ok(())
    // }
    //
    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    // assert_eq!(part2::run("input.txt")?, 1560919540245);
    // Ok(())
    // }
}
