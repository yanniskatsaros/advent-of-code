use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

type OccupiedPoints = HashSet<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct GridBoundary {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl GridBoundary {
    /// Counts the number of empty tiles in a given grid boundary for some set of occupied points
    fn empty_spaces(&self, op: &OccupiedPoints) -> u32 {
        // +1 to count each grid unit as a "tile" rather than a point
        // since the distance between two points constitutes a single "tile"
        let nx = self.x_max - self.x_min + 1;
        let ny = self.y_max - self.y_min + 1;

        (nx * ny) as u32 - op.iter().count() as u32
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct ProposedMove {
    prev: Point,
    next: Point,
}

impl ProposedMove {
    fn accept(&self) -> Point {
        self.next.clone()
    }

    fn reject(&self) -> Point {
        self.prev.clone()
    }
}

impl Point {
    /// Returns the point directly to the north of this point
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Returns the point directly to the north east of this point
    fn north_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    /// Returns the point directly to the east of this point
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Returns the point directly to the south east of this point
    fn south_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    /// Returns the point directly to the south of this point
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Returns the point directly to the south west of this point
    fn south_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    /// Returns the point directly to the west of this point
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Returns the point directly to the north west of this point
    fn north_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    /// Returns all 8 points that are adjacent to the point instance
    fn adjacent_points(&self) -> HashSet<Self> {
        HashSet::from_iter([
            self.north(),
            self.north_east(),
            self.east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.west(),
            self.north_west(),
        ])
    }

    /// Returns the 3 points adjacent to this point in the NW, N, NE directions
    fn adjacent_north(&self) -> HashSet<Self> {
        HashSet::from_iter([self.north_west(), self.north(), self.north_east()])
    }

    /// Returns the 3 points adjacent to this point in the NE, E, SE directions
    fn adjacent_east(&self) -> HashSet<Self> {
        HashSet::from_iter([self.north_east(), self.east(), self.south_east()])
    }

    /// Returns the 3 points adjacent to this point in the SE, S, SW directions
    fn adjacent_south(&self) -> HashSet<Self> {
        HashSet::from_iter([self.south_east(), self.south(), self.south_west()])
    }

    /// Returns the 3 points adjacent to this point in the SW, W, NW directions
    fn adjacent_west(&self) -> HashSet<Self> {
        HashSet::from_iter([self.south_west(), self.west(), self.north_west()])
    }

    /// Proposes moving to a new point by inspecting adjacent points in the correct order
    fn propose_move(
        &self,
        occupied: &OccupiedPoints,
        directions: (Direction, Direction, Direction, Direction),
    ) -> ProposedMove {
        // if there are no elves in any of the 8 adjacent points, the elf does nothing
        // we encode this as proposing to "do nothing" by remaining in the same location
        if occupied.intersection(&self.adjacent_points()).count() == 0 {
            return ProposedMove {
                prev: self.clone(),
                next: self.clone(),
            };
        }

        let directions = [directions.0, directions.1, directions.2, directions.3];

        // otherwise, the elf looks in each of the four directions (the order changes
        // every round) and proposes moving one step in the first valid direction
        for dir in directions.iter() {
            match dir {
                Direction::N => {
                    if occupied.intersection(&self.adjacent_north()).count() == 0 {
                        return ProposedMove {
                            prev: self.clone(),
                            next: self.north(),
                        };
                    }
                }
                Direction::E => {
                    if occupied.intersection(&self.adjacent_east()).count() == 0 {
                        return ProposedMove {
                            prev: self.clone(),
                            next: self.east(),
                        };
                    }
                }
                Direction::S => {
                    if occupied.intersection(&self.adjacent_south()).count() == 0 {
                        return ProposedMove {
                            prev: self.clone(),
                            next: self.south(),
                        };
                    }
                }
                Direction::W => {
                    if occupied.intersection(&self.adjacent_west()).count() == 0 {
                        return ProposedMove {
                            prev: self.clone(),
                            next: self.west(),
                        };
                    }
                }
            }
        }

        // the current point is completely surrounded and cannot move anywhere (?)
        ProposedMove {
            prev: self.clone(),
            next: self.clone(),
        }
    }
}

/// Accept/reject each point in the given set of `proposed` moves returning
/// the "new" state of occupied points for all accepted moves.
fn accept_reject(proposed: &Vec<ProposedMove>) -> OccupiedPoints {
    // build a map of the count of each new proposed move
    let counts = proposed.into_iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val.next)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        map
    });

    proposed
        .into_iter()
        .map(|p| {
            // if a proposed move has more than 1 count, reject it; accept it otherwise
            if let Some(1) = counts.get(&p.next) {
                p.accept()
            } else {
                p.reject()
            }
        })
        .collect()
}

/// Decode a "grove scan" string into a hash-set of points for each occupied point
/// (`#`) in the grove. Empty spaces (`.`) are ignored/discarded.
fn parse_grove_scan(s: &str) -> OccupiedPoints {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point {
                    x: x as i32,
                    y: y as i32,
                }),
                _ => None,
            })
        })
        .collect()
}

fn grid_boundary(op: &OccupiedPoints) -> GridBoundary {
    let x_min = op.into_iter().map(|p| p.x).min().unwrap();
    let y_min = op.into_iter().map(|p| p.y).min().unwrap();

    let x_max = op.into_iter().map(|p| p.x).max().unwrap();
    let y_max = op.into_iter().map(|p| p.y).max().unwrap();

    GridBoundary {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

/// Draws an `m x n` grid of the occupied points in the given grove
fn draw_grove(op: &OccupiedPoints) {
    // ( 0,  0) the origin, is the upper-left most point in our coordinate system
    // ( 0, -1) is one unit North of the origin
    // ( 1,  0) is one unit East of the origin
    // ( 0,  1) is one unit South of the origin
    // (-1,  0) is one unit West of the origin
    let gb = grid_boundary(op);
    let mut grid = String::new();

    for y in gb.y_min..=gb.y_max {
        for x in gb.x_min..=gb.x_max {
            let p = Point { x, y };

            if let Some(_) = op.get(&p) {
                grid.push_str("#");
            } else {
                grid.push_str(".");
            }
        }
        grid.push_str("\n");
    }

    println!("{grid}");
}

fn empty_tiles(op: &OccupiedPoints) -> u32 {
    let gb = grid_boundary(op);
    gb.empty_spaces(op)
}

fn simulate(occupied: &OccupiedPoints, rounds: u32) -> OccupiedPoints {
    fn rotate(
        ds: (Direction, Direction, Direction, Direction),
    ) -> (Direction, Direction, Direction, Direction) {
        (ds.1, ds.2, ds.3, ds.0)
    }

    let mut directions = (Direction::N, Direction::S, Direction::W, Direction::E);
    let mut occupied = occupied.clone();

    for _ in 0..rounds {
        let proposed = occupied
            .iter()
            .map(|p| p.propose_move(&occupied, directions))
            .collect::<Vec<_>>();
        occupied = accept_reject(&proposed);

        directions = rotate(directions);
    }

    occupied
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();

    // initial state (round 0)
    let occupied = parse_grove_scan(&input);
    draw_grove(&occupied);

    // state after 10 rounds
    let occupied = simulate(&occupied, 10);
    draw_grove(&occupied);

    println!("Part I: {}", empty_tiles(&occupied));
}
