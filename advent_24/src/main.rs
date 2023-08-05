use std::error::Error;

mod data {
    use advent_lib::{cord::NDCord, dbc, dir::Dir};
    use ndarray::{Array2, Axis};
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
        fmt::{Debug, Display},
        iter,
        rc::Rc,
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

    /// Iterator of Map state over time.
    #[derive(Clone, Debug)]
    pub struct ChronoMap {
        map: Map,
        // Map of all transitions already computed. Clones share a cache to avoid duplicate work.
        cache: Rc<RefCell<HashMap<Map, Map>>>,
    }

    impl ChronoMap {
        pub fn new(map: Map) -> Self {
            Self {
                map,
                cache: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }

    impl Display for ChronoMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Reverse axis because this iterates in logical order instead of lexicographical order
            for (idx, cell) in self.map.clone().reversed_axes().indexed_iter() {
                std::fmt::Display::fmt(&cell, f)?;
                if idx.1 == self.map.dim().0 - 1 {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    impl Iterator for ChronoMap {
        type Item = Map;

        fn next(&mut self) -> Option<Self::Item> {
            // Check cache first
            if let Some(next) = self.cache.borrow().get(&self.map) {
                dbg!("cached");
                self.map = next.clone();
                return Some(next.clone());
            }

            // Save wall positions but replace blizzards with floor so only the new positions remain after the following mutation.
            let mut new_map = Array2::from_shape_vec(
                self.map.dim(),
                self.map
                    .iter()
                    .map(|x| match x {
                        Cell::Wall => Cell::Wall,
                        Cell::Floor | Cell::Blizzards(_) => Cell::Floor,
                    })
                    .collect(),
            )
            .expect("Valid shape");
            // Move all blizzards to their new positions in the new_map.
            for (idx, cell) in self.map.indexed_iter() {
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
                                        Dir::Left => Some([self.map.dim().0 - 2, idx.1]),
                                        Dir::Up => Some([idx.0, self.map.dim().1 - 2]),
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

            // Cache result before returning
            let old_map = core::mem::replace(&mut self.map, new_map);
            self.cache.borrow_mut().insert(old_map, self.map.clone());
            Some(self.map.clone())
        }
    }

    #[derive(Clone, Debug)]
    pub struct Round {
        pub pos: Pos,
        pub counter: u32,
    }

    impl Round {
        /// All possible next states
        pub fn next_states<'a>(
            &'a self,
            map: &'a Map,
        ) -> impl Iterator<Item = Self> + Clone + Debug + 'a {
            iter::once(Action::Idle)
                .chain(enum_iterator::all().map(Action::Dir))
                .flat_map(|x| match x {
                    Action::Idle => Some(Self {
                        counter: self.counter + 1,
                        ..self.clone()
                    }),
                    Action::Dir(d) => {
                        let next = self.pos + d.to_velocity();
                        if 0 < next[0]
                            && next[0] < (map.dim().0 - 1).try_into().unwrap()
                            && 0 < next[1]
                            && next[1] < (map.dim().1 - 1).try_into().unwrap()
                        {
                            Some(Self {
                                pos: next,
                                counter: self.counter + 1,
                            })
                        } else {
                            None
                        }
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
                        map.dim().1.try_into().unwrap(),
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
        advent_lib::dbc!(data.len(), rows, cols, &data);
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
        data::{end, start, ChronoMap, Round},
        parse::parse_input,
    };
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let map = parse_input(input)?;
        let (start, end) = (start(&map).expect("start"), end(&map).expect("end"));
        let start = Round {
            pos: start,
            counter: 0,
        };
        let mut chrono = ChronoMap::new(map);
        for i in 0..100 {
            eprintln!("{i}\n{chrono}");
            chrono.next();
        }

        todo!()
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

    //     #[test]
    //     fn part1_ans() -> Result<(), Box<dyn Error>> {
    //         assert!(part1::run("input.txt")? > 61338);
    //         assert_eq!(part1::run("input.txt")?, 126350);
    //         Ok(())
    //     }

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
