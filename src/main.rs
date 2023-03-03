#[allow(unused_imports)]
use advent_lib::{self, dbc, parse::read_file_static};
use cached::proc_macro::cached;
use std::{collections::BTreeMap, error::Error};

/* TODO use djistra's algorithm after weighting edges inversely to the benefit of arriving at node at a time. */
/* Look into Floyd-Warshall https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm*/
/* Even more alternatively make a tree 30 minutes (steps) deep and find longest weighted path */

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt", 4000000)?);
    Ok(())
}

mod parse {
    use super::*;
    use regex::Regex;

    /// Panics if the regex partially matches 3 fields of values.
    pub fn parse_input(input: &str) -> (BTreeMap<&str, u32>, BTreeMap<(&str, &str), u32>) {
        //Valve TM has flow rate=3; tunnels lead to valves WB, PE, DX, TK, CH
        let re = Regex::new(
            r"Valve (\w\w) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ((?:\w(?:, )?)+)(?:\r\n|\r|\n)?",
        )
        .unwrap();
        // BTreeMaps can be hashed so they can be later used in the cached function.
        let mut distances = BTreeMap::new();
        let mut flowrates = BTreeMap::new();
        for cap in re.captures_iter(input) {
            let valve_id = cap.get(1).unwrap().as_str();
            let flowrate = cap.get(2).unwrap().as_str().to_string().parse().unwrap();
            let valve_connections = cap.get(3).unwrap().as_str();
            let valve_connections: Vec<_> = valve_connections.split(", ").collect();
            flowrates.insert(valve_id, flowrate);
            for valve_connection in valve_connections {
                distances.insert((valve_id, valve_connection), 1);
            }
        }

        // Distance to self is 0
        for &valve in flowrates.keys() {
            distances.insert((valve, valve), 0);
        }

        (flowrates, distances)
    }
}

fn floyd_wershall<'a>(
    flowrates: &'a BTreeMap<&'a str, u32>,
    distances: &mut BTreeMap<(&'a str, &'a str), u32>,
) {
    // Floyd-Warshall. Could have done dijkstra but learning new algorithm, its simple, everyone online referred to it for some reason even though its slower than dijkstra.
    for &k in flowrates.keys() {
        for &j in flowrates.keys() {
            for &i in flowrates.keys() {
                // Initialize distances with infinity.
                if distances.get(&(i, j)) == None {
                    distances.insert((i, j), u32::MAX);
                }
                if distances.get(&(i, k)) == None {
                    distances.insert((i, j), u32::MAX);
                }
                if distances.get(&(k, j)) == None {
                    distances.insert((i, j), u32::MAX);
                }

                distances.insert(
                    (i, j),
                    distances[&(i, j)].min(distances[&(i, k)].saturating_add(distances[&(k, j)])),
                );
            }
        }
    }
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}{}{:?}", time, start, flowrates) }"#
)]
fn max_value(
    time: u32,
    start: String,
    flowrates: BTreeMap<String, u32>,
    distances: &BTreeMap<(String, String), u32>,
) -> u32 {
    flowrates
        .iter()
        // Get the valve, flowrate, and distance from the start position to the valve
        .map(|(v, &f)| (v, f, distances[&(start.clone(), v.clone())].clone()))
        // For each valve that can be reached in time.
        .filter(|(_v, _f, d)| d < &time)
        // Map it to its max value.
        // Note that the valve shouldn't be directly reached by subrecursions so it is removed from flowrates.
        .map(|(v, f, d)| {
            (f * (time - d - 1))
                + if flowrates.len() > 1 {
                    max_value(
                        time - d - 1,
                        v.clone(),
                        {
                            let mut f_clone = flowrates.clone();
                            f_clone.remove_entry(v);
                            f_clone
                        },
                        distances,
                    )
                } else {
                    0
                }
        })
        .max()
        .unwrap_or_default()
}

mod part1 {
    use super::*;

    const MAX_MINUTES: u32 = 30;
    pub fn run(file: &str) -> Result<u32, Box<dyn Error>> {
        let input = read_file_static(file)?;
        let (flowrates, mut distances) = parse::parse_input(input);
        floyd_wershall(&flowrates, &mut distances); // calc distances

        let distances = distances
            .into_iter()
            // .filter(|&((u, v), _)| flowrates[u] != 0 && flowrates[v] != 0)
            .map(|((u, v), x)| ((u.to_string(), v.to_string()), x))
            .collect::<BTreeMap<_, _>>();
        // Remove zero valves (they're useless). Also convert indexes to Strings for max_value later
        let flowrates = flowrates
            .into_iter()
            .filter(|&(_, f)| f != 0)
            .map(|(v, f)| (v.to_string(), f))
            .collect::<BTreeMap<_, _>>();
        Ok(max_value(
            MAX_MINUTES,
            "AA".to_string(),
            flowrates,
            &distances,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input = read_file_static("inputtest.txt")?;
        let (flowrates, distances) = parse::parse_input(input);
        dbc!(flowrates, distances);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 1651);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("input.txt")?, 2119);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt", 20)?, 56000011);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt", 4000000)?, 11558423398893);
    //     Ok(())
    // }
}
