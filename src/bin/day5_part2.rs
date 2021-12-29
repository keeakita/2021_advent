use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

type Int = i32;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse(line: &str) -> (Point, Point) {
    let (start, end) = line.split_once(" -> ").unwrap();
    let (start_y, start_x) = start.split_once(",").unwrap();
    let (end_y, end_x) = end.split_once(",").unwrap();

    (
        Point {
            x: start_x.parse().unwrap(),
            y: start_y.parse().unwrap(),
        },
        Point {
            x: end_x.parse().unwrap(),
            y: end_y.parse().unwrap(),
        },
    )
}

fn draw(grid: &mut Vec<Vec<Int>>, vent: &(Point, Point)) {
    let start = &vent.0;
    let end = &vent.1;

    let min_x = min(start.x, end.x);
    let max_x = max(start.x, end.x);
    let min_y = min(start.y, end.y);
    let max_y = max(start.y, end.y);

    if min_y == max_y {
        for x in min_x..(max_x + 1) {
            grid[x][min_y] += 1;
        }
    } else if min_x == max_x {
        for y in min_y..(max_y + 1) {
            grid[min_x][y] += 1;
        }
    } else {
        let pos_x = end.x as i32 - start.x as i32 > 0;
        let pos_y = end.y as i32 - start.y as i32 > 0;
        let pos_slope = pos_x == pos_y;

        // TODO: I feel like there has to be a cleaner way, but the types being
        // usize makes the -1 * trick hard. A future improvement might be to
        // build a lightweight range iterator that can be flipped.
        if pos_slope {
            for step in 0..(max_x - min_x + 1) {
                println!("{}", min_x + step);
                grid[min_x + step][min_y + step] += 1;
            }
        } else {
            for step in 0..(max_x - min_x + 1) {
                println!("{}", min_x + step);
                grid[min_x + step][max_y - step] += 1;
            }
        }
    }
}

fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let vent_iter = stdin.lock().lines().map(|l| parse(&l.unwrap()));

    let mut max_x = 1;
    let mut max_y = 1;
    let mut vents: Vec<(Point, Point)> = Vec::new();
    for vent in vent_iter {
        if vent.0.x > max_x {
            max_x = vent.0.x;
        }
        if vent.1.x > max_x {
            max_x = vent.1.x;
        }
        if vent.0.y > max_y {
            max_y = vent.0.y;
        }
        if vent.1.y > max_y {
            max_y = vent.1.y;
        }
        vents.push(vent);
    }

    let mut board = Vec::with_capacity(max_x + 1);
    board.resize(max_x + 1, {
        let mut inner = Vec::with_capacity(max_y + 1);
        inner.resize(max_y + 1, 0);
        inner
    });

    for vent in vents {
        draw(&mut board, &vent);
    }

    let mut count = 0;
    for line in board.iter() {
        for space in line.iter() {
            if *space >= 2 {
                count += 1;
            }
        }
    }

    //print_board(&board);
    println!("Crossings > 2: {}", count);
}
