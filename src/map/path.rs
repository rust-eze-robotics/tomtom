use robotics_lib::interface::Direction;

#[derive(Clone)]
pub enum Move
{
    Go(Direction),
    Teleport
}

#[derive(Default, Clone)]
pub struct Path
{
    pub source: (usize, usize),
    pub destination: (usize, usize),
    pub moves: Vec<Move>,
    pub cost: usize
}

impl Path
{
    pub(crate) fn new(source: (usize, usize), destination: (usize, usize)) -> Path {
        Path {
            source: source,
            destination: destination,
            moves: Vec::new(),
            cost: usize::MAX
        }
    }
}