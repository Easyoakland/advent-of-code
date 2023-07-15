/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

Your puzzle answer was BSDMQFLSP.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?

*/

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};
lazy_static! {
    static ref ACTION_PARSE_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    part1::run()?;
    part2::run()?;
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        let mut actions = vec![];

        let mut lines = reader.lines();
        let mut state = parse_initial_state(&mut lines)?;
        for line in lines {
            let line = line?;
            let line = line.to_string();
            let nums = ACTION_PARSE_REGEX.captures_iter(&line).nth(0).unwrap();
            let nums = vec![
                usize::from_str_radix(&nums[1], 10).unwrap(),
                usize::from_str_radix(&nums[2], 10).unwrap(),
                usize::from_str_radix(&nums[3], 10).unwrap(),
            ];
            actions.push(nums);
        }

        for action in actions {
            for _amount in 0..action[0] {
                // Subtract one because of indexing by 1 in input file.
                let temp1 = state[action[1] - 1].pop_back().unwrap();
                state[action[2] - 1].push_back(temp1);
            }
        }

        let result = state
            .iter()
            .map(|col| col.iter().last().unwrap())
            .collect_vec();
        print!("Part 1 answer: ");
        result.iter().for_each(|l| print!("{l}"));
        println!();
        Ok(())
    }
}

mod part2 {
    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        let mut actions = vec![];

        let mut lines = reader.lines();
        let mut state = parse_initial_state(&mut lines)?;
        for line in lines {
            let line = line?;
            let line = line.to_string();
            let nums = ACTION_PARSE_REGEX.captures_iter(&line).nth(0).unwrap();
            let nums = vec![
                usize::from_str_radix(&nums[1], 10).unwrap(),
                usize::from_str_radix(&nums[2], 10).unwrap(),
                usize::from_str_radix(&nums[3], 10).unwrap(),
            ];
            actions.push(nums);
        }

        for action in actions {
            let mut moved_crates = Vec::new();
            for _amount in 0..action[0] {
                // Subtract one because of indexing by 1 in input file.
                // Crates are appended in reverse order onto the intermediate vector.
                moved_crates.push(state[action[1] - 1].pop_back().unwrap());
            }
            // So they are added to the new column in reverse from intermediate (reverse twice is equivalent to no reverse).
            moved_crates.iter().rev().for_each(|x| {
                state[action[2] - 1].push_back(*x);
            });
        }

        let result = state
            .iter()
            .map(|col| col.iter().last().unwrap())
            .collect_vec();
        print!("Part 2 answer: ");
        result.iter().for_each(|l| print!("{l}"));
        println!();
        Ok(())
    }
}

fn parse_initial_state(
    lines: &mut Lines<BufReader<File>>,
) -> Result<Vec<VecDeque<char>>, Box<dyn Error>> {
    let mut state = vec![];
    let mut num_crate_cols;
    let mut first = true;

    while let Some(line) = lines.next() {
        let line = line?;

        if line.trim().is_empty() {
            break;
        }
        // Stuff to do on element. (Identify shape)
        if first {
            // Since the last element has 3 instead of 4 chars must add 1 to compensate.
            num_crate_cols = line.chars().count() / 4 + 1;
            first = false;
            // Resize the vector to fit the state
            for _ in 0..num_crate_cols {
                state.push(VecDeque::new())
            }
        }

        // Split line into the important letters and add them to the ragged vec.
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if i % 4 == 1 { Some((i, c)) } else { None })
            .for_each(|(i, c)| {
                if c != ' ' {
                    state[i / 4].push_front(c);
                }
            });
    }

    Ok(state)
}
