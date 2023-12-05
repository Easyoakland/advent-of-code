use itertools::Itertools;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Div, Sub},
    rc::Rc, cell::RefCell
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);

impl Pos {
    fn op1(self, f: fn(isize) -> isize) -> Self {
        Pos(f(self.0), f(self.1))
    }
    fn op2<T: Into<isize>>(self, rhs: Self, f: fn(isize, isize) -> T) -> Self {
        Pos(f(self.0, rhs.0).into(), f(self.1, rhs.1).into())
    }
    fn op2_refutable<T: Into<Option<isize>>>(
        self,
        rhs: Self,
        f: fn(isize, isize) -> T,
    ) -> Option<Self> {
        let x = f(self.0, rhs.0).into()?;
        let y = f(self.1, rhs.1).into()?;
        Some(Pos(x, y))
    }
    fn manhattan_distance(self, other: &Self) -> u32 {
        let temp = self.op2(*other, isize::sub).op1(isize::abs);
        (temp.0 + temp.1).try_into().unwrap()
    }
}

impl Add<Self> for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.op2(rhs, isize::add)
    }
}

impl AddAssign<Self> for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Self> for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.op2(rhs, isize::sub)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {}", part1::run("input.txt")?);
    println!("Part 2 answer: {}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {

    use super::*;
    pub fn run(filename: &str) -> Result<u32, Box<dyn Error>> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);
        let mut hpos = Pos(0, 0);
        let mut tpos = Pos(0, 0);
        let mut visited_positions = HashSet::new();

        for line in reader.lines() {
            let line = line?;

            for _ in 0..u32::from_str_radix(&line[2..], 10).unwrap() {
                hpos = match &line[0..1] {
                    "U" => hpos + Pos(0, 1),
                    "D" => hpos + Pos(0, -1),
                    "L" => hpos + Pos(-1, 0),
                    "R" => hpos + Pos(1, 0),
                    _ => unimplemented!(),
                };

                update_tail(&mut tpos, &hpos);
                visited_positions.insert(tpos);
            }
        }

        Ok(visited_positions.len().try_into().unwrap())
    }
}

mod part2 {
    use super::*;
    pub fn run(filename: &str) -> Result<u32, Box<dyn Error>> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);

        let mut knots = vec![Pos::default(); 10];
        let mut visited_positions = HashSet::new();

        for line in reader.lines() {
            let line = line?;

            // As many times as indicated
            for _ in 0..u32::from_str_radix(&line[2..], 10).unwrap() {
                // move the rope head
                knots[0] = match &line[0..1] {
                    "U" => knots[0] + Pos(0, 1),
                    "D" => knots[0] + Pos(0, -1),
                    "L" => knots[0] + Pos(-1, 0),
                    "R" => knots[0] + Pos(1, 0),
                    _ => unimplemented!(),
                };
                // update tails
                for (head, tail) in knots.iter_mut().map(RefCell::new).map(Rc::new).tuple_windows() {
                    update_tail(&mut tail.borrow_mut(), &*head.borrow());
                }

                // and then add the location of the final tail to visited positions.
                visited_positions.insert(*knots.last().unwrap());
            }
        }

        Ok(visited_positions.len().try_into().unwrap())
    }
}

fn update_tail(tpos: &mut Pos, hpos: &Pos) {
    // Row/Column lagging
    if hpos.0 == tpos.0 && hpos.1.abs_diff(tpos.1) == 2 {
        let dif = hpos.1 - tpos.1;
        tpos.1 += dif.div(dif.abs())
    } else if hpos.1 == tpos.1 && hpos.0.abs_diff(tpos.0) == 2 {
        let dif = hpos.0 - tpos.0;
        tpos.0 += dif.div(dif.abs())
    }
    // Diagonal lagging
    // ex (2,1) - (1,0) = (1,0)
    else if hpos.manhattan_distance(tpos) >= 3 {
        let dif = *hpos - *tpos;
        // Manhattan distance is 3 when diagonal needs updating.
            if let Some(x) = dif.op2_refutable(dif.op1(isize::abs), isize::checked_div) {
                *tpos += x;
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::run("inputtest.txt").unwrap(), 13);
    }

    #[test]
    fn manhattan_distance_test() {
        let a = Pos(1, 0);
        let b = Pos(2, 1);
        assert_eq!(a.manhattan_distance(&b), 2);
        let a = Pos(1, 0);
        let b = Pos(2, 2);
        assert_eq!(a.manhattan_distance(&b), 3);
        let a = Pos(2, -3);
        let b = Pos(0, 0);
        assert_eq!(a.manhattan_distance(&b), 5);
    }

    #[test]
    fn test_part1_correct_ans() {
        assert_eq!(part1::run("input.txt").unwrap(), 6243)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::run("inputtest.txt").unwrap(), 1);
        assert_eq!(part2::run("inputtest2.txt").unwrap(), 36);
    }

    #[test]
    fn test_part2_correct_ans() {
        assert_eq!(part2::run("input.txt").unwrap(), 2630);
    }
}
