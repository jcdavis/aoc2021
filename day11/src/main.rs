use std::env;
use std::fs;

fn do_flash(board: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    board[i][j] = 0;
    let mut flashes = 1;

    for di in -1..=1 {
        for dj in -1..=1 {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < 10 && nj >= 0 && nj < 10 {
                let ri = ni as usize;
                let rj = nj as usize;
                if board[ri][rj] > 0 {
                    board[ri][rj] += 1;
                    if board[ri][rj] > 9 {
                        flashes += do_flash(board, ri, rj);
                    }
                }
            }
        }
    }
    flashes
}

fn do_turn(board: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashes = 0;

    for i in 0..10 {
        for j in 0..10 {
            board[i][j] += 1;
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if board[i][j] > 9 {
                flashes += do_flash(board, i, j)
            }
        }
    }
    flashes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut board: Vec<Vec<i32>> = input.lines().map(|line| {
        line.chars().map(|c| (c as i32) - ('0' as i32)).collect()
    }).collect();

    let mut turn = 1; 
    while do_turn(&mut board) < 100 {
        turn += 1
    }
    println!("{}", turn);
}
