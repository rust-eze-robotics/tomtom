use robotics_lib::interface::{look_at_sky, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::calculate_cost_go_with_environment;
use robotics_lib::world::{tile::Tile, tile::TileType, World};

use crate::plain::{PlainContent, PlainTileType};

fn get_coords_row_col(source: (usize, usize), direction: Direction) -> (usize, usize) {
    let (row, col) = source;

    match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    }
}

pub(crate) fn get_adjacent_tiles(
    map: &Vec<Vec<Option<Tile>>>,
    tile: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    let (row, col) = tile;
    let size = map.len();

    if col + 1 < size && map[row][col + 1].is_some() {
        ret.push((row, col + 1));
    }

    if row + 1 < size && map[row + 1][col].is_some() {
        ret.push((row + 1, col));
    }

    if col > 0 && map[row][col - 1].is_some() {
        ret.push((row, col - 1));
    }

    if row > 0 && map[row - 1][col].is_some() {
        ret.push((row - 1, col));
    }

    ret
}

pub(crate) fn get_specific_tiles(
    map: &Vec<Vec<Option<Tile>>>,
    plain_tile_type: &Option<PlainTileType>,
    plain_content: &Option<PlainContent>,
) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    let size = map.len();

    for row in 0..size {
        for col in 0..size {
            if let Some(tile) = map[row][col].as_ref() {
                let mut control = true;

                if let Some(plain_tile_type) = plain_tile_type {
                    if !plain_tile_type.eq_tile_type(&tile.tile_type) {
                        control = false;
                    }
                }

                if let Some(plain_content) = plain_content {
                    if !plain_content.eq_content(&tile.content) {
                        control = false;
                    }
                }

                if control {
                    ret.push((row, col));
                }
            }
        }
    }

    ret
}

pub(crate) fn calculate_go_cost(
    world: &World,
    map: &Vec<Vec<Option<Tile>>>,
    source: (usize, usize),
    direction: Direction,
) -> Result<usize, String> {
    let (source_row, source_col) = source;
    let (destination_row, destination_col) = get_coords_row_col(source, direction);

    if map[source_row][source_col].is_none() {
        return Err(String::from("Source is None!"));
    }

    if map[destination_row][destination_col].is_none() {
        return Err(String::from("Destination is None!"));
    }

    let source = map[source_row][source_col].clone().unwrap();
    let destination = map[destination_row][destination_col].clone().unwrap();

    if !destination.tile_type.properties().walk() {
        return Err(String::from("Go not allowed!"));
    }

    let mut base_cost = destination.tile_type.properties().cost();
    let mut elevation_cost = 0;

    base_cost =
        calculate_cost_go_with_environment(base_cost, look_at_sky(world), destination.tile_type);

    if destination.elevation > source.elevation {
        elevation_cost = (destination.elevation - source.elevation).pow(2);
    }

    Ok(base_cost + elevation_cost)
}

pub(crate) fn calculate_teleport_cost(
    robot: &impl Runnable,
    map: &Vec<Vec<Option<Tile>>>,
    destination: (usize, usize),
) -> Result<usize, String> {
    let (source_row, source_col) = (
        robot.get_coordinate().get_row(),
        robot.get_coordinate().get_col(),
    );

    let (destination_row, destination_col) = (destination.0, destination.1);

    let size = map.len();

    if source_row >= size || source_col >= size {
        return Err(String::from("Source out of bounds!"));
    }

    if destination_row >= size || destination_col >= size {
        return Err(String::from("Destination out of bounds!"));
    }

    match &map[source_row][source_col] {
        None => {
            return Err(String::from("Source is None!"));
        }
        Some(tile) => {
            if tile.tile_type != TileType::Teleport(true) {
                return Err(String::from("Source is not a teleport!"));
            }
        }
    }

    match &map[destination_row][destination_col] {
        None => {
            return Err(String::from("Destination is None!"));
        }
        Some(tile) => {
            if tile.tile_type != TileType::Teleport(true) {
                return Err(String::from("Destination is not a teleport!"));
            }
        }
    }

    Ok(30)
}
