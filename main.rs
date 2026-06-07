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
    let mut input = String::new();
    println!("初期パターンを入力 (例: 5 6 3 2 または 5,6 3,2)");
    stdout().flush().expect("フラッシュ失敗");
    stdin().read_line(&mut input).expect("入力失敗");
    let input = input.replace(",", " ");
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    
    println!("入力された座標: {:?}", nums);
}