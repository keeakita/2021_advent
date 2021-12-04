use std::io::{self, BufRead};

use log::debug;

type Int = i32;

fn bin_to_dec(bin: &[char]) -> Int {
    let mut dec = 0;
    for (pow, c) in bin.iter().rev().enumerate() {
        if *c == '1' {
            dec += 1 << pow;
        }
    }

    dec
}

fn filter_o2(ones: Int, zeros: Int) -> char {
    if ones > zeros {
        '1'
    } else if ones < zeros {
        '0'
    } else {
        '1'
    }
}

fn filter_co2(ones: Int, zeros: Int) -> char {
    if ones > zeros {
        '0'
    } else if ones < zeros {
        '1'
    } else {
        '0'
    }
}

fn calculate(lines: &Vec<Vec<char>>, pos: usize, filter: &dyn Fn(Int, Int) -> char) -> Vec<Vec<char>> {
    if lines.len() == 1 {
        return lines.clone();
    }

    let mut ones = 0;
    let mut zeros = 0;
    for line in lines.iter() {
        match line[pos] {
            '1' => { ones += 1; }
            '0' => { zeros += 1; }
            _ => { panic!("Not a valid digit") }
        }
    }

    let next_lines: Vec<Vec<char>>;
    let filter_val: char = filter(ones, zeros);
    next_lines = lines.iter().filter(|line|
        line[pos] == filter_val
    ).cloned().collect();

    debug!("{:?}", next_lines);
    debug!("zeros: {}, ones: {}", zeros, ones);
    return calculate(&next_lines, pos+1, filter);
}

fn main() {
    env_logger::init();

    let lines: Vec<Vec<char>> = {
        io::stdin().lock().lines()
        .map(|l|l.unwrap().chars().collect())
        .collect()
    };

    let oxygen = calculate(&lines, 0, &filter_o2);
    let co2 = calculate(&lines, 0, &filter_co2);
    println!("O2: {:?}", oxygen);
    println!("CO2: {:?}", co2);

    let oxygen_dec = bin_to_dec(&oxygen[0]);
    let co2_dec = bin_to_dec(&co2[0]);
    println!("O2 decimal: {:?}", oxygen_dec);
    println!("CO2 decimal: {:?}", co2_dec);
    println!("Multiplied: {:?}", oxygen_dec * co2_dec);
}
