use ndarray::Array2;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(file_name: &str) -> Result<u32, Box<dyn Error>> {
        let f = File::open(file_name)?;
        let mut result = 0;
        let reader = BufReader::new(f);

        let state = parse(reader)?;
        for elem in state.indexed_iter() {
            // print!("{:?}", elem);
        }
        Ok(result)
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
        .map(|c| match c {
            'a'..='z' => (*c as u8) - ('a' as u8),
            'S' => 0,
            'E' => 'z' as u8 - 'a' as u8,
            _ => panic!("Other values invalid."),
        })
        .collect();
    let output = Array2::from_shape_vec((nrow, ncol), data)?;

    Ok(output)
}
