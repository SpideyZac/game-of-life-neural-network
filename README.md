# game-of-life-neural-network

Hey welcome to my first semi advanced Rust project. I understand that the code is pretty bad and was quickly put together with no consideration and there could/should be improvements, especially removing all the .clone instances and using more references.

# What is it?

Game Of Life Neural Network is an implementation of Conway's Game of Life where a set of Genetic Algorithms attempt to develop a starting board state that acts like a neural network where
when certain cells are on other certain cells are on.

# Neural Network Architype

The neural net has 3 defined parts:

- The Input Points which are the cells where inputs will be entered
- The Output Points which are the cells where the outputs will be read
- The Check Point which is the cell that when alivve finalizes the output

# How to run
```
cargo build --release
.\target\release\game-of-life-neural-network(.exe depends on your operating system)
```
