use robotics_lib::interface::Direction;

/// Action enumerates the possible actions of a path.
///
/// # Variants:
/// - Go(Direction) => go to the parameter direction.
/// - Teleport((usize, usize)) => teleport to the tile of the parameter coordinates (row, col).
#[derive(Debug, Clone)]
pub enum Action {
    Go(Direction),
    Teleport((usize, usize)),
}

/// Path describes the path from a source tile to a destination tile, with specific cost and actions.
///
/// # Fields:
/// - source: (usize, usize) => source tile of coordinates (row, col).
/// - destination: (usize, usize) => destination tile of coordinates (row, col).
/// - actions: Vec<Action> => actions to be done to move from the source tile to the destination tile.
/// - cost: usize => energy cost of the path.
#[derive(Debug, Default, Clone)]
pub struct Path {
    pub source: (usize, usize),
    pub destination: (usize, usize),
    pub actions: Vec<Action>,
    pub cost: usize,
}

impl Path {
    pub(crate) fn new(source: (usize, usize), destination: (usize, usize), cost: usize) -> Path {
        Path {
            source,
            destination,
            actions: Vec::new(),
            cost,
        }
    }
}
