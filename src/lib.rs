use crate::dijkstra::dijkstra;
use crate::path::{Action, Path};
use crate::utils::{get_adjacent_tiles, get_specific_tiles};
use robotics_lib::interface::{go, teleport, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;

pub mod path;
mod dijkstra;
mod utils;

#[derive(Default)]
pub struct TomTom {}

impl Tools for TomTom {}

impl TomTom {
    /// get_path_to_coordinates returns the path having the smallest energy cost to reach the destination tile at the given coordinates
    /// (or the 'nearest' adjacent tile), considering: go interface costs, tiles' walkability and elevation, environmental conditions and teleports.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - adjacent: bool => if true the function will target the adjacent tiles to destination; if false it will target destination itself.
    /// - destination: (usize, usize) => destination tile of coordinates (row, col).
    ///
    /// # Return
    /// - Result<Path, String>   
    pub fn get_path_to_coordinates(
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

    /// get_path_to_tile returns the path having the smallest energy cost to reach the 'nearest' matched tile (or one of its adjacent tiles),
    /// considering: go interface costs, tiles' walkability and elevation, environmental conditions and teleports.
    /// Matched tiles are the tiles, discovered by the robot, that match the optional tile type and content.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - adjacent: bool => if true the function will target the adjacent tiles to the matched tiles, if false it will target the matched tiles themselves.
    /// - tile_type: Option<TileType> => optional tile type to be matched.
    /// - content: Option<Content> => optional content to be matched.  
    ///
    /// # Return
    /// - Result<Path, String>
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

    /// go_to_coordinates calls get_path_to_coordinates: if the result is Ok(path) and the robot has enough energy to complete the path, it moves
    /// the robot to the path's destination tile.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - adjacent: bool => if true the function will target the adjacent tiles to destination; if false it will target destination itself.
    /// - destination: (usize, usize) => destination tile of coordinates (row, col).
    ///
    /// # Return
    /// - Result<Path, String>
    pub fn go_to_coordinates(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        adjacent: bool,
        destination: (usize, usize),
    ) -> Result<Path, String> {
        match self.get_path_to_coordinates(robot, world, adjacent, destination) {
            Err(e) => Err(e),
            Ok(path) => {
                if !robot.get_energy().has_enough_energy(path.cost) {
                    return Err(String::from("Not enough energy!"));
                }

                for action in path.actions.iter() {
                    match action {
                        Action::Go(d) => {
                            if let Err(_) = go(robot, world, d.clone()) {
                                return Err(String::from("Error while calling go interface!"));
                            }
                        }
                        Action::Teleport((row, col)) => {
                            if let Err(_) = teleport(robot, world, (*row, *col)) {
                                return Err(String::from(
                                    "Error while calling teleport interface!",
                                ));
                            }
                        }
                    }
                }

                Ok(path)
            }
        }
    }

    /// Calls get_path_to_tile: if the result is Ok(path) and the robot has enough energy to complete the path, it moves
    /// the robot to the path's destination tile.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - adjacent: bool => if true the function will target the adjacent tiles to the matched tiles, if false it will target the matched tiles themselves.
    /// - tile_type: Option<TileType> => optional tile type to be matched.
    /// - content: Option<Content> => optional content to be matched.  
    ///
    /// # Return
    /// - Result<Path, String>
    pub fn go_to_tile(
        &self,
        robot: &mut impl Runnable,
        world: &mut World,
        adjacent: bool,
        tile_type: Option<TileType>,
        content: Option<Content>,
    ) -> Result<Path, String> {
        match self.get_path_to_tile(robot, world, adjacent, tile_type, content) {
            Err(e) => Err(e),
            Ok(path) => {
                if !robot.get_energy().has_enough_energy(path.cost) {
                    return Err(String::from("Not enough energy!"));
                }

                for action in path.actions.iter() {
                    match action {
                        Action::Go(d) => {
                            if let Err(_) = go(robot, world, d.clone()) {
                                return Err(String::from("Error while calling go interface!"));
                            }
                        }
                        Action::Teleport((row, col)) => {
                            if let Err(_) = teleport(robot, world, (*row, *col)) {
                                return Err(String::from(
                                    "Error while calling teleport interface!",
                                ));
                            }
                        }
                    }
                }

                Ok(path)
            }
        }
    }
}
