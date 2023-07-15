/*
--- Day 6: Tuning Trouble ---

The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

How many characters need to be processed before the first start-of-packet marker is detected?

 */

use std::{error::Error, fs::File, io::{BufReader, Read}};

fn main() -> Result<(), Box<dyn Error>> {
    part1::run()?;
    part2::run()?;
    Ok(())
}

mod part1 {
    const WINDOW_SIZE: usize = 4usize;

    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let mut reader = BufReader::new(f);

        let mut buffer = Vec::new();

        // read the whole file
        reader.read_to_end(&mut buffer)?;

        let pos = find_nonduplicate_index(&buffer, WINDOW_SIZE);

        println!("Part 1 answer: {}", pos.unwrap() + WINDOW_SIZE);

        Ok(())
    }
}

mod part2 {
    const WINDOW_SIZE: usize = 14usize;

    use super::*;
    pub fn run() -> Result<(), Box<dyn Error>> {
        let f = File::open("input.txt")?;
        let mut reader = BufReader::new(f);

        let mut buffer = Vec::new();

        // read the whole file
        reader.read_to_end(&mut buffer)?;

        let pos = find_nonduplicate_index(&buffer, WINDOW_SIZE);

        println!("Part 2 answer: {}", pos.unwrap() + WINDOW_SIZE);

        Ok(())
    }
}

fn find_nonduplicate_index(buffer: &[u8], windows_size: usize) -> Option<usize> {
    buffer.windows(windows_size).position(|w| {
        let mut non_overlapping = 0u32;
        // mod 26 because there are 26 letters that must be distinguished
        // Could improve by using custom iterator instead of `window` and skipping by the lower index of the duplicate instead of checking windows which still contains the duplicate offender.
        for character in w {
            let new = 1 << (character % 26) as u32;
            non_overlapping ^= new;

            // DEBUG
            // println!("{new:#032b}");
            // println!("{non_overlapping:#032b}");
            // println!();
        }

        // This will only be true if there were no overlaps.
        u32::count_ones(non_overlapping) == windows_size as u32
    })
}
