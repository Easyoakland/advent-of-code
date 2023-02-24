use ndarray::Array2;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Sub},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(file_name: &str) -> Result<u32, Box<dyn Error>> {
        let mut result = 0;

        let (start, end, state) = parse(file_name)?;
        for elem in state.indexed_iter() {
            // print!("{:?}", elem);
        }
        Ok(result)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cord(usize, usize);

impl Cord {
    fn op1(self, f: fn(usize) -> usize) -> Self {
        Cord(f(self.0), f(self.1))
    }
    fn op2<T: Into<usize>>(self, rhs: Self, f: fn(usize, usize) -> T) -> Self {
        Cord(f(self.0, rhs.0).into(), f(self.1, rhs.1).into())
    }
    fn op2_refutable<T: Into<Option<usize>>>(
        self,
        rhs: Self,
        f: fn(usize, usize) -> T,
    ) -> Option<Self> {
        let x = f(self.0, rhs.0).into()?;
        let y = f(self.1, rhs.1).into()?;
        Some(Cord(x, y))
    }
    fn manhattan_distance(self, other: &Self) -> u32 {
        let temp = self.op2(*other, usize::sub);
        (temp.0 + temp.1).try_into().unwrap()
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

fn offset_to_cord(offset: usize, width: usize) -> Cord {
    let y = offset / width;
    let x = offset - width * y;
    Cord(x, y)
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
            'a'..='z' => (*c as u8) - ('a' as u8),
            'S' => {
                start_offset = o;
                0
            }
            'E' => {
                end_offset = o;
                'z' as u8 - 'a' as u8
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

fn dfs(start: Cord, end: Cord, input: Array2<u8>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 31);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), Box<dyn Error>> {
        let out = parse("input.txt")?;
        assert_eq!(out.0, Cord(0, 20));
        assert_eq!(out.1, Cord(138, 20));
        Ok(())
    }
}
