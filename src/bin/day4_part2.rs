use std::{
    fmt::{Debug, Display},
    io::{self, BufRead},
};

use ansi_term::Style;
use log::debug;
use retain_mut::RetainMut;

type Int = i32;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Default, Clone, Copy)]
struct BingoSpace {
    num: Int,
    marked: bool,
}

impl BingoSpace {
    fn new(num: Int) -> Self {
        BingoSpace {
            num: num,
            marked: false,
        }
    }
}

#[derive(Debug, Default)]
struct BingoBoard {
    spaces: [[BingoSpace; BOARD_SIZE]; BOARD_SIZE],
    horiz_counts: [usize; BOARD_SIZE],
    vert_counts: [usize; BOARD_SIZE],
    //    diag_asc_count: usize,
    //    diag_desc_count: usize,
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.spaces {
            for space in line {
                let num_str = format!("{:>3}", space.num);
                let num_fmt = match space.marked {
                    true => Style::new().bold().paint(num_str).to_string(),
                    false => num_str,
                };
                write!(f, "{}", num_fmt)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl BingoBoard {
    fn new(lines: &mut dyn Iterator<Item = &String>) -> Self {
        let mut spaces = [[BingoSpace::default(); BOARD_SIZE]; BOARD_SIZE];

        for (x, line) in lines.enumerate() {
            let nums = line.split(" ").filter(|t| !t.is_empty());
            for (y, num) in nums.enumerate() {
                spaces[x][y] = BingoSpace::new(num.parse().unwrap());
            }
        }

        BingoBoard {
            spaces: spaces,
            ..Default::default()
        }
    }

    // Avoiding early returns in this function to ensure the counts data
    // structures remain accurate on a completed board.
    fn update_totals(&mut self, x: usize, y: usize) -> bool {
        let mut bingo = false;

        self.horiz_counts[x] += 1;
        if self.horiz_counts[x] == BOARD_SIZE {
            bingo = true;
        }

        self.vert_counts[y] += 1;
        if self.vert_counts[y] == BOARD_SIZE {
            bingo = true;
        }

        //        if x == y {
        //            self.diag_desc_count += 1;
        //            if self.diag_desc_count >= BOARD_SIZE {
        //                bingo = true;
        //            }
        //        }

        //        if (BOARD_SIZE - 1 - x) == y {
        //            self.diag_asc_count += 1;
        //            if self.diag_asc_count >= BOARD_SIZE {
        //                bingo = true;
        //            }
        //        }

        bingo
    }

    pub fn mark(&mut self, num: Int) -> bool {
        let mut bingo = false;
        let mut marks: Vec<(usize, usize)> = Vec::new();

        for (x, row) in self.spaces.iter_mut().enumerate() {
            for (y, space) in row.iter_mut().enumerate() {
                if space.num == num {
                    if !space.marked {
                        space.marked = true;
                        marks.push((x, y));
                    }
                }
            }
        }

        for coords in marks {
            if self.update_totals(coords.0, coords.1) {
                bingo = true;
            }
        }

        return bingo;
    }

    pub fn sum(&self) -> Int {
        let mut sum: Int = 0;
        for row in self.spaces {
            for space in row {
                if !space.marked {
                    sum += space.num;
                }
            }
        }

        sum
    }
}

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let calls_line = lines.next().expect("Unexpected EOF");
    let mut calls = calls_line.split(",").map(|n| n.parse().unwrap());

    let lines: Vec<String> = lines.collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    for window in lines.chunks(BOARD_SIZE + 1) {
        // Skip first blank line
        let board_lines = &mut window[1..].into_iter();
        boards.push(BingoBoard::new(board_lines));
    }

    for board in boards.iter() {
        debug!("\n{}", board);
    }

    let mut call: Option<Int> = None;
    while boards.len() > 1 {
        call = calls.next();
        debug!("{:?} call", call);
        debug!("{} len", boards.len());
        boards.retain_mut(|board| !board.mark(call.unwrap()));
    }

    // Keep playing until we finish this board
    let mut board = boards.pop().unwrap();
    loop {
        call = calls.next();
        if board.mark(call.unwrap()) {
            break;
        }
    }

    let sum = board.sum();
    let call = call.unwrap();
    let answer = call * sum;
    println!(
        "Slowest winning board:\n{}\nOn call: {}\nSum: {}\nAnswer: {}",
        board, call, sum, answer
    );
}
