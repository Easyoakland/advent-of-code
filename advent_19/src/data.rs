use core::fmt::Debug;
use derive_more::{Add, AddAssign, Mul, Sub, SubAssign};
use enum_iterator::Sequence;
use std::{
    cell::Cell,
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Default, Add, AddAssign, Sub, SubAssign, Mul, PartialEq, Eq, Hash)]
pub struct Resource {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
    pub geode: u8,
}

const LOG: bool = false;
pub static mut CNT: usize = 0;

impl PartialOrd for Resource {
    // Equal only if all resources are the same. If not equal:
    // Resource is less than other other only if consuming that many resources from other is possible and vice versa.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
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

impl From<Robot> for Resource {
    fn from(value: Robot) -> Self {
        value.gen()
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
pub enum Robot {
    #[default]
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
    /// The target robot to create next when possible. By always keeping a target will avoid not building a robot when possible and then building it later.
    pub target: Robot,
}

impl Round {
    pub fn do_round(&self) -> Vec<Self> {
        let mut new_rnd = self.clone();

        // Step 0: increment minute
        new_rnd.minute += 1;
        // Step 1: spend to create robot if target robot is possible to create
        let target_cost = self.blueprint.construct_cost(&self.target);
        let construct_target = target_cost <= self.resources;
        if construct_target {
            new_rnd.resources -= target_cost;
        }
        // Step 2: robots get resources
        for (robot, cnt) in &self.robots {
            new_rnd.resources += robot.gen() * cnt
        }

        // Step 3: create robots that resources were spent to create
        if construct_target {
            if LOG {
                let indent = (0..new_rnd.minute).map(|_| " ").collect::<String>();
                println!("{}Built: {:?}", indent, new_rnd.target);
            }
            let entry = new_rnd.robots.entry(self.target).or_default();
            *entry += 1;
        };

        // Step 4: if target was built acquire new target by branching. Otherwise only 1 way to continue
        if construct_target {
            enum_iterator::all()
                .map(|robot| Round {
                    target: robot,
                    ..new_rnd.clone()
                })
                .collect()
        } else {
            vec![new_rnd]
        }
    }
}

/// Assume that a geode bot is created every round. That's the best value this state could ever have.
pub fn best_case_geodes(round: &Round, last_minute: u8) -> u16 {
    let mut current_min = round.minute;
    let mut geode_bots: u16 = round
        .robots
        .get(&Robot::Geode)
        .map(|x| *x)
        .unwrap_or_default()
        .into();
    let mut out = round.resources.geode.into();
    while current_min < last_minute {
        out += geode_bots;
        geode_bots += 1;
        current_min += 1;
    }
    out
}

/// Maximum possible geodes with a given blueprint given the current round state.
#[cached::proc_macro::cached(
    key = "String",
    convert = r#"{ format!("{}{}", last_minute,  {
        let mut hasher = DefaultHasher::default();
        round.hash(&mut hasher);
        hasher.finish()
    }) }"#
)]
pub fn max_geodes(round: Round, last_minute: u8, best: Rc<Cell<u16>>) -> Option<u16> {
    unsafe { CNT += 1 };
    if round.minute >= last_minute {
        return Some(round.resources.geode.into());
    }

    if best_case_geodes(&round, last_minute) < best.get() {
        return None;
    } else {
        let branches = round.do_round();
        let res = branches
            .into_iter()
            .flat_map(|branch| max_geodes(branch, last_minute, best.clone()))
            .max()?;
        if LOG && res > 0 {
            let indent = (0..round.minute).map(|_| " ").collect::<String>();
            println!(
                "{}Expect: {}, Min: {}, Target: {:?}, Robots: {:?}, {:?}, {:?}",
                indent,
                res,
                round.minute,
                round.target,
                round.robots,
                round.resources,
                round.blueprint
            );
        }
        best.set(best.get().max(res));
        Some(res)
    }
}
