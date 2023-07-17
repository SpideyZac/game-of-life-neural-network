use crate::game::Cell;
use crate::game::Game;
use crate::genetic::GeneticAlgorithm;
use crate::population::Population;

use rand::{thread_rng, Rng};

pub struct TrainData {
    pub inputs: Vec<u8>,
    pub outputs: Vec<u8>,
}

pub struct Point {
    pub row: u32,
    pub col: u32,
}

pub fn train(
    size: u32,
    iterations: u32,
    mini_iterations: u32,
    width: u32,
    height: u32,
    training_data: &mut Vec<TrainData>,
    input_points: Vec<Point>,
    output_points: Vec<Point>,
    check_point: Point,
) -> GeneticAlgorithm {
    let mut range = thread_rng();

    let mut population = Population {
        size,
        players: vec![],
        mut_chance: 0.2,
        individual_mut_chance: 0.1,
        cells: width * height,
    };
    population.random();

    let mut best = population.players[0].clone();
    let mut best_score = 0;

    for _ in 0..iterations {
        let mut local_best = population.players[0].clone();
        let mut local_best_score = 0;
        let mut rewards: Vec<i32> = vec![];

        for ga in population.players.iter() {
            range.shuffle(training_data);

            let mut reward: i32 = (width as i32) * (height as i32)
                - (ga.cells.iter().filter(|x| **x == Cell::Alive).count() as i32);

            for training_data_point in training_data.iter() {
                let mut game = Game {
                    board: ga.cells.clone(),
                    width,
                    height,
                };

                let mut input_idxs: Vec<usize> = vec![];
                for input in input_points.iter() {
                    input_idxs.push(game.get_index(input.row, input.col));
                }

                let mut output_idxs: Vec<usize> = vec![];
                for output in output_points.iter() {
                    output_idxs.push(game.get_index(output.row, output.col));
                }

                let check_point_idx = game.get_index(check_point.row, check_point.col);

                for (i, input_data) in training_data_point.inputs.iter().enumerate() {
                    match input_data {
                        1 => game.board[input_idxs[i]] = Cell::Alive,
                        0 => game.board[input_idxs[i]] = Cell::Dead,
                        _ => (),
                    }
                }

                let mut mini_iter = 0;
                while mini_iter < mini_iterations {
                    if game.board[check_point_idx] == Cell::Alive {
                        for (i, output) in training_data_point.outputs.iter().enumerate() {
                            let mut correct = Cell::Alive;
                            if *output == 0 {
                                correct = Cell::Dead;
                            }

                            if game.board[output_idxs[i]] == correct {
                                reward += 10 + ((mini_iterations - mini_iter) as i32);
                            } else {
                                reward -= 10;
                            }
                        }

                        break;
                    }

                    game.update();
                    mini_iter += 1;

                    if mini_iter == mini_iterations {
                        reward -= 10;
                    }
                }
            }

            rewards.push(reward);

            if reward > local_best_score {
                local_best_score = reward;
                local_best = ga.clone();
            }

            if reward > best_score {
                best_score = reward;
                best = ga.clone();
            }
        }

        population.new_gen(local_best.clone());
    }

    best.clone()
}
