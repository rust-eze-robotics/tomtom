use crate::path::{Action, Path};
use crate::utils::{calculate_go_cost, calculate_teleport_cost};
use robotics_lib::interface::{robot_map, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::World;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq)]
struct State {
    node: (usize, usize),
    distance: usize,
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

pub(crate) fn dijkstra(
    robot: &impl Runnable,
    world: &World,
    source: (usize, usize),
    targets: Vec<(usize, usize)>,
) -> Result<Path, String> {
    if targets.is_empty() {
        return Err(String::from("Path not found!"));
    }

    match robot_map(world) {
        None => Err(String::from("Map not visible!")),
        Some(map) => {
            let (source_row, source_col) = (source.0, source.1);
            let size = map.len();

            if source_row >= size || source_col >= size {
                return Err(String::from("Source out of bounds!"));
            }

            let mut paths = Vec::new();
            let mut teleports = Vec::new();

            for row in 0..size {
                paths.push(Vec::new());

                for col in 0..size {
                    paths[row].push(Path::new(source, (row, col), usize::MAX));

                    if let Some(tile) = map[row][col].as_ref() {
                        if tile.tile_type == TileType::Teleport(true) {
                            teleports.push((row, col));
                        }
                    }
                }
            }

            paths[source_row][source_col].cost = 0;

            let mut heap = BinaryHeap::new();
            heap.push(State {
                node: source,
                distance: 0,
            });

            while !heap.is_empty() {
                let (row, col) = heap.peek().unwrap().node;
                let distance = heap.pop().unwrap().distance;

                if col + 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Right) {
                        if distance + cost < paths[row][col + 1].cost {
                            paths[row][col + 1].cost = distance + cost;
                            paths[row][col + 1].actions = paths[row][col].actions.clone();
                            paths[row][col + 1]
                                .actions
                                .push(Action::Go(Direction::Right));
                            heap.push(State {
                                node: (row, col + 1),
                                distance: distance + cost,
                            });
                        }
                    }
                }

                if row + 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Down) {
                        if distance + cost < paths[row + 1][col].cost {
                            paths[row + 1][col].cost = distance + cost;
                            paths[row + 1][col].actions = paths[row][col].actions.clone();
                            paths[row + 1][col]
                                .actions
                                .push(Action::Go(Direction::Down));
                            heap.push(State {
                                node: (row + 1, col),
                                distance: distance + cost,
                            });
                        }
                    }
                }

                if col - 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Left) {
                        if distance + cost < paths[row][col - 1].cost {
                            paths[row][col - 1].cost = distance + cost;
                            paths[row][col - 1].actions = paths[row][col].actions.clone();
                            paths[row][col - 1]
                                .actions
                                .push(Action::Go(Direction::Left));
                            heap.push(State {
                                node: (row, col - 1),
                                distance: distance + cost,
                            });
                        }
                    }
                }

                if row - 1 < size {
                    if let Ok(cost) = calculate_go_cost(robot, world, Direction::Up) {
                        if distance + cost < paths[row - 1][col].cost {
                            paths[row - 1][col].cost = distance + cost;
                            paths[row - 1][col].actions = paths[row][col].actions.clone();
                            paths[row - 1][col].actions.push(Action::Go(Direction::Up));
                            heap.push(State {
                                node: (row - 1, col),
                                distance: distance + cost,
                            });
                        }
                    }
                }

                if let Some(tile) = map[row][col].as_ref() {
                    if tile.tile_type == TileType::Teleport(true) {
                        for (teleport_row, teleport_col) in teleports.clone() {
                            if let Ok(cost) =
                                calculate_teleport_cost(robot, world, (teleport_row, teleport_col))
                            {
                                if distance + cost < paths[teleport_row][teleport_col].cost {
                                    paths[teleport_row][teleport_col].cost = distance + cost;
                                    paths[teleport_row][teleport_col].actions =
                                        paths[row][col].actions.clone();
                                    paths[teleport_row][teleport_col]
                                        .actions
                                        .push(Action::Teleport((teleport_row, teleport_col)));
                                    heap.push(State {
                                        node: (teleport_row, teleport_col),
                                        distance: distance + cost,
                                    });
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
                Err(String::from("Path not found!"))
            } else {
                Ok(ret)
            }
        }
    }
}
