use robotics_lib::interface::Direction;

#[derive(Debug, Clone)]
pub enum Action {
    Go(Direction),
    Teleport((usize, usize)),
}

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
