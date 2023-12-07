use robotics_lib::interface::Direction;

#[derive(Clone)]
pub enum Action {
    Go(Direction),
    Teleport((usize, usize)),
}

#[derive(Default, Clone)]
pub struct Path {
    pub source: (usize, usize),
    pub destination: (usize, usize),
    pub actions: Vec<Action>,
    pub cost: usize,
}

impl Path {
    pub(crate) fn new(source: (usize, usize), destination: (usize, usize), cost: usize) -> Path {
        Path {
            source: source,
            destination: destination,
            actions: Vec::new(),
            cost: cost
        }
    }
}
