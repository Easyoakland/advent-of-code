use cord::{offset_to_cord, Cord};
use ndarray::Array2;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::Sub,
};
mod cord;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let (start, end, state) = parse(file_name)?;
        Ok(dft_unweighted_astar(start, end, state).unwrap())
    }
}

fn parse(file_name: &str) -> Result<(Cord, Cord, Array2<u8>), Box<dyn Error>> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);

    let mut data = Vec::new();
    let mut nrow = 0;
    let mut ncol = 0;
    for line in reader.lines() {
        let line = line?;
        nrow += 1;
        for c in line.chars() {
            data.push(c);
            if nrow == 1 {
                ncol += 1;
            }
        }
    }
    let mut start_offset: usize = 0;
    let mut end_offset: usize = 0;
    let data: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(o, c)| match c {
            'a'..='z' => (*c as u8) - (b'a'),
            'S' => {
                start_offset = o;
                0
            }
            'E' => {
                end_offset = o;
                b'z' - b'a'
            }
            _ => panic!("Other values invalid."),
        })
        .collect();
    let output = Array2::from_shape_vec((nrow, ncol), data)?;

    Ok((
        offset_to_cord(start_offset, ncol),
        offset_to_cord(end_offset, ncol),
        output,
    ))
}

fn dft_unweighted_astar(start: Cord, end: Cord, input: Array2<u8>) -> Option<usize> {
    // Defining potential
    let potential = |node: Cord| node.manhattan_distance(&end);

    // Dijkstra
    let mut boundary_nodes = HashSet::from([start]);
    let mut distances = HashMap::from([(start, 0usize)]);

    while !boundary_nodes.is_empty() {
        // Remove closest node defined by distance + potential of node.
        let cur_node = *boundary_nodes
            .iter()
            .min_by_key(|&&x| distances[&x] + potential(x))
            .unwrap();
        boundary_nodes.remove(&cur_node);

        // If the end is reached return the distance to the end.
        if cur_node == end {
            return Some(distances[&end]);
        }

        // Increase scope of neighbors to neighbors of `cur_node`
        for neighbor in cur_node.neumann_neighborhood(1, input.dim().1, input.dim().0) {
            // NEW for advent of code
            // Confirm that neighbor is valid (height difference is <= 1 greater before continuing.
            // Index is (row,column) not (x,y) or (column, row). See above for loop and below line.
            if input[[neighbor.1, neighbor.0]] > input[[cur_node.1, cur_node.0]]
                && input[[neighbor.1, neighbor.0]].sub(input[[cur_node.1, cur_node.0]]) > 1
            {
                continue;
            }

            // +1 because all edge lengths are 1 on unweighted 2d grid.
            // On weighted graph would be the weight of the edge between cur_node and neighbor.
            let proposed_distance = distances[&cur_node] + 1;

            // If don't already have a distance for the specified node or if the new distance is shorter
            // replace/insert the new distance for the neighbor
            if !distances.contains_key(&neighbor) || proposed_distance < distances[&neighbor] {
                distances.insert(neighbor, proposed_distance);
                boundary_nodes.insert(neighbor);
            }
        }
    }

    // If not found after full search then no distance.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 31);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 534);
        Ok(())
    }

    #[test]
    fn test_test_parse() -> Result<(), Box<dyn Error>> {
        let out = parse("inputtest.txt")?;
        assert_eq!(out.0, Cord(0, 0));
        assert_eq!(out.1, Cord(5, 2));
        println!(
            "{}",
            out.2
                .indexed_iter()
                .map(|((_, y), v)| {
                    if y != out.2.dim().1 - 1 {
                        String::from((v + 'a' as u8) as char)
                    } else {
                        format!("{}{}", (v + 'a' as u8) as char, '\n')
                    }
                })
                .collect::<String>()
        );
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), Box<dyn Error>> {
        let out = parse("input.txt")?;
        assert_eq!(out.0, Cord(0, 20));
        assert_eq!(out.1, Cord(138, 20));
        println!(
            "{}",
            out.2
                .indexed_iter()
                .map(|((_, y), v)| {
                    if y != out.2.dim().1 - 1 {
                        String::from((v + 'a' as u8) as char)
                    } else {
                        format!("{}{}", (v + 'a' as u8) as char, '\n')
                    }
                })
                .collect::<String>()
        );
        Ok(())
    }
}
