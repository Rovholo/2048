use rand::Rng;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(PartialEq)]
enum KeyBoardKey {Up,Down,Left,Right,Null,Quit}

fn main() {
    gameplay();
}

fn gameplay() {
    let mut board = create_board();
    place_initial_tiles(&mut board);
    print_board(board);
    let play = true;
    while play {
        let key =  key_check();
        let mut moved:bool = false;
        match key {
            KeyBoardKey::Up => moved = move_up(&mut board),
            KeyBoardKey::Down => moved = move_down(&mut board),
            KeyBoardKey::Left => moved = move_left(&mut board),
            KeyBoardKey::Right => moved = move_right(&mut board),
            KeyBoardKey::Quit => break,
            _ => {},
        }
        if moved {
            place_random_tile(&mut board);
            print_board(board);
        }
    
        if check_gameover(board) {
            println!("Game Over!!!");
            break;
        }
    }
}

fn check_gameover(board: [ [u32; 4]; 4]) -> bool {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 0 || (i > 0 && board[i][j] == board[i-1][j])  || (j > 0 && board[i][j] == board[i][j-1]) {
                return false;
            }
        }
    }
    true
}

fn key_check() -> KeyBoardKey {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut key : KeyBoardKey =  KeyBoardKey::Null;
    for c in stdin.keys() {
        // write!(
        //     stdout,
        //     "{}{}",
        //     termion::cursor::Goto(1, 1),
        //     termion::clear::All
        // )
        // .unwrap();

        match c.unwrap() {
            Key::Left => {key = KeyBoardKey::Left; break},
            Key::Right => {key = KeyBoardKey::Right; break},
            Key::Up => {key = KeyBoardKey::Up; break},
            Key::Down => {key = KeyBoardKey::Down; break},
            Key::Ctrl('q') | Key::Esc => {key = KeyBoardKey::Quit; break},
            _ => break,
        }
    }
    stdout.flush().unwrap();
    return key;
}

fn place_initial_tiles(board: &mut [ [u32; 4]; 4]) {
    place_tile(board,2);
    place_tile(board,2);
}

fn place_random_tile(board: &mut [ [u32; 4]; 4]) {
    let ls = [2,4];
    let mut rng = rand::thread_rng();
    let number = ls[rng.gen_range(0,ls.len())];
    place_tile(board,number);
}

fn place_tile(board: &mut [ [u32; 4]; 4], number: u32) {
    let mut rng = rand::thread_rng();
    let vec: Vec<[usize;2]> = available_coord(*board);
    if vec.len() > 0 {
        let coord = vec[rng.gen_range(0,vec.len())];
        board[coord[0]][coord[1]] = number;
    }
}

fn available_coord(board: [ [u32; 4]; 4]) -> Vec<[usize;2]> {
    let mut vec: Vec<[usize;2]> =  Vec::new();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 0 {
                vec.push([i,j]);
            }
        }
    }
    vec
}


fn create_board() -> [ [u32; 4]; 4] {
    [[0; 4]; 4]
}

fn move_up(board: &mut [ [u32; 4]; 4]) -> bool {
    let mut moved:bool = false;
    for i in 1..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] != 0 {
                let mut i2  = i;
                let val = board[i2][j];
                while i2 > 0 && board[i2][j] == val && (board[i2-1][j] == 0 || board[i2-1][j]== val) {
                    board[i2-1][j] += val;
                    board[i2][j] = 0;
                    i2 -= 1;
                    moved = true;
                }
            }
        }
    }
    moved
}

fn move_down(board: &mut [ [u32; 4]; 4]) -> bool {
    let mut moved:bool = false;
    for i in (1..board.len()).rev() {
        for j in 0..board[i].len() {
            if board[i-1][j] != 0 {
                let mut i2  = i;
                let val = board[i2-1][j];
                while i2 < board.len() && board[i2-1][j] == val && (board[i2][j] == 0 || board[i2][j]== val) {
                    board[i2][j] += val;
                    board[i2-1][j] = 0;
                    i2 += 1;
                    moved = true;
                }
            }
        }
    }
    moved
}

fn move_left(board: &mut [ [u32; 4]; 4]) -> bool {
    let mut moved:bool = false;
    for i in 0..board.len() {
        for j in 1..board[i].len() {
            if board[i][j] != 0 {
                let mut j2  = j;
                let val = board[i][j2];
                while j2 > 0 && board[i][j2] == val && (board[i][j2-1] == 0 || board[i][j2-1]== val) {
                    board[i][j2-1] += val;
                    board[i][j2] = 0;
                    j2 -= 1;
                    moved = true;
                }
            }
        }
    }
    moved
}

fn move_right(board: &mut [ [u32; 4]; 4]) -> bool {
    let mut moved:bool = false;
    for i in 0..board.len() {
        for j in (1..board[i].len()).rev() {
            if board[i][j-1] != 0 {
                let mut j2  = j;
                let val = board[i][j2-1];
                while j2 < board.len() && board[i][j2-1] == val && (board[i][j2] == 0 || board[i][j2]== val) {
                    board[i][j2] += val;
                    board[i][j2-1] = 0;
                    j2 += 1;
                    moved = true;
                }
            }
        }
    }
    moved
}

fn print_board(board: [ [u32; 4]; 4]) {
    let length = max_val(board).to_string().len()+1;
    let mut out:String = String::new();
    for row in board {
        for col in row {
            out += &col.to_string();
            for _ in col.to_string().len()..length {
                out += &" ";
            }
        }
        out += "\n\n";
    }
    println!("{}",out);
}

fn max_val(arr : [ [u32; 4];4 ]) -> u32 {
    let mut num = 0;
    for row in arr {
        for col in row {
            if col > num {
                num = col;
            }
        }
    }
    num
}