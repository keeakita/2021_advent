use std::io::{self, BufRead};

type Int = u32;

fn main() {
    let mut last: Int = 0;
    let mut depth_count: Int = 0;

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.expect("Failed to read stdin");
        let current: Int = match str::parse::<Int>(&line) {
            Ok(i) => i,
            Err(_) => panic!("Failed to parse number: {}", line),
        };

        if i != 0 {
            if current > last {
                depth_count += 1;
            }
        }
        last = current;
    }

    println!("Number of depth increases: {}", depth_count);
}
