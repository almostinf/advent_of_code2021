use std::fs::File;
use std::io::prelude::*;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Default)]
struct Game {
    open_board: Vec<i32>,
    close_board: Vec<bool>,
    sum: i32,
}

impl Game {
    pub fn new(open_board: &Vec<i32>) -> Self {
        Game {
            open_board: open_board.clone(),
            close_board: vec![false; BOARD_SIZE * BOARD_SIZE],
            sum: open_board.iter().sum(),
        }
    }

    pub fn set_num(&mut self, num: i32) {
        if let Some(pos) = self.open_board.iter().position(|elem| *elem == num) {
            self.close_board[pos] = true;
            self.sum -= num;
        }
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (x * BOARD_SIZE) + y
    }

    pub fn check_on_win(&self) -> Option<i32> {
        let mut check;

        for i in 0..BOARD_SIZE {
            check = true;
            for j in 0..BOARD_SIZE {
                let pos = self.xy_to_idx(i, j);
                if self.close_board[pos] == false {
                    check = false;
                    break;
                }
            }
            if check {
                return Some(self.sum);
            }
        }

        for i in 0..BOARD_SIZE {
            check = true;
            for j in 0..BOARD_SIZE {
                let pos = self.xy_to_idx(j, i);
                if self.close_board[pos] == false {
                    check = false;
                    break;
                }
            }
            if check {
                return Some(self.sum);
            }
        }

        None
    }
}

fn init_boards(split: Vec<&str>) -> Vec<Game> {
    let mut boards = Vec::<Game>::new();
    let mut buffer = Vec::<i32>::with_capacity(BOARD_SIZE * BOARD_SIZE);
    for (i, elem) in split.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if buffer.len() == BOARD_SIZE * BOARD_SIZE {
            let game = Game::new(&buffer);
            boards.push(game);
            buffer.clear();
        }
        elem.split(' ')
            .map(|elem| elem.trim())
            .filter(|elem| elem.parse::<i32>().is_ok())
            .for_each(|elem| buffer.push(elem.parse::<i32>().expect("Is not a number")))
    }
    if buffer.len() == 25 {
        let game = Game::new(&buffer);
        boards.push(game);
    }
    boards
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let split = content.split("\r\n").collect::<Vec<_>>();
    let predictions = split[0]
        .split(',')
        .map(|elem| elem.parse::<i32>().expect("Parse error!"))
        .collect::<Vec<_>>();
    let mut boards = init_boards(split);

    let mut is_win = false;
    for pred in predictions {
        if is_win {
            break;
        }
        boards.iter_mut().for_each(|elem| elem.set_num(pred));
        boards.iter().for_each(|elem| {
            if let Some(sum) = elem.check_on_win() {
                println!("pred: {}, sum: {}", pred, sum);
                println!("win: {}", pred * sum);
                is_win = true;
            }
        });
    }

    Ok(())
}
