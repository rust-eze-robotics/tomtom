use robotics_lib::interface::Tools;
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use robotics_lib::world::tile::{Content, TileType};

use crate::map::path::Path;

mod path;
mod utils;

pub struct MapTool {}

impl Tools for MapTool {}

impl MapTool
{
    pub fn get_path_to_coordinate(robot: &impl Runnable, world: &World, adjacent: bool, destination: (usize, usize)) -> Result<Path, ()> {

    }

    pub fn get_path_to_tile(robot: &impl Runnable, world: &World, adjacent: bool, tile_type: Option<TileType>, content: Option<Content>) -> Result<Path, ()> {

    }

    pub fn go_to_coordinate(robot: &impl Runnable, world: &World, adjacent: bool, destination: (usize, usize)) -> Result<(), ()> {

    }

    pub fn go_to_tile(robot: &impl Runnable, world: &World, adjacent: bool, tile_type: Option<TileType>, content: Option<Content>) Result<(), ()> {

    }
}   