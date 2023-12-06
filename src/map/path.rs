use robotics_lib::interface::Direction;

#[derive(Default, Clone)]
pub struct Path
{
    pub source: (usize, usize),
    pub destination: (usize, usize),
    pub moves: Vec<Direction>,
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