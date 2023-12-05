use std::error::Error;

mod data {
    use std::fmt::Display;

    pub type SnafuDigit = i8;
    pub type Val = i64;
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct SnafuNum(pub Vec<SnafuDigit>);

    impl From<SnafuNum> for Val {
        fn from(value: SnafuNum) -> Self {
            value
                .0
                .into_iter()
                .enumerate()
                .map(|(i, x)| Val::from(x) * (0..i).map(|_| 5).product::<Val>())
                .sum()
        }
    }

    impl FromIterator<SnafuDigit> for SnafuNum {
        fn from_iter<T: IntoIterator<Item = SnafuDigit>>(iter: T) -> Self {
            SnafuNum(iter.into_iter().collect())
        }
    }

    impl PartialOrd for SnafuNum {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Val::from(self.clone()).partial_cmp(&Val::from(other.clone()))
        }
    }

    impl Ord for SnafuNum {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl Display for SnafuNum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for x in self.0.iter().rev() {
                let c = match x {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => unreachable!("Invalid Snaffu digit"),
                };
                write!(f, "{c}")?;
            }
            Ok(())
        }
    }

    impl From<Val> for SnafuNum {
        fn from(value: Val) -> Self {
            // The smallest place where placing a 2 is larger than the value.
            // Subtract values from this place.
            let mut highest_place = 0usize;
            while (2 * (0..highest_place).map(|_| 5).product::<Val>()) < value {
                highest_place += 1;
            }
            let mut snafu_so_far = SnafuNum(vec![0; highest_place + 1]);
            // Check the new value for a place in a snafu compared to the target.
            let check_place_val = |num: &SnafuNum, place, val: i8| {
                let mut temp = num.clone();
                temp.0[place] = val;
                temp
            };
            let mut num = 'val: {
                for digit_place in (0..=highest_place).rev() {
                    const DIGIT_VAL: [i8; 5] = [-2, -1, 0, 1, 2];
                    snafu_so_far = match DIGIT_VAL.binary_search_by(|x| {
                        Val::from(check_place_val(&snafu_so_far, digit_place, (*x).into()))
                            .cmp(&value)
                    }) {
                        // If equal then don't need to keep calculating digits (leaving rest as 0).
                        Ok(i) => {
                            snafu_so_far.0[digit_place] = DIGIT_VAL[i];
                            break 'val snafu_so_far;
                        }
                        // Use value closer to final answer and continue calculating digits.
                        Err(i) => {
                            let hi = DIGIT_VAL
                                .get(i)
                                .map(|val| check_place_val(&snafu_so_far, digit_place, *val));
                            let lo = (i.checked_sub(1)).map(|i| {
                                let lo_val = DIGIT_VAL[i];
                                check_place_val(&snafu_so_far, digit_place, lo_val)
                            });
                            match (lo, hi) {
                                (None, Some(hi)) => hi,
                                (Some(lo), None) => lo,
                                (Some(lo), Some(hi)) => {
                                    use std::cmp::Ordering::*;
                                    match (value - Val::from(lo.clone()))
                                        .cmp(&(Val::from(hi.clone()) - value))
                                    {
                                        Less | Equal => lo,
                                        Greater => hi,
                                    }
                                }
                                (None, None) => unreachable!("Either low or high exist"),
                            }
                        }
                    };
                }
                snafu_so_far
            }
            .0
            .into_iter()
            .rev()
            .skip_while(|&x| x == 0)
            .collect::<Vec<_>>();
            num.reverse(); // keep little endian
            SnafuNum(num)
        }
    }
}

mod parse {
    use crate::data::SnafuNum;
    use std::io::{BufRead, BufReader};

    pub fn parse_input(input: &str) -> Result<Vec<SnafuNum>, std::io::Error> {
        BufReader::new(input.as_bytes())
            .lines()
            .map(|x| {
                x.map(|x| {
                    {
                        x.chars().flat_map(|x| match x {
                            '=' => Some(-2),
                            '-' => Some(-1),
                            '0' => Some(0),
                            '1' => Some(1),
                            '2' => Some(2),
                            '\n' => None,
                            c => panic!("Invalid input '{c}'"),
                        })
                    }
                    .rev() // make little endian
                    .collect()
                })
            })
            .collect()
    }
}

mod part1 {
    use super::*;
    use crate::{
        data::{SnafuNum, Val},
        parse::parse_input,
    };
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<String, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let nums = parse_input(input)?;
        Ok(SnafuNum::from(nums.into_iter().map(Val::from).sum::<Val>()).to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, "2=-1=0");
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, "2-20=01--0=0=0=2-120");
        Ok(())
    }
}
