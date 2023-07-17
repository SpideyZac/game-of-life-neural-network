use crate::genetic::GeneticAlgorithm;
use crate::game::random_cells;

pub struct Population {
    pub size: u32,
    pub players: Vec<GeneticAlgorithm>,
    pub mut_chance: f32,
    pub individual_mut_chance: f32,
    pub cells: u32,
}

impl Population {
    pub fn random(&mut self) {
        self.players = vec![];

        for _ in 0..self.size {
            self.players.push(GeneticAlgorithm {
                cells: random_cells(self.cells),
                mut_chance: self.mut_chance,
                individual_mut_chance: self.individual_mut_chance,
            });
        }
    }

    pub fn new_gen(&mut self, best: GeneticAlgorithm) {
        self.players = vec![];

        for _ in 0..self.size - 1 {
            let mut best_clone = best.clone();
            best_clone.mutate();

            self.players.push(best_clone);
        }

        self.players.push(best);
    }
}
