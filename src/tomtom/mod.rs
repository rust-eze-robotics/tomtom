use robotics_lib::interface::{go, teleport, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;

use self::dijkstra::dijkstra;
use self::path::{Action, Path};
use self::utils::{get_adjacent_tiles, get_specific_tiles};

mod dijkstra;
mod path;
mod utils;

pub struct TomTom {}

impl Tools for TomTom {}

impl TomTom {
    pub fn get_path_to_coordinate(
        &self,
        robot: &impl Runnable,
        world: &World,
        adjacent: bool,
        destination: (usize, usize),
    ) -> Result<Path, String> {
        let source = (
            robot.get_coordinate().get_row(),
            robot.get_coordinate().get_col(),
        );
        let mut targets = Vec::new();

        if adjacent {
            targets.append(&mut get_adjacent_tiles(world, destination));
        } else {
            targets.push(destination);
        }

        dijkstra(robot, world, source, targets)
    }

    pub fn get_path_to_tile(
        &self,
        robot: &impl Runnable,
        world: &World,
        adjacent: bool,
        tile_type: Option<TileType>,
        content: Option<Content>,
    ) -> Result<Path, String> {
        let source = (
            robot.get_coordinate().get_row(),
            robot.get_coordinate().get_col(),
        );
        let destinations = get_specific_tiles(world, &tile_type, &content);

        let mut targets = Vec::new();

        for destination in destinations {
            if adjacent {
                targets.append(&mut get_adjacent_tiles(world, destination));
            } else {
                targets.push(destination);
            }
        }

        dijkstra(robot, world, source, targets)
    }

    pub fn go_to_coordinate(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        adjacent: bool,
        destination: (usize, usize),
    ) -> Result<(), String> {
        match self.get_path_to_coordinate(robot, world, adjacent, destination) {
            Err(e) => Err(e),
            Ok(path) => {
                for action in path.actions {
                    match action {
                        Action::Go(d) => {
                            if let Err(_) = go(robot, world, d) {
                                return Err(String::from("Error while calling go interface!"));
                            }
                        }
                        Action::Teleport((row, col)) => {
                            if let Err(_) = teleport(robot, world, (row, col)) {
                                return Err(String::from(
                                    "Error while calling teleport interface!",
                                ));
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }

    pub fn go_to_tile(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        adjacent: bool,
        tile_type: Option<TileType>,
        content: Option<Content>,
    ) -> Result<(), String> {
        match self.get_path_to_tile(robot, world, adjacent, tile_type, content) {
            Err(e) => Err(e),
            Ok(path) => {
                for action in path.actions {
                    match action {
                        Action::Go(d) => {
                            if let Err(_) = go(robot, world, d) {
                                return Err(String::from("Error while calling go interface!"));
                            }
                        }
                        Action::Teleport((row, col)) => {
                            if let Err(_) = teleport(robot, world, (row, col)) {
                                return Err(String::from(
                                    "Error while calling teleport interface!",
                                ));
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
