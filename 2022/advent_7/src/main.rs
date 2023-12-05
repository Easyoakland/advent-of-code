use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    part1::run()?;
    part2::run()?;
    Ok(())
}

#[derive(Debug)]
enum FileSystemObj {
    File(String, u32),
    Dir(String),
}

mod part1 {
    use super::*;
    const MAX_DIR_SIZE: u32 = 100000u32;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let mut result = 0;
        let reader = BufReader::new(f);
        let mut current_dir: Vec<String> = Vec::new();
        let mut filesystem_state: BTreeMap<String, Vec<FileSystemObj>> = BTreeMap::new();

        parse(reader, &mut current_dir, &mut filesystem_state)?;

        for file_sys_obj in &filesystem_state {
            let dirname = file_sys_obj.0;
            let size = find_dir_size(&filesystem_state, &dirname);
            if size <= MAX_DIR_SIZE {
                result += size;
            }
        }

        println!("Part 1 answer: {}", result);

        Ok(())
    }
}

mod part2 {
    use super::*;
    const STORAGE_LIMIT: u32 = 70000000;
    const FREE_SPACE_REQUIRED: u32 = 30000000;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        let mut current_dir: Vec<String> = Vec::new();
        let mut filesystem_state: BTreeMap<String, Vec<FileSystemObj>> = BTreeMap::new();
        let mut removed_choice_options: BinaryHeap<_> = BinaryHeap::new();

        parse(reader, &mut current_dir, &mut filesystem_state)?;

        let total_size = find_dir_size(&filesystem_state, &"".to_string());

        for file_sys_obj in &filesystem_state {
            let dirname = file_sys_obj.0;
            let size = find_dir_size(&filesystem_state, &dirname);
            if STORAGE_LIMIT - (total_size - size) >= FREE_SPACE_REQUIRED {
                removed_choice_options.push((Reverse(size), dirname));
            }
        }

        let result = removed_choice_options.peek().unwrap();
        println!("Part 2 answer: {:?}", result);

        Ok(())
    }
}

fn parse(
    reader: BufReader<File>,
    current_dir: &mut Vec<String>,
    filesystem_state: &mut BTreeMap<String, Vec<FileSystemObj>>,
) -> Result<(), Box<dyn Error>> {
    let mut lines = reader.lines().peekable();
    while let Some(Ok(line)) = lines.next() {
        match &line[0..1] {
            "$" => match &line[2..4] {
                "cd" => match &line[5..] {
                    ".." => {
                        current_dir.pop();
                    }
                    "." => unimplemented!(),
                    "/" => current_dir.clear(),
                    x => current_dir.push(x.to_string()),
                },
                "ls" => {
                    while let Some(Ok(line2)) = lines.peek() {
                        if &line2[0..1] == "$" {
                            break;
                        }
                        let line = lines.next().unwrap()?;
                        let split_line: Vec<_> = line.split_ascii_whitespace().collect();
                        match split_line[0] {
                            "dir" => match filesystem_state.get_mut(
                                &current_dir
                                    .iter()
                                    .cloned()
                                    .map(|s| String::from(s + "/"))
                                    .collect::<String>(),
                            ) {
                                Some(x) => x.push(FileSystemObj::Dir(split_line[1].to_string())),
                                None => {
                                    filesystem_state.insert(
                                        current_dir
                                            .iter()
                                            .cloned()
                                            .map(|s| String::from(s + "/"))
                                            .collect::<String>(),
                                        vec![FileSystemObj::Dir(split_line[1].to_string())],
                                    );
                                }
                            },
                            size => match filesystem_state.get_mut(
                                &current_dir
                                    .iter()
                                    .cloned()
                                    .map(|s| String::from(s + "/"))
                                    .collect::<String>(),
                            ) {
                                Some(x) => x.push(FileSystemObj::File(
                                    split_line[1].to_string(),
                                    u32::from_str_radix(size, 10).unwrap(),
                                )),
                                None => {
                                    filesystem_state.insert(
                                        current_dir
                                            .iter()
                                            .cloned()
                                            .map(|s| String::from(s + "/"))
                                            .collect::<String>(),
                                        vec![FileSystemObj::File(
                                            split_line[1].to_string(),
                                            u32::from_str_radix(size, 10).unwrap(),
                                        )],
                                    );
                                }
                            },
                        }
                    }
                }
                x => panic!("{x} is invalid command!"),
            },
            x => {
                dbg!(x);
                unimplemented!()
            }
        }
    }
    // println!("{:?}", filesystem_state);
    Ok(())
}

fn find_dir_size(filesystem_state: &BTreeMap<String, Vec<FileSystemObj>>, dir: &String) -> u32 {
    let mut size = 0;
    let subdir_vec: Vec<_> = filesystem_state
        .get(dir)
        .unwrap()
        .iter()
        .filter_map(|fso| match fso {
            FileSystemObj::Dir(s) => Some(dir.clone() + &s + "/"),
            FileSystemObj::File(_, x) => {
                size += x;
                None
            }
        })
        .collect();
    for subdir in subdir_vec {
        size += find_dir_size(filesystem_state, &subdir);
    }

    size
}
