use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin,stdout,Write};

const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const FRAME_MS: u64 = 150;   

fn main() {
    initialize();

}

fn initialize() {
    let mut grid = vec![vec![false;WIDTH];HEIGHT];
    let mut pattern = vec![(1,1),(1,2),(1,3),(1,4)]
    for p in pattern {
        grid[p.0][p.1] = true;
    }
}
