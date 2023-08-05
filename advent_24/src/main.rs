use std::error::Error;

mod data {
    use advent_lib::{cord::NDCord, dir::Dir};
    use ndarray::{Array2, Axis};
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
        iter,
    };
    pub type Map = Array2<Cell>;
    pub type Val = isize;
    pub type Pos = NDCord<Val, 2>;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Cell {
        Wall,
        Floor,
        Blizzards(Vec<Dir>),
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Cell::Wall => write!(f, "#"),
                Cell::Floor => write!(f, "."),
                Cell::Blizzards(b) => {
                    if b.len() == 1 {
                        match b[0] {
                            Dir::Right => write!(f, ">"),
                            Dir::Down => write!(f, "v"),
                            Dir::Left => write!(f, "<"),
                            Dir::Up => write!(f, "^"),
                        }
                    } else {
                        std::fmt::Display::fmt(&b.len(), f)
                    }
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Action {
        Idle,
        Dir(Dir),
    }

    /// Knows how to format a map. Clunky way to get a function that formats with these args.
    pub struct DisplayMap<'a> {
        pub map: &'a Map,
        pub elf: &'a Pos,
    }

    impl<'a> Display for DisplayMap<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Reverse axis because this iterates in logical order instead of lexicographical order
            for (idx, cell) in self.map.clone().reversed_axes().indexed_iter() {
                if self.elf[0] == idx.1.try_into().unwrap()
                    && self.elf[1] == idx.0.try_into().unwrap()
                {
                    write!(f, "E")?;
                } else {
                    std::fmt::Display::fmt(&cell, f)?;
                }
                if idx.1 == self.map.dim().0 - 1 {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    pub fn next_map(map: &Map) -> Map {
        // Save wall positions but replace blizzards with floor so only the new positions remain after the following mutation.
        let mut new_map = Array2::from_shape_vec(
            map.dim(),
            map.iter()
                .map(|x| match x {
                    Cell::Wall => Cell::Wall,
                    Cell::Floor | Cell::Blizzards(_) => Cell::Floor,
                })
                .collect(),
        )
        .expect("Valid shape");

        // Move all blizzards to their new positions in the new_map.
        for (idx, cell) in map.indexed_iter() {
            if let Cell::Blizzards(blizzards) = cell {
                for blizzard in blizzards {
                    let mut wrap_dest = None;
                    if let Some(to_cell) = new_map.get_mut(
                        (blizzard.to_velocity()
                            + [
                                isize::try_from(idx.0).unwrap(),
                                isize::try_from(idx.1).unwrap(),
                            ]
                            .into())
                        .0
                        .map(|x| usize::try_from(x).unwrap()),
                    ) {
                        match to_cell {
                            Cell::Wall => {
                                // Save where the blizzard is going to wrap at for later.
                                wrap_dest = match blizzard {
                                    Dir::Right => Some([1, idx.1]),
                                    Dir::Down => Some([idx.0, 1]),
                                    Dir::Left => Some([map.dim().0 - 2, idx.1]),
                                    Dir::Up => Some([idx.0, map.dim().1 - 2]),
                                }
                            }
                            x @ Cell::Floor => *x = Cell::Blizzards(vec![*blizzard]),
                            Cell::Blizzards(b) => b.push(*blizzard),
                        }
                    }
                    // Handle the wall case
                    if let Some(wrap_dest) = wrap_dest {
                        match &mut new_map[wrap_dest] {
                                x @ Cell::Floor => *x = Cell::Blizzards(vec![*blizzard]),
                                Cell::Blizzards(b) => b.push(*blizzard),
                                Cell::Wall => unreachable!("No walls within the box. Blizzard should wrap to a non-wall location."),
                            }
                    }
                }
            }
        }

        new_map
    }

    /// Wraps `Pos` with a counter so the neighbor function in astar knows what the state of the map is for `next_states`
    #[derive(Clone, Copy, Hash, Debug, Eq)]
    pub struct Round {
        pub pos: Pos,
        pub counter: usize,
    }

    impl PartialEq for Round {
        /// Purposefully don't check counter when comparing so astar finds end based only on pos.
        /// Technically this is not consistent with the stated requirements of Hash but
        /// this causes values in hash to be stored in different buckets locations depending on both
        /// their position and the map's state (counter) when they get there while only checking the pos to terminate the search.
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl Round {
        /// All possible next states.
        pub fn next_states<'a>(
            &'a self,
            next_map: &'a Map,
        ) -> impl Iterator<Item = Self> + Clone + Debug + 'a {
            iter::once(Action::Idle)
                .chain(enum_iterator::all().map(Action::Dir))
                .map(|x| match x {
                    Action::Idle => Self {
                        counter: self.counter + 1,
                        ..self.clone()
                    },
                    Action::Dir(d) => Self {
                        pos: self.pos + d.to_velocity(),
                        counter: self.counter + 1,
                    },
                })
                // Only keep moves that occupy a floor next round.
                .filter(|x| {
                    // Avoid converting signed to unsigned when negative. No signed positions in the map.
                    if x.pos[0] < 0 || x.pos[1] < 0 {
                        false
                    } else {
                        next_map.get(x.pos.map(|x| usize::try_from(x).unwrap()))
                            == Some(&Cell::Floor)
                    }
                })
        }
    }

    pub fn start(map: &Map) -> Option<Pos> {
        map.index_axis(Axis(1), 0)
            .indexed_iter()
            .find_map(|(i, x)| {
                if x == &Cell::Floor {
                    Some(Pos::from([i.try_into().unwrap(), 0]))
                } else {
                    None
                }
            })
    }

    pub fn end(map: &Map) -> Option<Pos> {
        map.axis_iter(Axis(1)).last().and_then(|x| {
            x.indexed_iter().find_map(|(i, x)| {
                if x == &Cell::Floor {
                    Some(Pos::from([
                        i.try_into().unwrap(),
                        (map.dim().1 - 1).try_into().unwrap(),
                    ]))
                } else {
                    None
                }
            })
        })
    }
}

mod parse {
    use crate::data::{Cell, Map};
    use advent_lib::{
        dir::Dir,
        parse::yap::{all_consuming, line_ending, AllConsuming},
    };
    use ndarray::ArrayBase;
    use yap::{IntoTokens, Tokens};

    pub fn initial_map(input: &mut impl Tokens<Item = char>) -> Map {
        let mut data = Vec::new();
        // Is first line
        let mut first = true;
        // 1 cell will be open instead of wall (entrance). Start count from 1 to handle.
        let mut rows = 1;
        let mut cols = 0;
        loop {
            loop {
                if input.token('#') {
                    data.push(Cell::Wall);
                    if first {
                        rows += 1;
                    }
                } else if input.token('.') {
                    data.push(Cell::Floor)
                } else if input.token('^') {
                    data.push(Cell::Blizzards(vec![Dir::Up]))
                } else if input.token('<') {
                    data.push(Cell::Blizzards(vec![Dir::Left]))
                } else if input.token('>') {
                    data.push(Cell::Blizzards(vec![Dir::Right]))
                } else if input.token('v') {
                    data.push(Cell::Blizzards(vec![Dir::Down]))
                } else {
                    first = false;
                    break;
                }
            }
            if line_ending(input).is_some() {
                cols += 1;
            } else {
                break;
            }
        }
        ArrayBase::from_shape_vec((cols, rows), data)
            .expect("Valid dimensions")
            .reversed_axes() // make x axis first number and y axis second number
    }

    pub fn parse_input(input: &str) -> Result<Map, AllConsuming<String>> {
        all_consuming(&mut input.into_tokens(), initial_map)
    }
}

mod part1 {
    use super::*;
    use crate::{
        data::{end, next_map, start, Round},
        parse::parse_input,
    };
    use advent_lib::{algorithms, parse::read_and_leak};

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let map = parse_input(input)?;
        let (start, end) = (start(&map).expect("start"), end(&map).expect("end"));
        // The nth index indicates the next map for the nth state
        let mut cached_next = vec![next_map(&map)];
        let starting_round = Round {
            pos: start,
            counter: 0,
        };
        let ending_round = Round {
            pos: end,
            // Ignored in Eq and everything else
            counter: Default::default(),
        };
        let dist = algorithms::astar(
            starting_round,
            ending_round,
            |x| {
                while cached_next.len() <= x.counter {
                    cached_next.push(next_map(cached_next.last().expect("nonempty")))
                }
                let next_map = &cached_next[x.counter];
                x.next_states(next_map).collect::<Vec<_>>().into_iter()
            },
            |x| x.pos.manhattan_distance(&end),
            |_, _| 1,
            false,
        )
        .expect("Some answer")
        .0;
        Ok(dist.try_into().unwrap())
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
        assert_eq!(part1::run("inputtest.txt")?, 18);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 281);
        Ok(())
    }

    //     #[test]
    //     fn test_part2() -> Result<(), Box<dyn Error>> {
    //         assert_eq!(part2::run("inputtest.txt")?, 5031);
    //         Ok(())
    //     }

    //     #[test]
    //     fn part2_ans() -> Result<(), Box<dyn Error>> {
    //         assert_eq!(part2::run("input.txt")?, 129339);
    //         Ok(())
    //     }
}
