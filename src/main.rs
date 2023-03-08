#[allow(unused_imports)]
use advent_lib::{cord::Cord, dbc, parse::read_file_static};
use data::{Action, Rock};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

type CordType = usize;
const LOGGING: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod data {
    use super::*;
    use std::fmt::Display;

    #[derive(Debug, Clone, Copy)]
    pub enum Action {
        Left,
        Right,
    }

    pub const TYPES_OF_ROCK: usize = 5;
    #[derive(Debug, Clone, Copy)]
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

    #[derive(Clone)]
    pub struct Rock {
        pub kind: RockKind,
        pub cord: Cord<CordType>, // bottom left position
    }

    impl Rock {
        pub fn hitbox(&self) -> Box<dyn Iterator<Item = Cord<CordType>>> {
            match &self.kind {
                RockKind::One => Box::new(self.cord.interpolate(&(self.cord + Cord(3, 0)))),
                RockKind::Two => Box::new(
                    std::iter::once(self.cord + Cord(1, 0))
                        .chain(
                            (self.cord + Cord(0, 1))
                                .interpolate(&((self.cord + Cord(0, 1)) + Cord(2, 0))),
                        )
                        .chain(std::iter::once(self.cord + Cord(1, 2))),
                ),
                RockKind::Three => Box::new(
                    self.cord
                        .interpolate(&(self.cord + Cord(2, 0)))
                        .chain((self.cord + Cord(2, 1)).interpolate(&(self.cord + Cord(2, 2)))),
                ),
                RockKind::Four => Box::new(self.cord.interpolate(&(self.cord + Cord(0, 3)))),
                RockKind::Five => Box::new(self.cord.interpolate(&(self.cord + Cord(1, 1)))),
            }
        }

        /// Returns true if hitbox intersects occupied cells
        fn hitbox_check(&self, occupied_cells: &HashSet<Cord<CordType>>) -> bool {
            self.hitbox().any(|cord| occupied_cells.contains(&cord))
        }
        pub fn fall(&mut self, occupied_cells: &HashSet<Cord<CordType>>) -> bool {
            let proposed_next = {
                let mut out = self.clone();
                out.cord -= Cord(0, 1);
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
                    if self.hitbox().collect::<HashSet<_>>().contains(&Cord(x, y)) {
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
        pub occupied_cells: HashSet<Cord<CordType>>,
        pub highlight_cells: HashSet<Cord<CordType>>,
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
                    if self.highlight_cells.contains(&Cord(x, y)) {
                        write!(f, "@")?;
                    } else if self.occupied_cells.contains(&Cord(x, y)) {
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
    occupied_cells: &mut HashSet<Cord<CordType>>,
    actions: &mut VecDeque<Action>,
) {
    while let Some(action) = actions.pop_front() {
        actions.push_back(action); // actions repeat instead of running out
        match action {
            Action::Left => {
                //DEBUG
                if LOGGING {
                    print!("<");
                }

                if (2..=CHAMBER_WIDTH).contains(&rock.cord.0)
                    && !rock
                        .hitbox()
                        .any(|cord| occupied_cells.contains(&(cord - Cord(1, 0))))
                {
                    rock.cord -= Cord(1, 0)
                }
            }
            Action::Right => {
                //DEBUG
                if LOGGING {
                    print!(">")
                };

                if (1..CHAMBER_WIDTH).contains(&(rock.hitbox().fold(0, |acc, c| acc.max(c.0))))
                    && !rock
                        .hitbox()
                        .any(|cord| occupied_cells.contains(&(cord + Cord(1, 0))))
                {
                    rock.cord += Cord(1, 0)
                }
            }
        }

        // // DEBUG print
        // println!(
        //     "stream\n{}",
        //     data::Grid {
        //         occupied_cells: {
        //             let mut out = occupied_cells.clone();
        //             out.extend(rock.hitbox());
        //             out
        //         },
        //         highest: rock.cord.1 + 5,
        //     }
        // );

        if !rock.fall(occupied_cells) {
            break;
        }

        // // DEBUG print
        // println!(
        //     "gravity\n{}",
        //     data::Grid {
        //         occupied_cells: {
        //             let mut out = occupied_cells.clone();
        //             out.extend(rock.hitbox());
        //             out
        //         },
        //         highest: rock.cord.1 + 5,
        //     }
        // );
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
        let mut actions = parse::parse_input(input);
        let mut grid = Grid {
            occupied_cells: Cord(0, 0).interpolate(&Cord(8, 0)).collect(),
            highest: 0,
            highlight_cells: HashSet::new(),
        };
        for i in 0..2022 {
            // Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
            let mut rock = Rock {
                kind: ((i % TYPES_OF_ROCK) + 1).into(),
                cord: Cord(3, grid.highest + 4),
            };

            // DEBUG print
            if LOGGING {
                println!(
                    "New Rock {}\n{}",
                    i + 1,
                    data::Grid {
                        occupied_cells: grid.occupied_cells.clone(),
                        highest: rock.cord.1 + 5,
                        highlight_cells: rock.hitbox().collect(),
                    }
                );
            }

            // Repeatedly apply jet streams and gravity to move rock until it hits something.
            drop_rock(&mut rock, &mut grid.occupied_cells, &mut actions);

            // Update highest to highest including the newly placed rock.
            grid.highest = grid
                .highest
                .max(rock.hitbox().fold(0, |acc, cord| acc.max(cord.1)));

            // DEBUG print
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

mod parse {
    use super::*;

    pub fn parse_input(input: &str) -> VecDeque<Action> {
        let mut actions = VecDeque::new();
        for c in input.chars() {
            match c {
                '<' => actions.push_back(Action::Left),
                '>' => actions.push_back(Action::Right),
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

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 1707);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt")?, 2615);
    //     Ok(())
    // }
}
