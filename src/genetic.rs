use crate::game::Cell;

use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct GeneticAlgorithm {
    pub cells: Vec<Cell>,
    pub mut_chance: f32,
    pub individual_mut_chance: f32,
}

impl GeneticAlgorithm {
    pub fn mutate(&mut self) {
        let mut range = thread_rng();

        if range.gen_range(0f32, 1f32) <= self.mut_chance {
            for index in 0..self.cells.len() {
                if range.gen_range(0f32, 1f32) <= self.individual_mut_chance {
                    let new_cell = match self.cells[index] {
                        Cell::Alive => Cell::Dead,
                        Cell::Dead => Cell::Alive,
                    };

                    self.cells[index] = new_cell;
                }
            }
        }
    }
}
