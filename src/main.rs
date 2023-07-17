pub mod game;
pub mod genetic;
pub mod population;
pub mod train;

use crate::train::train;
use crate::train::TrainData;
use crate::train::Point;
use crate::game::Cell;
use crate::game::Game;

use std::{thread, time::Duration};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let best = train(
        100,
        100,
        100,
        10,
        10,
        &mut vec![
            TrainData {
                inputs: vec![0, 0],
                outputs: vec![0],
            },
            TrainData {
                inputs: vec![1, 0],
                outputs: vec![1],
            },
            TrainData {
                inputs: vec![0, 1],
                outputs: vec![1],
            },
            TrainData {
                inputs: vec![1, 1],
                outputs: vec![0],
            },
        ],
        vec![
            Point {
                row: 3,
                col: 0,
            },
            Point {
                row: 5,
                col: 0,
            },
        ],
        vec![
            Point {
                row: 4,
                col: 9,
            },
        ],
        Point {
            row: 4,
            col: 4,
        },
    );

    println!("Training Elapsed: {:.2?}", now.elapsed());

    loop {
        for training_data_point in &mut vec![
            TrainData {
                inputs: vec![0, 0],
                outputs: vec![0],
            },
            TrainData {
                inputs: vec![1, 0],
                outputs: vec![1],
            },
            TrainData {
                inputs: vec![0, 1],
                outputs: vec![1],
            },
            TrainData {
                inputs: vec![1, 1],
                outputs: vec![0],
            },
        ].iter() {
            let mut game = Game {
                board: best.cells.clone(),
                width: 10,
                height: 10,
            };

            let mut input_idxs: Vec<usize> = vec![];
            for input in vec![
                Point {
                    row: 3,
                    col: 0,
                },
                Point {
                    row: 5,
                    col: 0,
                },
            ].iter() {
                input_idxs.push(game.get_index(input.row, input.col));
            }

            let mut output_idxs: Vec<usize> = vec![];
            for output in vec![
                Point {
                    row: 4,
                    col: 9,
                },
            ].iter() {
                output_idxs.push(game.get_index(output.row, output.col));
            }

            let check_point_idx = game.get_index(Point {
                row: 4,
                col: 4,
            }.row, Point {
                row: 4,
                col: 4,
            }.col);

            for (i, input_data) in training_data_point.inputs.iter().enumerate() {
                match input_data {
                    1 => game.board[input_idxs[i]] = Cell::Alive,
                    0 => game.board[input_idxs[i]] = Cell::Dead,
                    _ => (),
                }
            }

            println!("{game}");
            thread::sleep(Duration::from_secs(1));

            loop {
                game.update();
                println!("{game}");
                thread::sleep(Duration::from_secs(1));

                if game.board[check_point_idx] == Cell::Alive {
                    for (i, output) in training_data_point.outputs.iter().enumerate() {
                        let mut correct = Cell::Alive;
                        if *output == 0 {
                            correct = Cell::Dead;
                        }

                        if game.board[output_idxs[i]] == correct {
                            println!("{:?}", training_data_point.inputs);
                            println!("{:?}", training_data_point.outputs);
                            println!("{:?}", game.board[output_idxs[i]]);
                            println!("Correct!");
                        } else {
                            println!("Wrong!");
                        }
                    }
                    println!("\n\n");

                    break;
                }
            }
        }
    }
}
