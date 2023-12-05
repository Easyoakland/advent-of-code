use ndarray::Array2;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    isize,
};

fn main() -> Result<(), Box<dyn Error>> {
    part1::run()?;
    part2::run()?;
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let mut result = 0;
        let reader = BufReader::new(f);

        let state = parse(reader)?;
        for elem in state.indexed_iter() {
            if check_if_visible(&state, elem) {
                result += 1;
            }
        }
        println!("Part 1 answer: {}", result);
        Ok(())
    }
}

mod part2 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt").unwrap();
        let mut result = 0;
        let reader = BufReader::new(f);

        let state = parse(reader).unwrap();
        for elem in state.indexed_iter() {
            result = result.max(calc_scenic_score(&state, elem));
        }
        println!("Part 2 answer: {}", result);
        Ok(())
    }
}

fn parse(reader: BufReader<File>) -> Result<Array2<u8>, Box<dyn Error>> {
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
    let data: Vec<_> = data
        .iter()
        .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();
    let output = Array2::from_shape_vec((nrow, ncol), data)?;

    Ok(output)
}

fn check_if_visible(state: &Array2<u8>, elem: ((usize, usize), &u8)) -> bool {
    let elem = (elem.0, *elem.1);
    // Handle edge
    if elem.0 .0 == 0
        || elem.0 .0 == state.shape()[0] - 1
        || elem.0 .1 == 0
        || elem.0 .1 == state.shape()[1] - 1
    {
        return true;
    }
    // Handle non-edge zeros
    else if elem.1 == 0 {
        return false;
    }
    // Handle other interior items
    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    'directions: for direction in directions.iter() {
        // Check if tree is visible from given direction from tree.
        let mut distance = 1;
        while let Some(&x) = state.get([
            match isize::saturating_add(isize::try_from(elem.0 .0).unwrap(), direction.0 * distance)
            {
                isize::MIN..=-1 => return true, // If out of range on left edge then direction must be unblocked.
                x => usize::try_from(x).unwrap(),
            },
            match isize::saturating_add(isize::try_from(elem.0 .1).unwrap(), direction.1 * distance)
            {
                isize::MIN..=-1 => return true, // If out of range on right edge then direction must be unblocked.
                x => usize::try_from(x).unwrap(),
            },
        ]) {
            if x >= elem.1 {
                continue 'directions;
            }
            distance += 1;
        }
        return true; // If reached then direction was unblocked from bottom or right.
    }
    // If no direction was clear then they are all blocked and this tree is not visible.
    return false;
}

fn calc_scenic_score(state: &Array2<u8>, elem: ((usize, usize), &u8)) -> u32 {
    let elem = (elem.0, *elem.1);
    let mut out = 1;
    // Handle edge
    if elem.0 .0 == 0
        || elem.0 .0 == state.shape()[0] - 1
        || elem.0 .1 == 0
        || elem.0 .1 == state.shape()[1] - 1
    {
        return 0;
    }
    // Handle non-edge zeros
    else if elem.1 == 0 {
        return 1;
    }
    // Handle other interior items
    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    'directions: for direction in directions.iter() {
        // Check if tree is visible from given direction from tree.
        let mut distance = 1;
        while let Some(&x) = state.get([
            match isize::saturating_add(isize::try_from(elem.0 .0).unwrap(), direction.0 * distance)
            {
                isize::MIN..=-1 => {
                    // If out of range on left edge then multiply by that direction score (-1 because don't include step past edge) and go to next direction.
                    out *= distance - 1;
                    continue 'directions;
                }
                x => usize::try_from(x).unwrap(),
            },
            match isize::saturating_add(isize::try_from(elem.0 .1).unwrap(), direction.1 * distance)
            {
                isize::MIN..=-1 => {
                    // If out of range on top edge then multiply by that direction score (-1 because don't include step past edge) and go to next direction.
                    out *= distance - 1;
                    continue 'directions;
                }
                x => usize::try_from(x).unwrap(),
            },
        ]) {
            if x >= elem.1 {
                out *= distance;
                continue 'directions;
            }
            distance += 1;
        }
        out *= distance - 1; // If reached then direction was unblocked from bottom or right (-1 because don't include step past edge).
    }
    // If no direction was clear then they are all blocked and this tree is not visible.
    out.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = {
            let f = File::open("testinput.txt").unwrap();
            let mut result = 0;
            let reader = BufReader::new(f);

            let state = parse(reader).unwrap();
            for elem in state.indexed_iter() {
                if check_if_visible(&state, elem) {
                    result += 1;
                }
            }
            result
        };
        assert_eq!(answer, 21);
        ()
    }

    #[test]
    fn test_part2() {
        let answer = {
            let f = File::open("testinput.txt").unwrap();
            let mut result = 0;
            let reader = BufReader::new(f);

            let state = parse(reader).unwrap();
            for elem in state.indexed_iter() {
                result = result.max(calc_scenic_score(&state, elem));
            }
            result
        };
        assert_eq!(answer, 8);
        ()
    }
}
