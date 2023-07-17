use std::fmt;

use rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub fn random_cells(n: u32) -> Vec<Cell> {
    let mut cell: Vec<Cell> = vec![];
    let mut range = thread_rng();

    for _ in 0..n {
        if range.gen_range(0f32, 1f32) <= 0.5 {
            cell.push(Cell::Alive);
        } else {
            cell.push(Cell::Dead);
        }
    }

    cell
}

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub board: Vec<Cell>,
}

impl Game {
    pub fn set_state(&mut self, state: Vec<Cell>) {
        self.board = state;
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn count_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.board[idx] as u8;
            }
        }
        count
    }

    pub fn update(&mut self) {
        let mut next = self.board.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.board[idx];
                let neighbors = self.count_neighbors(row, col);
                let next_cell = match (cell, neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.board = next;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.board.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
