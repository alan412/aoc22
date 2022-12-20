use nom::{
    bytes::complete::take_till, character::complete::digit1, combinator::map_res, sequence::tuple,
    IResult,
};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct RobotCost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

#[derive(Debug)]
struct Blueprint {
    num: i32,
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Inventory {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

#[derive(Debug)]
pub struct Factory {
    blueprints: Vec<Blueprint>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
    fn produce(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }
    fn one_before(&self) -> Self {
        Self {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore - self.ore_robots,
            clay: self.clay - self.clay_robots,
            obsidian: self.obsidian - self.obsidian_robots,
            geodes: self.geodes - self.geode_robots,
        }
    }
    fn next_after(&self) -> Self {
        Self {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
        }
    }
    fn build(self, blueprint: &Blueprint, robot: Material) -> Self {
        let mut other = self;
        let cost = blueprint.cost(robot);

        other.ore -= cost.ore;
        other.clay -= cost.clay;
        other.obsidian -= cost.obsidian;

        other.produce();
        match robot {
            Material::Ore => other.ore_robots += 1,
            Material::Clay => other.clay_robots += 1,
            Material::Obsidian => other.obsidian_robots += 1,
            Material::Geode => other.geode_robots += 1,
        };
        other
    }
    fn robots_of_type(&self, robot: Material) -> i32 {
        match robot {
            Material::Ore => self.ore_robots,
            Material::Clay => self.clay_robots,
            Material::Obsidian => self.obsidian_robots,
            Material::Geode => self.geode_robots,
        }
    }
}

impl RobotCost {
    fn new(ore: i32, clay: i32, obsidian: i32) -> Self {
        Self {
            ore,
            clay,
            obsidian,
        }
    }
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let result: IResult<
            &str,
            (
                &str,
                i32,
                &str,
                i32,
                &str,
                i32,
                &str,
                i32,
                &str,
                i32,
                &str,
                i32,
                &str,
                i32,
            ),
        > = tuple((
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
            take_till(|c: char| c.is_numeric()),
            map_res(digit1, i32::from_str),
        ))(line);
        let tuple = result.unwrap().1;
        Self {
            num: tuple.1,
            ore_robot_cost: RobotCost::new(tuple.3, 0, 0),
            clay_robot_cost: RobotCost::new(tuple.5, 0, 0),
            obsidian_robot_cost: RobotCost::new(tuple.7, tuple.9, 0),
            geode_robot_cost: RobotCost::new(tuple.11, 0, tuple.13),
        }
    }
    fn cost(&self, robot: Material) -> RobotCost {
        match robot {
            Material::Ore => self.ore_robot_cost,
            Material::Clay => self.clay_robot_cost,
            Material::Obsidian => self.obsidian_robot_cost,
            Material::Geode => self.geode_robot_cost,
        }
    }
    fn can_build(&self, inv: &Inventory, robot: Material) -> bool {
        let cost = self.cost(robot);
        inv.ore >= cost.ore && inv.clay >= cost.clay && inv.obsidian >= cost.obsidian
    }
    // Not just can we, but should we ... "2 Big Macs for 2$"
    fn should_build(&self, inv: &Inventory, robot: Material, built: bool) -> bool {
        if robot == Material::Geode {
            return true;
        }
        let material_cost = |c: RobotCost| match robot {
            Material::Ore => c.ore,
            Material::Clay => c.clay,
            Material::Obsidian => c.obsidian,
            Material::Geode => unreachable!(),
        };

        let max_cost = [
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost,
            self.geode_robot_cost,
        ]
        .into_iter()
        .map(material_cost)
        .max()
        .unwrap_or(0);

        let still_needed = inv.robots_of_type(robot) < max_cost;

        if !built {
            let prev_inventory = inv.one_before();
            let skipped = self.can_build(&prev_inventory, robot);
            still_needed && !skipped
        } else {
            still_needed
        }
    }
}

impl Factory {
    pub fn new() -> Self {
        Self {
            blueprints: Vec::new(),
        }
    }
    pub fn add_line(&mut self, line: &str) {
        self.blueprints.push(Blueprint::new(line));
    }
    pub fn pt_1(&self) -> i32 {
        let mut quality_levels = 0;
        for blueprint in self.blueprints.iter() {
            println!("{:?}", blueprint);
            let max = Self::search(&blueprint, 24);
            println!("--{}", max);
            quality_levels += max * blueprint.num;
        }
        quality_levels
    }
    pub fn pt_2(&self) -> i32 {
        self.blueprints
            .iter()
            .take(3)
            .map(|blueprint| Self::search(blueprint, 32))
            .product()
    }
    fn search(factory: &Blueprint, minutes: usize) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((Inventory::new(), 0, false));

        let mut cache = vec![0; minutes + 1];

        while let Some((inv, min, built)) = queue.pop_front() {
            let prior_best = cache[min];
            if inv.geodes < prior_best {
                continue;
            }
            cache[min] = prior_best.max(inv.geodes);

            if min == minutes {
                continue;
            }
            if factory.can_build(&inv, Material::Geode) {
                queue.push_back((inv.build(factory, Material::Geode), min + 1, true));
                continue;
            }
            queue.push_back((inv.next_after(), min + 1, false));

            for robot in [Material::Obsidian, Material::Clay, Material::Ore] {
                if factory.can_build(&inv, robot) && factory.should_build(&inv, robot, built) {
                    queue.push_back((inv.build(factory, robot), min + 1, true));
                }
            }
        }
        return cache[minutes];
    }
}
