use robotics_lib::utils::{go_allowed, in_bounds, calculate_cost_go_with_environment};
use robotics_lib::interface::{Direction, robot_map, look_at_sky};
use robotics_lib::runner::Runnable;
use robotics_lib::world::{World, tile::TileType, tile::Tile};

fn get_coords_row_col(robot: &impl Runnable, direction: Direction) -> (usize, usize) {
    let row = robot.get_coordinate().get_row();
    let col = robot.get_coordinate().get_col();
    match direction {
        | Direction::Up => (row - 1, col),
        | Direction::Down => (row + 1, col),
        | Direction::Left => (row, col - 1),
        | Direction::Right => (row, col + 1),
    }
}

pub(crate) fn get_adjacent_tiles(robot: &impl Runnable, world: &World, size: usize, tile: (usize, usize)) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    if let Some(map) = robot_map(world) {
        let (row, col) = tile;

        if col + 1 < size && map[row][col + 1].is_some() {
            ret.push((row, col + 1));
        }

        if row + 1 < size && map[row + 1][col].is_some() {
            ret.push((row + 1, col));
        }

        if col - 1 < size && map[row][col - 1].is_some() {
            ret.push((row, col - 1));
        }

        if row - 1 < size && map[row - 1][col].is_some() {
            ret.push((row - 1, col));
        }
    }

    ret
}

pub(crate) fn calculate_go_cost(robot: &impl Runnable, world: &World, direction: Direction) -> Result<usize, String> {
    if go_allowed(robot, world, &direction).is_err() {
        return Err(String::from("Go not allowed!"));
    }

    match robot_map(world) {
        None => {
            return Err(String::from("Map not visible!"));
        },
        Some(map) => {
            let (source_row, source_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
            let (destination_row, destination_col) = get_coords_row_col(robot, direction);

            if map[source_row][source_col].is_none() {
                return Err(String::from("Source is None!"));
            }

            if map[destination_row][destination_col].is_none() {
                return Err(String::from("Destination is None!"));
            }

            let source = map[source_row][source_col].unwrap();
            let destination = map[destination_row][destination_col].unwrap();

            let mut base_cost = destination.tile_type.properties().cost();
            let mut elevation_cost;

            base_cost = calculate_cost_go_with_environment(base_cost, look_at_sky(world), destination.tile_type);

            if destination.elevation > source.elevation {
                elevation_cost = (destination.elevation - source.elevation).pow(2);
            }
            
            Ok(base_cost + elevation_cost)
        }
    }
}

pub(crate) fn calculate_teleport_cost(robot: &impl Runnable, world: &World, destination: (usize, usize)) -> Result<usize, String> {
    match robot_map(world) {
        None => {
            return Err(String::from("Map not visible!"));
        },
        Some(map) => {
            let (source_row, source_col) = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
            let (destination_row, destination_col) = (destination.0, destination.1);

            if source_row >= map.len() || source_col >= map[0].len() {
                return Err(String::from("Source out of bounds!"));
            }

            if destination_row >= map.len() || destination_col >= map[0].len() {
                return Err(String::from("Destination out of bounds!"));
            }

            if map[source_row][source_col].is_none() {
                return Err(String::from("Source is None!"));
            }

            if map[destination_row][destination_col].is_none() {
                return Err(String::from("Destination is None!"));
            }

            if map[source_row][source_col].unwrap().tile_type != TileType::Teleport(true) {
                return Err(String::from("Source is not a teleport!"));
            }

            if map[destination_row][destination_col].unwrap().tile_type != TileType::Teleport(true) {
                return Err(String::from("Destination is not a teleport!"));
            }

            Ok(30)
        }
    }
}