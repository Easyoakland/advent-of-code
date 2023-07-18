#[allow(unused_imports)]
use advent_lib::dbc;
use advent_lib::{cord::Cord, parse::read_file_static};
use data::{Action, Rock};
use std::{
    collections::{BTreeSet, HashSet},
    error::Error,
};

type CordType = usize;
const LOGGING: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod data {
    use super::*;
    use std::fmt::Display;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Action {
        Left,
        Right,
    }

    pub const TYPES_OF_ROCK: usize = 5;
    #[derive(Debug, Clone, Copy, Hash)]
    pub enum RockKind {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
    }

    impl From<usize> for RockKind {
        fn from(value: usize) -> Self {
            match value {
                1 => Self::One,
                2 => Self::Two,
                3 => Self::Three,
                4 => Self::Four,
                5 => Self::Five,
                x => panic!("Can't convert from {x} into RockKind"),
            }
        }
    }

    impl Display for RockKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", *self as u8)
        }
    }

    #[derive(Clone, Hash)]
    pub struct Rock {
        pub kind: RockKind,
        pub cord: Cord<CordType, 2>, // bottom left position
    }

    impl Rock {
        pub fn hitbox(&self) -> Box<dyn Iterator<Item = Cord<CordType, 2>>> {
            match &self.kind {
                RockKind::One => Box::new(self.cord.interpolate(&(self.cord + Cord([3, 0])))),
                RockKind::Two => Box::new(
                    std::iter::once(self.cord + Cord([1, 0]))
                        .chain(
                            (self.cord + Cord([0, 1]))
                                .interpolate(&((self.cord + Cord([0, 1])) + Cord([2, 0]))),
                        )
                        .chain(std::iter::once(self.cord + Cord([1, 2]))),
                ),
                RockKind::Three => Box::new(
                    self.cord
                        .interpolate(&(self.cord + Cord([2, 0])))
                        .chain((self.cord + Cord([2, 1])).interpolate(&(self.cord + Cord([2, 2])))),
                ),
                RockKind::Four => Box::new(self.cord.interpolate(&(self.cord + Cord([0, 3])))),
                RockKind::Five => Box::new(self.cord.interpolate(&(self.cord + Cord([1, 1])))),
            }
        }

        /// Returns true if hitbox intersects occupied cells
        fn hitbox_check(&self, occupied_cells: &BTreeSet<Cord<CordType, 2>>) -> bool {
            self.hitbox().any(|cord| occupied_cells.contains(&cord))
        }
        pub fn fall(&mut self, occupied_cells: &BTreeSet<Cord<CordType, 2>>) -> bool {
            let proposed_next = {
                let mut out = self.clone();
                out.cord -= Cord([0, 1]);
                out
            };
            if !proposed_next.hitbox_check(occupied_cells) {
                self.cord = proposed_next.cord;
                true
            } else {
                false
            }
        }
    }

    impl Display for Rock {
        /*
        ####

        .#.
        ###
        .#.

        ..#
        ..#
        ###

        #
        #
        #
        #

        ##
        ##
         */
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in (0..=CHAMBER_WIDTH).rev() {
                write!(f, "|")?;
                for x in 1..=CHAMBER_WIDTH {
                    if self
                        .hitbox()
                        .collect::<HashSet<_>>()
                        .contains(&Cord([x, y]))
                    {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f, "|")?;
            }
            match self.kind {
                RockKind::One => write!(f, "####"),
                RockKind::Two => write!(f, ".#.\n###\n.#."),
                RockKind::Three => write!(f, "..#\n..#\n###"),
                RockKind::Four => write!(f, "#\n#\n#\n#"),
                RockKind::Five => write!(f, "##\n##"),
            }
        }
    }

    #[derive(Clone)]
    pub struct Grid {
        pub occupied_cells: BTreeSet<Cord<CordType, 2>>,
        pub highlight_cells: HashSet<Cord<CordType, 2>>,
        pub highest: usize,
    }

    impl Display for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in (({
                if self.highest >= 20 {
                    self.highest - 20
                } else {
                    0
                }
            })..=self.highest)
                .rev()
            {
                write!(f, "|")?;
                for x in 1..=CHAMBER_WIDTH {
                    if self.highlight_cells.contains(&Cord([x, y])) {
                        write!(f, "@")?;
                    } else if self.occupied_cells.contains(&Cord([x, y])) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f, "|{y}")?;
            }
            Ok(())
        }
    }
}

const CHAMBER_WIDTH: usize = 7;

fn drop_rock(
    rock: &mut Rock,
    occupied_cells: &mut BTreeSet<Cord<CordType, 2>>,
    actions: &mut impl Iterator<Item = Action>,
) {
    while let Some(action) = actions.next() {
        match action {
            Action::Left => {
                if LOGGING {
                    print!("<");
                }

                if (2..=CHAMBER_WIDTH).contains(&rock.cord[0])
                    && !rock
                        .hitbox()
                        .any(|cord| occupied_cells.contains(&(cord - Cord([1, 0]))))
                {
                    rock.cord -= Cord([1, 0])
                }
            }
            Action::Right => {
                if LOGGING {
                    print!(">")
                };

                if (1..CHAMBER_WIDTH).contains(&(rock.hitbox().fold(0, |acc, c| acc.max(c[0]))))
                    && !rock
                        .hitbox()
                        .any(|cord| occupied_cells.contains(&(cord + Cord([1, 0]))))
                {
                    rock.cord += Cord([1, 0])
                }
            }
        }

        if !rock.fall(occupied_cells) {
            break;
        }
    }

    //DEBUG
    if LOGGING {
        println!();
    }

    occupied_cells.extend(rock.hitbox());
}

mod part1 {
    use super::*;
    use crate::data::{Grid, TYPES_OF_ROCK};

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_file_static(file_name)?;
        let actions = parse::parse_input(input);
        let mut actions = actions.into_iter().cycle();
        let mut grid = Grid {
            occupied_cells: Cord([0, 0]).interpolate(&Cord([8, 0])).collect(),
            highest: 0,
            highlight_cells: HashSet::new(),
        };
        for i in 0..2022 {
            // Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
            let mut rock = Rock {
                kind: ((i % TYPES_OF_ROCK) + 1).into(),
                cord: Cord([3, grid.highest + 4]),
            };

            if LOGGING {
                println!(
                    "New Rock {}\n{}",
                    i + 1,
                    data::Grid {
                        occupied_cells: grid.occupied_cells.clone(),
                        highest: rock.cord[1] + 5,
                        highlight_cells: rock.hitbox().collect(),
                    }
                );
            }

            // Repeatedly apply jet streams and gravity to move rock until it hits something.
            drop_rock(&mut rock, &mut grid.occupied_cells, &mut actions);

            // Update highest to highest including the newly placed rock.
            grid.highest = grid
                .highest
                .max(rock.hitbox().fold(0, |acc, cord| acc.max(cord[1])));

            if LOGGING {
                println!(
                    "New Rock {} Placed at Height {}\n{}",
                    i + 1,
                    grid.highest,
                    {
                        let mut out = grid.clone();
                        out.highlight_cells.extend(rock.hitbox());
                        out
                    }
                );
                // println!("{}: {}", i + 1, grid.highest)
                // println!("{}", grid.highest)
            }
        }
        Ok(grid.highest)
    }
}

mod part2 {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use super::*;
    use crate::data::{Grid, TYPES_OF_ROCK};

    #[derive(Debug, Default, Clone)]
    struct Cycle {
        start_height: usize,
        end_height: usize,
        start_rnd: usize,
        end_rnd: usize,
    }

    impl Cycle {
        fn delta_height(&self) -> usize {
            self.end_height - self.start_height
        }
        fn rnd_length(&self) -> usize {
            self.end_rnd - self.start_rnd
        }
    }

    /// Using knowledge of how much height is gained in a cycle fast forward as far as possible.
    /// # Return
    /// Returns (new_round, new_height)
    #[must_use]
    fn skip_simulation(
        cycle: Cycle,
        current_round: usize,
        target_round: usize,
        current_height: usize,
    ) -> (usize, usize) {
        let cycle_repeat_cnt = (target_round - current_round) / cycle.rnd_length();
        (
            current_round + cycle_repeat_cnt * cycle.rnd_length(),
            current_height + cycle_repeat_cnt * cycle.delta_height(),
        )
    }

    const TARGET_RND_NUM: usize = 1000000000000;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_file_static(file_name)?;
        let actions = parse::parse_input(input);
        let action_len = actions.len();
        let mut actions = actions.into_iter().cycle();
        let mut grid = Grid {
            occupied_cells: Cord([0, 0]).interpolate(&Cord([8, 0])).collect(),
            highest: 0,
            highlight_cells: HashSet::new(),
        };
        let mut cached_states = HashSet::new();
        let mut highest_per_col = vec![0; CHAMBER_WIDTH];
        let mut cycle: Cycle = Default::default();
        let mut looking_for_cycle_end = false;
        let mut skipped = false;
        let mut rnd = 0;
        while rnd < TARGET_RND_NUM {
            if LOGGING && rnd % 100 == 0 {
                println!("{rnd}");
            }
            // Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
            let mut rock = Rock {
                kind: ((rnd % TYPES_OF_ROCK) + 1).into(),
                cord: Cord([3, grid.highest + 4]),
            };

            if LOGGING {
                println!(
                    "New Rock {}\n{}",
                    rnd + 1,
                    data::Grid {
                        occupied_cells: grid.occupied_cells.clone(),
                        highest: rock.cord[1] + 5,
                        highlight_cells: rock.hitbox().collect(),
                    }
                );
            }

            // Repeatedly apply jet streams and gravity to move rock until it hits something.
            drop_rock(&mut rock, &mut grid.occupied_cells, &mut actions);

            // Update highest to highest including the newly placed rock.
            grid.highest = grid
                .highest
                .max(rock.hitbox().fold(0, |acc, cord| acc.max(cord[1])));
            for col in 1..=CHAMBER_WIDTH {
                highest_per_col[col - 1] = grid
                    .occupied_cells
                    .iter()
                    .filter(|&x| x[0] == col)
                    .max_by_key(|x| x[1])
                    .map(|&x| x)
                    .unwrap_or_default()[1]
            }

            // Clear rocks that are no longer relevant.
            let min_col = *highest_per_col.iter().min().unwrap();
            grid.occupied_cells.retain(|&x| x[1] >= min_col);

            if LOGGING {
                println!("{:?}", highest_per_col);
                println!(
                    "New Rock {} Placed at Height {}\n{}",
                    rnd + 1,
                    grid.highest,
                    {
                        let mut out = grid.clone();
                        out.highlight_cells.extend(rock.hitbox());
                        out
                    }
                );
            }

            // The things that dictate a particular round are the actions, shape of relevant rocks, and the currently dropping rock.
            let round_hash = {
                let mut hasher = DefaultHasher::default();
                // The position in the actions is part of the information needed to uniquely identify state.
                {
                    let mut out = Vec::with_capacity(action_len);
                    for _ in 0..action_len {
                        out.push(actions.next().unwrap()); // Iterator loops one full cycle so it is not impacted by this.
                    }
                    out
                }
                .hash(&mut hasher);
                grid.occupied_cells
                    .iter()
                    .map(|&c| c - Cord([0, min_col])) // Normalize y value so lowest occupied cell is at 0 for hash purposes.
                    .collect::<BTreeSet<_>>()
                    .hash(&mut hasher);
                rock.kind.hash(&mut hasher); // Only care about rock's type not its position
                hasher.finish()
            };

            // If haven't skipped forward in the simulation yet and the round state matches a previous round's.
            if !skipped && cached_states.contains(&round_hash) {
                // Either start measuring the cycle that is created
                if !looking_for_cycle_end {
                    cycle.start_height = grid.highest;
                    cycle.start_rnd = rnd;
                    looking_for_cycle_end = true;
                    cached_states.clear(); // Don't match again until another cycle goes.
                }
                // Or if already started a cycle, measure the cycle and skip with that information.
                else {
                    cycle.end_height = grid.highest;
                    cycle.end_rnd = rnd;
                    cycle.rnd_length();
                    let (new_round, new_height) =
                        skip_simulation(cycle.clone(), rnd, TARGET_RND_NUM - 1, grid.highest); // Since rounds are 0 indexed number -1 is actual target
                    rnd = new_round;
                    let old_heighest = grid.highest;
                    // Shift all occupied cells upward the amount that was skipped.
                    let mut to_add = Vec::new();
                    for cell in &grid.occupied_cells {
                        to_add.push(Cord([cell[0], cell[1] + new_height - old_heighest]));
                    }
                    grid.occupied_cells.clear();
                    grid.occupied_cells.extend(to_add.iter());
                    grid.highest = new_height;
                    skipped = true;
                }
            }

            // Remember the state for cycle detection.
            cached_states.insert(round_hash);

            rnd += 1;
        }
        Ok(grid.highest)
    }
}

mod parse {
    use super::*;

    pub fn parse_input(input: &str) -> Vec<Action> {
        let mut actions = Vec::new();
        for c in input.chars() {
            match c {
                '<' => actions.push(Action::Left),
                '>' => actions.push(Action::Right),
                '\r' | '\n' => break,
                x => unimplemented!("Unexpected character '{}'?", x),
            }
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input = read_file_static("inputtest.txt")?;
        let actions = parse::parse_input(input);
        dbc!(actions);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 3068);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 3151);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 1514285714288);
        Ok(())
    }

    #[test]
    fn part2_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("input.txt")?, 1560919540245);
        Ok(())
    }
}
