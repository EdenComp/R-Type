use crate::game::GameEnd;

#[derive(Clone)]
pub struct Simulation {
    pub self_simulation: NestedSimulation,
    pub nested: Vec<NestedSimulation>,
}

#[derive(Clone)]
pub struct NestedSimulation {
    pub next_move: (i8, i8),
    pub games: (i32, i32, i32),
    pub percentages: (f32, f32, f32),
    pub simulations: usize,
    pub divider: f32,
}

impl Simulation {
    pub fn new(pos: (i8, i8), simulations: usize) -> Simulation {
        Simulation {
            self_simulation: NestedSimulation::new(pos, simulations),
            nested: Vec::new(),
        }
    }
}

impl NestedSimulation {
    pub fn new(pos: (i8, i8), simulations: usize) -> NestedSimulation {
        NestedSimulation {
            next_move: pos,
            games: (0, 0, 0),
            percentages: (0.0, 0.0, 0.0),
            simulations,
            divider: simulations as f32,
        }
    }

    pub fn add_game(&mut self, end: GameEnd) {
        match end {
            GameEnd::Victory => self.games.0 += 1,
            GameEnd::Defeat => self.games.1 += 1,
            GameEnd::Draw => self.games.2 += 1,
        }
    }

    pub fn calculate_percentages(&mut self) {
        self.percentages.0 = (self.games.0 as f32 / self.divider) * 100.0;
        self.percentages.1 = (self.games.1 as f32 / self.divider) * 100.0;
        self.percentages.2 = (self.games.2 as f32 / self.divider) * 100.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::types::NestedSimulation;
    use crate::game::GameEnd;

    #[test]
    fn test_calcul_percentage() {
        let mut nested_simulation = NestedSimulation::new((0, 0), 100);
        nested_simulation.games = (100, 100, 0);
        nested_simulation.calculate_percentages();
        assert_eq!(nested_simulation.percentages, (100.0, 100.0, 0.0));
    }

    #[test]
    fn test_add_game() {
        let mut nested_simulation = NestedSimulation::new((0, 0), 100);
        nested_simulation.add_game(GameEnd::Victory);
        nested_simulation.add_game(GameEnd::Defeat);
        nested_simulation.add_game(GameEnd::Draw);
        assert_eq!(nested_simulation.games, (1, 1, 1));
    }
}
