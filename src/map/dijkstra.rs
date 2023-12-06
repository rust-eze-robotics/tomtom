use std::collections::BinaryHeap;
use std::cmp::Ordering;
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use robotics_lib::interface::{Direction, robot_map};
use robotics_lib::world::tile::TileType;
use crate::map::path::{Path, Move};
use crate::map::utils::{calculate_go_cost, calculate_teleport_cost};

#[derive(Eq)]
struct State
{
    node: (usize, usize),
    distance: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.distance == other.distance
    }
}

pub(crate) fn dijkstra(robot: &impl Runnable, world: &World, size: usize, source: (usize, usize), targets: Vec<(usize, usize)>) -> Result<Path, String> {
    match robot_map(world) {
        None => {
            Err(String::from("Map not visible!"))
        },
        Some(map) => {
            let (source_row, source_col) = (source.0, source.1);

            if source_row >= size || source_col >= size {
                return Err(String::from("Source out of bounds!"));
            }

            let mut paths = Vec::new();
            let mut teleports = Vec::new();

            for row in 0..size {
                paths.push(Vec::new());
                
                for col in 0..size {
                    paths[row].push(Path::new(source, (row, col)));

                    if let Some(tile) = map[row][col] {
                        if tile.tile_type == TileType::Teleport(true) {
                            teleports.push((row, col));
                        }
                    }
                }
            }

            paths[source_row][source_col].cost = 0;

            let mut heap = BinaryHeap::new();
            heap.push(State{node: source, distance: 0});

            while !heap.is_empty() {
                let (row, col) = heap.peek().unwrap().node;
                let distance = heap.pop().unwrap().distance;

                if col + 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Right) {
                        if distance + cost < paths[row][col + 1].cost {
                            paths[row][col + 1].cost = distance + cost;
                            paths[row][col + 1].moves = paths[row][col].moves;
                            paths[row][col + 1].moves.push(Move::Go(Direction::Right));
                            heap.push(State{node: (row, col + 1), distance: distance + cost});
                        }
                    }
                }

                if row + 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Down) {
                        if distance + cost < paths[row + 1][col].cost {
                            paths[row + 1][col].cost = distance + cost;
                            paths[row + 1][col].moves = paths[row][col].moves;
                            paths[row + 1][col].moves.push(Move::Go(Direction::Down));
                            heap.push(State{node: (row + 1, col), distance: distance + cost});
                        }
                    }
                }

                if col - 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Left) {
                        if distance + cost < paths[row][col - 1].cost {
                            paths[row][col - 1].cost = distance + cost;
                            paths[row][col - 1].moves = paths[row][col].moves;
                            paths[row][col - 1].moves.push(Move::Go(Direction::Left));
                            heap.push(State{node: (row, col + 1), distance: distance + cost});
                        }
                    }
                }

                if row - 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Up) {
                        if distance + cost < paths[row - 1][col].cost {
                            paths[row - 1][col].cost = distance + cost;
                            paths[row - 1][col].moves = paths[row][col].moves;
                            paths[row - 1][col].moves.push(Move::Go(Direction::Up));
                            heap.push(State{node: (row - 1, col), distance: distance + cost});
                        }
                    }
                }

                if let Some(tile) = map[row][col] {
                    if tile.tile_type == TileType::Teleport(true) {
                        for (teleport_row, teleport_col) in teleports {
                            if let Ok(cost) = calculate_teleport_cost(robot, world, (teleport_row, teleport_col)) {
                                if distance + cost < paths[teleport_row][teleport_col].cost {
                                    paths[teleport_row][teleport_col].cost = distance + cost;
                                    paths[teleport_row][teleport_col].moves = paths[row][col].moves;
                                    paths[teleport_row][teleport_col].moves.push(Move::Teleport);
                                    heap.push(State{node: (teleport_row, teleport_col), distance: distance + cost});
                                }
                            }
                        }
                    }
                }
            }

            let mut ret = Path::default();
            ret.cost = usize::MAX;

            for (target_row, target_col) in targets {
                if paths[target_row][target_col].cost < ret.cost {
                    ret = paths[target_row][target_col].clone();
                }
            }

            if ret.cost == usize::MAX {
                Ok(ret)
            } else {
                Err(String::from("Path not found!"))
            }
        }
    }
} 