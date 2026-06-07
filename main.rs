use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin,stdout,Write};

const WIDTH: usize = 100;
const HEIGHT: usize = 50;
const FRAME_MS: u64 = 150;   

fn main() {
    initialize();

}

fn initialize()　-> Vec<Vec<bool>> {
    let mut grid = vec![vec![false;WIDTH];HEIGHT];
    let mut pattern = vec![(1,1),(1,2),(1,3),(1,4)];
    for p in pattern {
        grid[p.0][p.1] = true;
    }
    return grid
}

// live or die関数
fn life(grid : Vec<Vec<bool>>) {
    // 行を探索
    for index_num in grid.len() {
        // 列を探索
        for column_num in grid[index_num].len() {
            // 周りで生きているセルの数(初期化)
            let mut living_cell = 0; 
            // 上にセルがあれば生存判定
            if index_num != 0 {
                // 上のセルが生きて入ればliveing_cell +=1
                if grid[index_num-1][column_num] == true {
                    living_cell += 1;                   
                }
            }
            // 下
            if index_num != grid.len() {
                if grid[index_num+1][column_num] == true {
                    living_cell += 1;                   
                }
            }
            // 左
            if column_num != 0 {
                if grid[index_num][column_num-1] == true {
                    living_cell += 1;                    
                }
            }
            // 右
            if column_num != grid[index].len() {
                if grid[index_num][column_num+1] == true {
                    living_cell += 1;
                }
            }
            // living_cellが三個ちょうどなら今いるセルをtrueそれ以外ならfalse
            if living_cell == 3 {
                grid[index_num][column_num] == true;
                else if {
                    grid[index_num][column_num] == false;
                }
            }
            }
        }
    }
}
