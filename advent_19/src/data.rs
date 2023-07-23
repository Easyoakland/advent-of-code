use derive_more::{Add, AddAssign, Mul, Sub, SubAssign};
use enum_iterator::Sequence;
use std::{cmp::Ordering, collections::BTreeMap};

#[derive(
    Clone, Copy, Debug, Default, Add, AddAssign, Sub, SubAssign, Mul, Sequence, PartialEq, Eq, Hash,
)]
pub struct Resource {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
    pub geode: u8,
}

impl PartialOrd for Resource {
    // Equal only if all resources are the same. If not equal:
    // Resource is less than other other only if consuming that many resources from other is possible and vice versa.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.obsidian),
            self.geode.cmp(&other.geode),
        ) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => {
                Some(Ordering::Equal)
            }
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
            ) => Some(Ordering::Less),
            (
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater | Ordering::Equal,
            ) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Blueprint {
    pub id: usize,
    pub ore_robot_cost: Resource,
    pub clay_robot_cost: Resource,
    pub obsidian_robot_cost: Resource,
    pub geode_robot_cost: Resource,
}

impl Blueprint {
    pub fn construct_cost(&self, robot: &Robot) -> Resource {
        match robot {
            Robot::Ore => self.ore_robot_cost,
            Robot::Clay => self.clay_robot_cost,
            Robot::Obsidian => self.obsidian_robot_cost,
            Robot::Geode => self.geode_robot_cost,
        }
    }

    pub fn affordable_robots(&self, available_resources: Resource) -> Vec<Robot> {
        let res: Vec<Robot> = enum_iterator::all()
            .filter(|x| self.construct_cost(x) <= available_resources)
            .collect();
        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
pub enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    pub fn gen(&self) -> Resource {
        match self {
            Robot::Ore => Resource {
                ore: 1,
                ..Default::default()
            },
            Robot::Clay => Resource {
                clay: 1,
                ..Default::default()
            },
            Robot::Obsidian => Resource {
                obsidian: 1,
                ..Default::default()
            },
            Robot::Geode => Resource {
                geode: 1,
                ..Default::default()
            },
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Round {
    pub minute: u8,
    pub blueprint: Blueprint,
    pub resources: Resource,
    pub robots: BTreeMap<Robot, u8>,
}

impl Round {
    pub fn do_round_in_place(&mut self, robot: Option<Robot>) {
        // Step 0: increment minute
        self.minute += 1;
        // Step 1: spend to create robots
        robot.map(|robot| self.resources -= self.blueprint.construct_cost(&robot));
        // Step 2: robots get resources
        for (robot, cnt) in &self.robots {
            self.resources += robot.gen() * cnt
        }
        // Step 3: create robots that resources were spent to create
        robot.map(|robot| {
            let entry = self.robots.entry(robot).or_default();
            *entry += 1;
        });
    }

    pub fn do_round(&self, robots_to_create: Option<Robot>) -> Self {
        let mut new_rnd = self.clone();
        new_rnd.do_round_in_place(robots_to_create);
        new_rnd
    }

    /// Maximum possible geodes with a given blueprint given the current round state.
    pub fn max_geodes(&self, last_minute: u8) -> u8 {
        let indent = (0..self.minute).map(|_| " ").collect::<String>();
        println!(
            "{}Min: {}, Robots: {:?}, {:?}, {:?}",
            indent, self.minute, self.robots, self.resources, self.blueprint
        );
        if self.minute >= last_minute {
            return self.resources.geode;
        }

        std::iter::once(None)
            .chain(
                self.blueprint
                    .affordable_robots(self.resources)
                    .into_iter()
                    .map(Option::Some),
            )
            .map(|robots_to_create| self.do_round(robots_to_create).max_geodes(last_minute))
            .max()
            .expect("Nonempty")
    }
}
