use data::{Dir, Map, Pos};
use enum_iterator::All;
use std::{collections::BTreeMap, error::Error, iter::Cycle};

mod data {
    use advent_lib::cord::NDCord;
    use enum_iterator::Sequence;
    use std::collections::BTreeSet;

    pub type Val = isize;
    pub type Pos = NDCord<Val, 2>;
    pub type Map = BTreeSet<Pos>;

    #[derive(Clone, Copy, Debug, Sequence)]
    pub enum Dir {
        North,
        South,
        West,
        East,
    }

    impl Dir {
        pub fn move_pos(&self, pos: Pos) -> Pos {
            match self {
                Dir::North => pos + [0, -1].into(),
                Dir::South => pos + [0, 1].into(),
                Dir::East => pos + [1, 0].into(),
                Dir::West => pos + [-1, 0].into(),
            }
        }

        /// Check if elf in this direction.
        pub fn elf_in_dir(&self, elf: Pos, map: &Map) -> bool {
            let top_left = elf + [-1, -1].into();
            let top_right = elf + [1, -1].into();
            let bottom_left = elf + [-1, 1].into();
            let bottom_right = elf + [1, 1].into();
            match self {
                Dir::North => (top_left)
                    .interpolate(&top_right)
                    .any(|x| map.get(&x).is_some()),
                Dir::South => (bottom_left)
                    .interpolate(&bottom_right)
                    .any(|x| map.get(&x).is_some()),
                Dir::East => (top_right)
                    .interpolate(&bottom_right)
                    .any(|x| map.get(&x).is_some()),
                Dir::West => (top_left)
                    .interpolate(&bottom_left)
                    .any(|x| map.get(&x).is_some()),
            }
        }
    }
}

mod parse {
    use crate::data::{Map, Pos};
    use advent_lib::{
        cord::NDCord,
        parse::yap::{all_consuming, line_ending, AllConsuming},
    };
    use std::{collections::BTreeSet, fs::File, io::Write, path::Path};
    use yap::{IntoTokens, Tokens};

    pub fn initial_map(input: &mut impl Tokens<Item = char>) -> BTreeSet<Pos> {
        let mut out = BTreeSet::new();
        let mut cursor = Pos::default();
        loop {
            loop {
                if input.token('#') {
                    out.insert(cursor);
                    cursor[0] += 1;
                } else if input.token('.') {
                    cursor[0] += 1;
                } else {
                    break;
                }
            }
            if let None = line_ending(input) {
                break;
            }
            cursor[0] = 0;
            cursor[1] += 1;
        }
        out
    }
    pub fn parse_input(input: &str) -> Result<Map, AllConsuming<String>> {
        all_consuming(&mut input.into_tokens(), initial_map)
    }

    #[allow(dead_code)]
    pub fn log_state(file: &Path, map: &Map) {
        let mut file = File::create(file).unwrap();
        let mut extents = NDCord::extents_iter(map.iter().copied()).unwrap();
        extents.0.swap(0, 1);
        extents.1.swap(0, 1);
        extents
            .0
            .interpolate(&extents.1)
            .map(|mut x| {
                x.swap(0, 1);
                if map.get(&x).is_none() {
                    write!(file, ".").unwrap();
                } else {
                    write!(file, "#").unwrap();
                }
                if x.0[0] == extents.1[1] {
                    writeln!(file).unwrap();
                }
            })
            .for_each(drop);
    }
}

fn do_round(map: &mut Map, global_next_dir: &mut Cycle<All<Dir>>) {
    let transitions: BTreeMap<Pos, Pos> = map
        .iter()
        .map(|&elf| {
            // Check all directions.
            let should_move = elf.moore_neighborhood(1).any(|pos| map.get(&pos).is_some());

            // Don't move if no nearby elf.
            if !should_move {
                return (elf, elf);
            }

            // Move in the first direction that is clear.
            let mut next_dir = global_next_dir.clone();
            for _ in 0..4 {
                let dir = next_dir.next().unwrap();
                let res = dir.elf_in_dir(elf, &map);
                if !res {
                    return (elf, dir.move_pos(elf));
                }
            }

            // Don't move if no direction is clear.
            (elf, elf)
        })
        .collect();
    // Get all moves
    let all_moves: Vec<_> = transitions.iter().map(|x| x.1).collect();
    // Only move an elf to its destination if that destination has only one source
    *map = map
        .iter()
        .map(|&x| {
            let to = transitions[&x];
            if !(all_moves.iter().filter(|&&&x| x == to).count() > 1) {
                to
            } else {
                x
            }
        })
        .collect();
    // Increment global next direction.
    global_next_dir.next();
}

mod part1 {
    use super::*;
    use crate::{
        data::{Dir, Val},
        parse::parse_input,
    };
    use advent_lib::{cord::NDCord, parse::read_and_leak};

    const ROUND_END: usize = 10;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let mut map = parse_input(input)?;
        let mut global_next_dir = enum_iterator::all::<Dir>().cycle();
        (0..ROUND_END).for_each(|_| do_round(&mut map, &mut global_next_dir));
        let extents = NDCord::extents_iter(map.iter().copied()).expect("nonempty");
        Ok(extents
            .0
            .interpolate(&extents.1)
            .filter(|x| map.get(x).is_none())
            .count()
            .try_into()
            .unwrap())
    }
}

mod part2 {
    use super::*;
    use crate::{
        data::{Dir, Val},
        parse::parse_input,
    };
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let map = parse_input(input)?;
        let mut global_next_dir = enum_iterator::all::<Dir>().cycle();
        let mut map_before = map.clone();
        let mut map_after = map;
        let mut countdown = 4; // try all 4 dirs before giving up
        let mut round = 1;
        while countdown != 0 {
            do_round(&mut map_after, &mut global_next_dir);
            if map_before == map_after {
                countdown -= 1;
            } else {
                round += 4 - countdown + 1;
                countdown = 4;
            }
            map_before = map_after.clone();
        }
        Ok(round)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 110);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 4146);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 20);
        Ok(())
    }

    #[test]
    fn part2_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("input.txt")?, 957);
        Ok(())
    }
}
