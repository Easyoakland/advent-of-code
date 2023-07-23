use advent_lib::{
    cord::Cord,
    parse::{nom::parse_from, read_and_leak},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

type Voxel = Cord<isize, 3>;

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
        let out = [out.0, out.1, out.2];
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
        let input = read_and_leak(file_name)?;
        let (_, mut voxels) = parse_input(input)?;
        // Sort so faster to find voxels later.
        voxels.sort();
        let non_blob_neighbors = |voxel: Voxel| {
            voxel
                .neumann_neighborhood(1)
                .filter(|x| voxels.binary_search(x).is_err()) // filter out blob elements from neighbors
                .collect::<Vec<_>>()
                .into_iter()
        };
        Ok(voxels
            .iter()
            .map(|&voxel| non_blob_neighbors(voxel).count())
            .sum())
    }
}

mod part2 {
    use super::*;
    use crate::parse::parse_input;
    use advent_lib::algorithms::flood_fill;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let (_, mut voxels) = parse_input(input)?;
        let mut voxel_exposed = ((0..voxels.len()).map(|_| 0)).collect::<Vec<_>>();
        // Sort so faster to find voxels later.
        voxels.sort();

        let (min_x, max_x) = (
            voxels.iter().min_by(|x, y| x[0].cmp(&y[0])).unwrap()[0],
            voxels.iter().max_by(|x, y| x[0].cmp(&y[0])).unwrap()[0],
        );
        let (min_y, max_y) = (
            voxels.iter().min_by(|x, y| x[1].cmp(&y[1])).unwrap()[1],
            voxels.iter().max_by(|x, y| x[1].cmp(&y[1])).unwrap()[1],
        );
        let (min_z, max_z) = (
            voxels.iter().min_by(|x, y| x[2].cmp(&y[2])).unwrap()[2],
            voxels.iter().max_by(|x, y| x[2].cmp(&y[2])).unwrap()[2],
        );

        // Function to find the neighbors of a given voxel that aren't part of the lava blob.
        let non_blob_neighbors = |voxel: Voxel| {
            voxel
                .neumann_neighborhood(1)
                // Filter out blob elements from neighbors.
                .filter(|node| voxels.binary_search(node).is_err())
                // Filter out nodes far beyond the blob leaving only the blob and immediate exposing air.
                // For example in 1D everything within | would be a valid neighbor: A A | A X A X X A | A A
                .filter(|node| {
                    min_x - 1 <= node[0]
                        && node[0] <= max_x + 1
                        && min_y - 1 <= node[1]
                        && node[1] <= max_y + 1
                        && min_z - 1 <= node[2]
                        && node[2] <= max_z + 1
                })
                .collect::<Vec<_>>()
                .into_iter()
        };
        // Starting outside the blob's bounds.
        let start = [max_x + 1, max_y + 1, max_z + 1].into();
        // Find all air connected to the outside.
        let external_air = flood_fill(start, non_blob_neighbors);

        // Count the faces/neighbors each voxel has that touch external air.
        voxels.iter().enumerate().for_each(|(i, &voxel)| {
            non_blob_neighbors(voxel).for_each(|neighbor| {
                if external_air.contains(&neighbor) {
                    voxel_exposed[i] += 1;
                }
            })
        });
        Ok(voxel_exposed.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::dbc;

    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input = read_and_leak("inputtest.txt")?;
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
