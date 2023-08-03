#[derive(Clone, Debug)]
struct Ore(u64);

#[derive(Clone, Debug)]
struct Clay(u64);

#[derive(Clone, Debug)]
struct Obsidian(u64);

#[derive(Clone, Debug)]
struct Geode(u64);

#[derive(Clone, Debug)]
struct Resources {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: Ore(0),
            clay: Clay(0),
            obsidian: Obsidian(0),
            geode: Geode(0),
        }
    }

    /// Returns `true` if there are enough resources to build the given `robot` from the specified blueprint, `bp`
    fn can_purchase(&self, robot: &Robot, bp: &Blueprint) -> bool {
        match robot {
            Robot::Ore => {
                let Ore(cost) = bp.ore;
                let Ore(available) = self.ore;

                available >= cost
            }
            Robot::Clay => {
                let Ore(cost) = bp.clay;
                let Ore(available) = self.ore;

                available >= cost
            }
            Robot::Obsidian => {
                let (Ore(ore_cost), Clay(clay_cost)) = bp.obsidian;
                let Ore(ore_available) = self.ore;
                let Clay(clay_available) = self.clay;

                (ore_available >= ore_cost) & (clay_available >= clay_cost)
            }
            Robot::Geode => {
                let (Ore(ore_cost), Obsidian(obsidian_cost)) = bp.geode;
                let Ore(ore_available) = self.ore;
                let Obsidian(obsidian_available) = self.obsidian;

                (ore_available >= ore_cost) & (obsidian_available >= obsidian_cost)
            }
        }
    }

    /// Deducts the necessary resources to purchase the given `robot` from the provided
    /// blueprint, `bp`. If there are not enough resources available, this method will panic.
    fn purchase(&mut self, robot: &Robot, bp: &Blueprint) -> () {
        match robot {
            Robot::Ore => {
                let Ore(cost) = bp.ore;
                let Ore(available) = self.ore;

                if available >= cost {
                    let remaining = available - cost;
                    self.ore = Ore(remaining);
                } else {
                    panic!(
                        "Not enough resources available! {:?} {:?} {:?}",
                        robot, bp, self
                    );
                }
            }
            Robot::Clay => {
                let Ore(cost) = bp.clay;
                let Ore(available) = self.ore;

                if available >= cost {
                    let remaining = available - cost;
                    self.ore = Ore(remaining);
                } else {
                    panic!(
                        "Not enough resources available! {:?} {:?} {:?}",
                        robot, bp, self
                    );
                }
            }
            Robot::Obsidian => {
                let (Ore(ore_cost), Clay(clay_cost)) = bp.obsidian;
                let Ore(ore_available) = self.ore;
                let Clay(clay_available) = self.clay;

                if (ore_available >= ore_cost) & (clay_available >= clay_cost) {
                    let rem_ore = ore_available - ore_cost;
                    let rem_clay = clay_available - clay_cost;
                    self.ore = Ore(rem_ore);
                    self.clay = Clay(rem_clay);
                } else {
                    panic!(
                        "Not enough resources available! {:?} {:?} {:?}",
                        robot, bp, self
                    );
                }
            }
            Robot::Geode => {
                let (Ore(ore_cost), Obsidian(obsidian_cost)) = bp.geode;
                let Ore(ore_available) = self.ore;
                let Obsidian(obsidian_available) = self.obsidian;

                if (ore_available >= ore_cost) & (obsidian_available >= obsidian_cost) {
                    let rem_ore = ore_available - ore_cost;
                    let rem_obsidian = obsidian_available - obsidian_cost;
                    self.ore = Ore(rem_ore);
                    self.obsidian = Obsidian(rem_obsidian);
                } else {
                    panic!(
                        "Not enough resources available! {:?} {:?} {:?}",
                        robot, bp, self
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn harvest_into(&self, resources: &mut Resources) -> () {
        match self {
            Self::Ore => {
                let Ore(n) = resources.ore;
                resources.ore = Ore(n + 1);
            }
            Self::Clay => {
                let Clay(n) = resources.clay;
                resources.clay = Clay(n + 1);
            }
            Self::Obsidian => {
                let Obsidian(n) = resources.obsidian;
                resources.obsidian = Obsidian(n + 1);
            }
            Self::Geode => {
                let Geode(n) = resources.geode;
                resources.geode = Geode(n + 1);
            }
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore: Ore,
    clay: Ore,
    obsidian: (Ore, Clay),
    geode: (Ore, Obsidian),
}

fn robot_builds_available(bp: &Blueprint, resources: &Resources) -> Vec<Robot> {
    let possible = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];
    possible
        .into_iter()
        .filter_map(|robot| {
            if resources.can_purchase(&robot, &bp) {
                Some(robot)
            } else {
                None
            }
        })
        .collect::<Vec<Robot>>()
}

fn best_action(
    choices: Vec<Robot>,
    bp: &Blueprint,
    robots: &Vec<Robot>,
    resources: &Resources,
    t: u64,
    time_limit: u64,
) -> Option<Robot> {
    let (best, _) = choices
        .into_iter()
        .map(|r| Some(r))
        .chain([None]) // this is needed for the "no robot purchase" option
        .map(|r| {
            let mut robots = robots.clone();
            let mut resources = resources.clone();

            if let Some(ref robot) = r {
                // spend the resources at the top of the minute
                resources.purchase(&robot, bp);

                // harvest each robot's resources
                for robot in robots.iter() {
                    robot.harvest_into(&mut resources);
                }

                // the new robot is ready to be added to the fleet
                robots.push(robot.clone());
            } else {
                // this the case where there is no new robot purchased
                // we just need to harvest resources for the existing robots
                for robot in robots.iter() {
                    robot.harvest_into(&mut resources);
                }
            }

            let Geode(n) = harvest(bp, &mut robots, &mut resources, t + 1, time_limit);
            (r, n)
        })
        .max_by_key(|x| x.1) // choose the action that maximizes the number of geodes harvested
        .expect("Cannot choose action from empty set!");

    best
}

fn harvest(
    bp: &Blueprint,
    robots: &mut Vec<Robot>,
    resources: &mut Resources,
    t: u64,
    time_limit: u64,
) -> Geode {
    let mut pending_robot: Option<Robot> = None;

    // only consider building at the *top* of the minute, if time has not expired
    if t < time_limit {
        // 1. at the top of the minute, spend resources to create a robot (if any)
        // it is possible there are no robots that can be built if we don't have enough resources
        let available = robot_builds_available(bp, resources);

        // a decision needs to be made on whether to purchase a robot, or wait to save resources for
        // the future for a (potentially) different robot to purchase
        let action = best_action(available, bp, robots, resources, t, time_limit);
        if let Some(new_robot) = action {
            resources.purchase(&new_robot, bp);
            pending_robot = Some(new_robot);
        }
    }

    // 2. after spending any resources, collect all available resources from each robot
    // note: we defer collecting resources from the new robot since the build is not available
    // until the end of the current minute
    for robot in robots.iter() {
        robot.harvest_into(resources);
    }

    // 3. the entire minute has expired, the new robot (if any) is ready
    if let Some(new_robot) = pending_robot {
        robots.push(new_robot);
    }

    if t < time_limit {
        harvest(bp, robots, resources, t + 1, time_limit)
    } else {
        resources.geode.clone()
    }
}

fn main() {
    let bp = Blueprint {
        id: 1,
        ore: Ore(4),
        clay: Ore(2),
        obsidian: (Ore(3), Clay(14)),
        geode: (Ore(2), Obsidian(7)),
    };
    println!("Simulate using Blueprint: {}", bp.id);

    // we have one Ore collecting robot to begin
    let mut robots = vec![Robot::Ore];

    // and have no resources initally
    let mut resources = Resources::new();

    let geodes = harvest(&bp, &mut robots, &mut resources, 0, 24);
    dbg!(geodes);
}
