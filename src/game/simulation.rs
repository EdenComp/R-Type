pub struct Simulation {
    pub next_move: (i8, i8),
    pub games: (i32, i32, i32),
    pub percentages: (f32, f32, f32),
    pub nested: Vec<Simulation>,
}

impl Simulation {
    pub fn new(pos: (i8, i8)) -> Simulation {
        Simulation {
            next_move: pos,
            games: (0, 0, 0),
            percentages: (0.0, 0.0, 0.0),
            nested: Vec::new(),
        }
    }
}
