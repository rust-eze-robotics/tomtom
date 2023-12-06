use robotics_lib::interface::Tools;
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use robotics_lib::world::tile::{Content, TileType};

use self::path::Path;
use self::dijkstra::dijkstra;
use self::utils::get_adjacent_tiles;

mod path;
mod dijkstra;
mod utils;

pub struct MapTool {}

impl Tools for MapTool {}

impl MapTool
{
    pub fn get_path_to_coordinate(robot: &impl Runnable, world: &World, size: usize, adjacent: bool, destination: (usize, usize)) -> Result<Path, String> {
        let source = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
        let mut targets = Vec::new();
        
        if adjacent {
            targets = get_adjacent_tiles(robot, world, size, destination);
        } else {
            targets.push(destination);
        }

        dijkstra(robot, world, size, source, targets)
    }

    pub fn get_path_to_tile(robot: &impl Runnable, world: &World, size: usize, adjacent: bool, tile_type: Option<TileType>, content: Option<Content>) -> Result<Path, String> {

    }

    pub fn go_to_coordinate(robot: &impl Runnable, world: &World, size: usize, adjacent: bool, destination: (usize, usize)) -> Result<(), String> {

    }

    pub fn go_to_tile(robot: &impl Runnable, world: &World, size: usize, adjacent: bool, tile_type: Option<TileType>, content: Option<Content>) Result<(), String> {

    }
}   