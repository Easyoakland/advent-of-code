use std::error::Error;

mod data {
    use advent_lib::{cord::NDCord, parse::yap::digit1};
    use ndarray::Array2;
    use num_derive::{FromPrimitive, ToPrimitive};
    use num_traits::{FromPrimitive, ToPrimitive};
    use std::{
        collections::{BTreeMap, HashMap},
        error::Error,
        fs::File,
        io::Write,
        path::Path,
        str::FromStr,
    };
    use yap::{types::StrTokens, IntoTokens, Tokens};
    pub type Val = isize;
    pub type VelocityVal = isize;
    pub type Pos = NDCord<Val, 2>;
    pub type Velocity = NDCord<VelocityVal, 2>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PosKind {
        Open,
        Wall,
    }

    impl TryFrom<char> for PosKind {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use PosKind::*;
            match value {
                '.' => Ok(Open),
                '#' => Ok(Wall),
                c => Err(c),
            }
        }
    }

    impl FromStr for PosKind {
        type Err = char;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.chars().next().expect("TODO").try_into()
        }
    }

    pub type Map = BTreeMap<Pos, PosKind>;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub enum Rotation {
        Left,
        Right,
    }

    impl TryFrom<char> for Rotation {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use Rotation::*;
            match value {
                'R' => Ok(Right),
                'L' => Ok(Left),
                c => Err(c),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub enum Move {
        Forward(VelocityVal),
        Rotate(Rotation),
    }

    impl<'a> TryFrom<&mut StrTokens<'a>> for Move {
        type Error = ();

        fn try_from(value: &mut StrTokens<'a>) -> Result<Self, Self::Error> {
            digit1(value)
                .map(|x| Move::Forward(x.expect("parse error")))
                .ok_or(value)
                .or_else(|x| {
                    x.next()
                        .ok_or(())
                        .and_then(|x| Rotation::try_from(x).map_err(|_| ()))
                        .and_then(|x| Ok(Move::Rotate(x)))
                })
        }
    }

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tok = &mut s.into_tokens();
            tok.try_into()
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct Cursor {
        pub pos: Pos,
        pub dir: Velocity,
        pub face: Pos,
    }

    impl Cursor {
        pub fn from_map_start1(map: &Map) -> Self {
            let pos = *map
                .iter()
                .find(|(pos, &x)| pos[1] == 0 && x == PosKind::Open) // This works because Btree is ordered
                .expect("Starting position")
                .0;
            Cursor {
                pos,
                dir: Velocity::from([1, 0]),
                face: [0, 0].into(),
            }
        }
        pub fn from_map_start2(map: &Map, map_sidelength: isize) -> Self {
            let absolute_pos = *map
                .iter()
                .find(|(pos, &x)| pos[1] == 0 && x == PosKind::Open) // This works because Btree is ordered
                .expect("Starting position")
                .0;
            Cursor {
                pos: [0, 0].into(),
                dir: Velocity::from([1, 0]),
                face: absolute_pos / map_sidelength,
            }
        }

        pub fn to_global_pos(&self, map_side_len: isize) -> Pos {
            let origin = self.face * map_side_len;
            origin + self.pos
        }
    }

    pub fn rotate(dir: &mut Velocity, rotation: &Rotation) {
        dir.swap(0, 1);
        match rotation {
            // 1,0 -> 0,-1 -> -1,0 -> 0,1
            Rotation::Left => dir[1] *= -1,
            // 1,0 -> 0,1 -> -1,0 -> 0,-1
            Rotation::Right => dir[0] *= -1,
        }
    }

    impl Cursor {
        pub fn mov(&mut self, mov: Move, next_cursor: impl Fn(&Cursor, VelocityVal) -> Cursor) {
            match mov {
                Move::Forward(distance) => *self = next_cursor(&self, distance),
                Move::Rotate(rotation) => rotate(&mut self.dir, &rotation),
            }
        }
    }

    #[allow(dead_code)]
    pub fn log_state(
        file: &Path,
        extents: &(Pos, Pos),
        map: &Map,
        backtrace: &HashMap<Pos, Velocity>,
    ) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file)?;
        let mut extents = extents.clone();
        extents.0.swap(0, 1);
        extents.1.swap(0, 1);
        for cord in extents.0.interpolate(&extents.1).map(|mut x| {
            x.swap(0, 1);
            x
        }) {
            let mut c = ' ';
            if let Some(x) = backtrace.get(&cord) {
                c = match x {
                    NDCord([1, 0]) => '>',
                    NDCord([0, 1]) => 'v',
                    NDCord([-1, 0]) => '<',
                    NDCord([0, -1]) => '^',
                    _ => unreachable!("Invalid direction"),
                };
            } else if let Some(x) = map.get(&cord) {
                c = match x {
                    PosKind::Open => '.',
                    PosKind::Wall => '#',
                };
            }
            write!(file, "{c}")?;
            if cord[0] == extents.1[1] {
                writeln!(file)?;
            }
        }
        Ok(())
    }

    // Direction ordered such that clockwise rotation requires adding to the discriminant.
    #[derive(
        Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive,
    )]
    pub enum Dir {
        Right = 0,
        Down = 1,
        Left = 2,
        Up = 3,
    }

    impl Dir {
        pub fn rotate(self, rotation: &Rotation) -> Self {
            match rotation {
                Rotation::Right => {
                    Self::from_i8((self.to_i8().unwrap() + 1).rem_euclid(4)).unwrap()
                }
                Rotation::Left => Self::from_i8((self.to_i8().unwrap() - 1).rem_euclid(4)).unwrap(),
            }
        }

        pub fn to_velocity(self) -> Velocity {
            match self {
                Dir::Right => [1, 0],
                Dir::Down => [0, 1],
                Dir::Left => [-1, 0],
                Dir::Up => [0, -1],
            }
            .into()
        }

        pub fn from_velocity(velocity: Velocity) -> Self {
            match velocity.0 {
                [1, 0] => Dir::Right,
                [-1, 0] => Dir::Left,
                [0, 1] => Dir::Down,
                [0, -1] => Dir::Up,
                _ => unreachable!("Invalid velocity"),
            }
        }
    }

    /// A face has its own map and edges which connect to other [`Face`]s.
    /// It also has a position on the original input in a 4x4 grid for all possible positions of faces.
    #[derive(Clone, Debug)]
    pub struct Face {
        /// Position of the map from the original unfolded layout. ex `[0,0]` means the top left was the top left of the original folded layout.
        pub pos: Pos,
        pub inner_map: Array2<PosKind>,
        pub edges: BTreeMap<Dir, Pos>,
    }

    impl Face {
        pub fn new(pos: Pos, inner_map: Array2<PosKind>) -> Self {
            Face {
                pos,
                inner_map,
                edges: BTreeMap::new(),
            }
        }
    }

    /// Fold the cube such that all the faces know the face cordinate in the 4x4 grid of their edges.
    pub fn fold_cube(faces: &mut Vec<Face>) {
        let faces_c = faces.clone();
        // Wrap the 4x4 face grid
        let wrapping_add = |this: Pos, other: Pos| {
            let a = this + other;
            Pos::from([a[0].rem_euclid(4), a[1].rem_euclid(4)])
        };
        // Fill in edges that exist explicitly in the input.
        for face in &mut faces.iter_mut() {
            if let Some(n) = faces_c
                .iter()
                .find(|x| x.pos == wrapping_add(face.pos, [-1, 0].into()))
            {
                face.edges.insert(Dir::Left, n.pos);
            }
            if let Some(n) = faces_c
                .iter()
                .find(|x| x.pos == wrapping_add(face.pos, [1, 0].into()))
            {
                face.edges.insert(Dir::Right, n.pos);
            }
            if let Some(n) = faces_c
                .iter()
                .find(|x| x.pos == wrapping_add(face.pos, [0, -1].into()))
            {
                face.edges.insert(Dir::Up, n.pos);
            }
            if let Some(n) = faces_c
                .iter()
                .find(|x| x.pos == wrapping_add(face.pos, [0, 1].into()))
            {
                face.edges.insert(Dir::Down, n.pos);
            }
        }
        // Fill in the rest by noting that if a face has two connections in an L shape then the two ends of the L also meet when folded.
        // Ex. |a|
        //     |b||c|
        // Then a's right side is c's top side. (a's bottom side rotated left touches c's left side rotated right)
        loop {
            for b in faces.clone() {
                for edge_ba in b.edges.clone() {
                    for edge_bc in b.edges.clone() {
                        // If L shape
                        if edge_ba.0.rotate(&Rotation::Right) == edge_bc.0 {
                            if let Some(a) = faces.iter().find(|x| x.pos == edge_ba.1).cloned() {
                                if let Some(c) = faces.iter().find(|x| x.pos == edge_bc.1).cloned()
                                {
                                    let edge_ab = a
                                        .edges
                                        .iter()
                                        .find(|x| *x.1 == b.pos)
                                        .expect("Connected already");
                                    let edge_cb = c
                                        .edges
                                        .iter()
                                        .find(|x| *x.1 == b.pos)
                                        .expect("Connected already");
                                    if let Some(a) = faces.iter_mut().find(|x| x.pos == edge_ba.1) {
                                        a.edges.insert(edge_ab.0.rotate(&Rotation::Left), c.pos);
                                    }
                                    if let Some(c) = faces.iter_mut().find(|x| x.pos == edge_bc.1) {
                                        c.edges.insert(edge_cb.0.rotate(&Rotation::Right), a.pos);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if faces.iter().map(|x| x.edges.len()).sum::<usize>() == 6 * 4 {
                break;
            }
        }
    }
}

mod parse {
    use crate::data::{Map, Move, PosKind};
    use advent_lib::parse::yap::{all_consuming, ParseError};
    use std::collections::BTreeMap;
    use yap::{types::StrTokens, IntoTokens, Tokens};

    pub fn map(input: &mut StrTokens) -> Map {
        let mut out = BTreeMap::new();
        let origin = [0, 0];
        let mut current_pos = origin.clone();
        for c in input.tokens_while(|x| matches!(x, '\n' | ' ' | '.' | '#')) {
            if c == '\n' {
                current_pos[0] = origin[0];
                current_pos[1] += 1;
                continue;
            }
            if c == ' ' {
                current_pos[0] += 1;
                continue;
            }
            match PosKind::try_from(c) {
                Ok(x) => {
                    out.insert(current_pos.into(), x);
                    current_pos[0] += 1;
                }
                Err(_) => break,
            }
        }
        out
    }

    pub fn moves(input: &mut StrTokens) -> Vec<Move> {
        let mut out = Vec::new();
        loop {
            match input.try_into() {
                Ok(x) => out.push(x),
                Err(_) => break,
            }
        }
        out
    }

    pub fn parse_input(input: &str) -> Result<(Map, Vec<Move>), ParseError<char>> {
        all_consuming(&mut input.into_tokens(), |t| (map(t), moves(t)))
    }
}

mod part1 {
    use super::*;
    use crate::{
        data::{Cursor, Dir, Pos, PosKind, Val, VelocityVal},
        parse::parse_input,
    };
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let (map, moves) = parse_input(input)?;
        let extents = Pos::extents_iter(map.iter().map(|x| *x.0)).expect("Nonempty iter");
        let mut cursor = Cursor::from_map_start1(&map);
        let next_pos = |cursor: &Cursor, distance: VelocityVal| {
            let mut new_pos = cursor.pos.clone();
            // Keep moving in that direction until done.
            for _ in 0..distance {
                let mut next_pos = new_pos + cursor.dir;
                new_pos = loop {
                    match map.get(&next_pos) {
                        // Don't change position if will hit a wall.
                        Some(&PosKind::Wall) => break new_pos,
                        // Change position if won't hit a wall.
                        Some(&PosKind::Open) => {
                            // eprintln!("{:?} -> {:?}", &self, next_pos);
                            break next_pos;
                        }
                        // Loop around if the map doesn't include that position.
                        None => {
                            let a = next_pos + cursor.dir;
                            next_pos = Pos::from([
                                a[0].rem_euclid(extents.1[0]),
                                a[1].rem_euclid(extents.1[1]),
                            ]);
                        }
                    }
                };
            }
            Cursor {
                pos: new_pos,
                dir: cursor.dir,
                face: [0, 0].into(),
            }
        };
        for mov in moves {
            cursor.mov(mov, next_pos);
        }
        Ok(1000 * (cursor.pos[1] + 1)
            + 4 * (cursor.pos[0] + 1)
            + Dir::from_velocity(cursor.dir) as isize)
    }
}

mod part2 {
    use super::*;
    use crate::{
        data::{fold_cube, Cursor, Dir, Face, Map, Pos, PosKind, Val, VelocityVal},
        parse::parse_input,
    };
    use advent_lib::{cord::NDCord, iters::NDCartesianProduct, parse::read_and_leak};
    use ndarray::Array2;
    use std::collections::BTreeMap;

    fn folded_cube(
        map_sidelength: isize,
        map: &Map,
    ) -> Result<Vec<Face>, Box<(dyn std::error::Error + 'static)>> {
        let mut faces_unordered = BTreeMap::new();
        // Iterator over all possible face locations. A cube can't be more than 4 faces long when flattened.
        // Longest unfolded cube is
        /*
          |x|      0.
        |x|x|x|    1.
          |x|      2.
          |x|      3.
         */
        let it = (0..4).into_iter().map(|x| x * map_sidelength);
        // Find all the faces that exist and where they exist using a cordinate system for just their potential locations (4x4 grid).
        for top_left in NDCartesianProduct::new([it.clone(), it.clone()]).map(NDCord) {
            if map.get(&top_left).is_some() {
                let bottom_right = top_left + Pos::from([map_sidelength - 1, map_sidelength - 1]);
                let inner_map = Array2::from_shape_vec(
                    (map_sidelength as usize, map_sidelength as usize),
                    top_left
                        .interpolate(&bottom_right)
                        .map(|x| map[&x])
                        .collect(),
                )?;
                faces_unordered.insert(top_left / map_sidelength, inner_map);
            }
        }
        let mut faces: Vec<Face> = faces_unordered
            .into_iter()
            .map(|(pos, inner_map)| Face::new(pos, inner_map))
            .collect();
        fold_cube(&mut faces);
        Ok(faces)
    }

    fn change_face(cursor: &Cursor, faces: &Vec<Face>, map_side_len: isize) -> Cursor {
        let face_from = faces.iter().find(|x| x.pos == cursor.face).unwrap();
        let dir_from = Dir::from_velocity(cursor.dir);
        let face_to = faces
            .iter()
            .find(|x| x.pos == face_from.edges[&dir_from])
            .unwrap();
        let dir_to = *face_to
            .edges
            .iter()
            .find_map(|(dir, &face)| {
                if face == face_from.pos {
                    Some(dir)
                } else {
                    None
                }
            })
            .unwrap();
        // Do what probably amounts to a matrix multiplication by a rotation matrix.
        let mut rotated_axis = if dir_from == dir_to {
            [
                map_side_len - 1 - cursor.pos[0],
                map_side_len - 1 - cursor.pos[1],
            ]
            .into()
        } else if dir_from.rotate(&data::Rotation::Right) == dir_to {
            [cursor.pos[1], map_side_len - 1 - cursor.pos[0]].into()
        } else if dir_from
            .rotate(&data::Rotation::Right)
            .rotate(&data::Rotation::Right)
            == dir_to
        {
            cursor.pos
        } else if dir_from.rotate(&data::Rotation::Left) == dir_to {
            [map_side_len - 1 - cursor.pos[1], cursor.pos[0]].into()
        } else {
            unreachable!()
        };
        // Fix the inner absolute position based on which side its entering the face from.
        match dir_to {
            Dir::Right => rotated_axis[0] = map_side_len - 1,
            Dir::Down => rotated_axis[1] = map_side_len - 1,
            Dir::Left => rotated_axis[0] = 0,
            Dir::Up => rotated_axis[1] = 0,
        }
        Cursor {
            pos: rotated_axis,
            dir: dir_to.to_velocity() * -1, // turn to leave the edge used to enter the face_to
            face: face_to.pos,
        }
    }

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let (map, moves) = parse_input(input)?;
        let map_side_len = ((map.len() / 6) as f64).sqrt() as Val;
        let faces = folded_cube(map_side_len, &map)?;
        // Global position.
        let mut cursor = Cursor::from_map_start2(&map, map_side_len);
        let next_cursor = |cursor: &Cursor, distance: VelocityVal| {
            let mut new_cursor = cursor.clone();
            // Keep moving in that direction until done.
            for _ in 0..distance {
                let mut next_cursor = Cursor {
                    pos: new_cursor.pos + new_cursor.dir,
                    ..new_cursor.clone()
                };
                new_cursor = loop {
                    match faces
                        .iter()
                        .find(|x| x.pos == next_cursor.face)
                        .unwrap()
                        .inner_map
                        .get(next_cursor.pos.map(|x| x as usize))
                    {
                        // Don't change position if will hit a wall.
                        Some(&PosKind::Wall) => break new_cursor,
                        // Change position if won't hit a wall.
                        Some(&PosKind::Open) => break next_cursor,
                        // Wrap when leaving a face
                        None => {
                            next_cursor = change_face(&cursor, &faces, map_side_len);
                        }
                    }
                };
            }
            new_cursor
        };
        for mov in moves {
            cursor.mov(mov, &next_cursor);
        }
        Ok(1000 * (cursor.to_global_pos(map_side_len)[1] + 1)
            + 4 * (cursor.to_global_pos(map_side_len)[0] + 1)
            + Dir::from_velocity(cursor.dir) as Val)
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
        assert_eq!(part1::run("inputtest.txt")?, 6032);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert!(part1::run("input.txt")? > 61338);
        assert_eq!(part1::run("input.txt")?, 126350);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 5031);
        Ok(())
    }

    #[test]
    fn part2_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("input.txt")?, 129339);
        Ok(())
    }
}
