use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

use rand::seq::{IteratorRandom, SliceRandom};

fn main() {
    println!("Hello, world!");
    Sudoku::new_wfc();
}

const SUDOKU_BOX_WIDTH: usize = 3;
const SUDOKU_SIDE_LENGTH: usize = SUDOKU_BOX_WIDTH * SUDOKU_BOX_WIDTH;

struct Sudoku {
    grid: [[Option<usize>; SUDOKU_SIDE_LENGTH]; SUDOKU_SIDE_LENGTH],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Entered,
    StillNeeded,
    Invalid,
}

#[derive(Debug, PartialEq, Eq)]
struct Cell {
    x: usize,
    y: usize,
    b: usize,
    n: [bool; SUDOKU_SIDE_LENGTH],
    state: CellState,
}

fn get_box(x: usize, y: usize) -> usize {
    x / SUDOKU_BOX_WIDTH + SUDOKU_BOX_WIDTH * (y / 3)
}

impl Sudoku {
    fn new_wfc() -> Self {
        let mut to_collapse = BinaryHeap::new();
        let mut sudoku = [[None; SUDOKU_SIDE_LENGTH]; SUDOKU_SIDE_LENGTH],
        for y in 0..SUDOKU_SIDE_LENGTH {
            for x in 0..SUDOKU_SIDE_LENGTH {
                
            }
        }

        todo!("wave-function-collapse")
    }
}


