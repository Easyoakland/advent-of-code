use crate::data::Voxel;
#[allow(unused_imports)]
use advent_lib::{
    algorithms::astar,
    dbc,
    parse::{parse_from, read_file_static},
};
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod data {
    use advent_lib::cord::abs_diff;
    use derive_more::{Add, Sub};

    use super::*;

    type Datatype = isize;
    #[derive(Clone, Copy, Debug, Default, Add, Sub, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Voxel(pub Datatype, pub Datatype, pub Datatype);
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
        pub fn moore_neighborhood(
            &self,
            radius: isize,
        ) -> impl Iterator<Item = Voxel> + Clone + '_ {
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
        pub fn neumann_neighborhood(
            &self,
            radius: isize,
        ) -> impl Iterator<Item = Voxel> + Clone + '_ {
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
        let mut voxel_exposed = vec![0usize; voxels.len()];
        // Sort so faster to find voxels later.
        voxels.sort();
        // Don't mutate further.
        let voxels = voxels;
        let non_blob_neighbors = |voxel: Voxel| {
            voxel
                .neumann_neighborhood(1)
                .filter(|x| voxels.binary_search(x).is_err()) // filter out blob elements from neighbors
                .collect::<Vec<_>>()
                .into_iter()
        };
        // For every voxel.
        for (i, voxel) in voxels.iter().enumerate() {
            // It is exposed on 6 - the number of neighboring voxels that are in the blob faces.
            for _neighbor in non_blob_neighbors(*voxel) {
                // If that neighbor actually exists in the blob.
                voxel_exposed[i] += 1;
            }
        }
        Ok(voxel_exposed.iter().sum())
    }
}

mod part2 {
    use super::*;
    use crate::parse::parse_input;
    use rayon::prelude::{ParallelBridge, ParallelIterator};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_file_static(file_name)?;
        let (_, mut voxels) = parse_input(input)?;
        let voxel_exposed = ((0..voxels.len()).map(|_| AtomicUsize::new(0))).collect::<Vec<_>>();
        // Sort so faster to find voxels later.
        voxels.sort();
        // Don't mutate further.
        let voxels = voxels;

        let (min_x, max_x) = (
            voxels.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
            voxels.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
        );
        let (min_y, max_y) = (
            voxels.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
            voxels.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1,
        );
        let (min_z, max_z) = (
            voxels.iter().min_by(|x, y| x.2.cmp(&y.2)).unwrap().2,
            voxels.iter().max_by(|x, y| x.2.cmp(&y.2)).unwrap().2,
        );

        println!("There are {} voxels", voxels.len());
        // TODO make correct
        // Don't weight neighbors
        let non_blob_neighbors = |voxel: Voxel| {
            voxel
                .neumann_neighborhood(1)
                // Filter out blob elements from neighbors.
                .filter(|node| voxels.binary_search(node).is_err())
                // Filter out nodes far beyond the blob leaving only the blob and immediate exposing air.
                // For example in 1D everything within | would be a valid neighbor: A A | A X A X X A | A A
                .filter(|node| {
                    min_x - 1 <= node.0
                        && node.0 <= max_x + 1
                        && min_y - 1 <= node.1
                        && node.1 <= max_y + 1
                        && min_z - 1 <= node.2
                        && node.2 <= max_z + 1
                })
                .collect::<Vec<_>>()
                .into_iter()
        };
        // For every voxel.
        voxels
            .iter()
            .enumerate()
            .par_bridge() // This takes a long time (2048 astar searches (1 per voxel) * ~19^3 voxels = ~14 mil distance checks). The parallelism helps somewhat.
            .for_each(|(i, voxel)| {
                if i % 100 == 0 {
                    println!("Checked first {i} voxels");
                }
                // For each non-blob face/neighbor.
                non_blob_neighbors(*voxel).for_each(|neighbor| {
                    // If that neighbor can be reached from outside the blob without crossing through the blob (not internal air pocket).
                    if astar::<Voxel, isize, _>(
                        (max_x + 1, max_y + 1, max_z + 1).into(),
                        neighbor,
                        non_blob_neighbors,
                        |_| 0, // 0 is faster than -> `|neighbor: Voxel| neighbor.manhattan_distance(voxel)`. I guess the heuristic is bad?
                        |_, _| 1,
                    )
                    .is_some()
                    {
                        voxel_exposed[i].fetch_add(1, Ordering::Relaxed);
                    }
                })
            });
        Ok(voxel_exposed.into_iter().map(|x| x.into_inner()).sum())
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

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 58);
        Ok(())
    }

    #[test]
    fn part2_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("input.txt")?, 2080);
        Ok(())
    }
}
