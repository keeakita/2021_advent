use std::collections::VecDeque;
use std::io::{self, BufRead};

use log::debug;

type Int = u32;

fn main() {
    env_logger::init();

    let mut window: VecDeque<Int> = VecDeque::with_capacity(2); // Used as a queue
    let mut last_win_sum: Int = 0;
    let mut depth_count: Int = 0;

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.expect("Failed to read stdin");
        let current: Int = match str::parse::<Int>(&line) {
            Ok(i) => i,
            Err(_) => panic!("Failed to parse number: {}", line),
        };
        debug!("Current: {}", current);

        if i >= 2 {
            let sum = window[0] + window[1] + current;
            debug!("Sum: {}", sum);
            if i > 2 {
                if sum > last_win_sum {
                    depth_count += 1;
                    debug!("Increased!");
                }
            }
            last_win_sum = sum;
        }

        window.push_back(current);
        if window.len() > 2 {
            window.pop_front();
        }

        debug!("Window: {:?}", window);
    }

    println!("Number of depth increases: {}", depth_count);
}
