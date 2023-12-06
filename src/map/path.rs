use robotics_lib::interface::Direction;

pub(crate) struct Path
{
    source: (usize, usize),
    destination: (usize, usize),
    moves: Vec<Direction>,
    energy_cost: usize
}