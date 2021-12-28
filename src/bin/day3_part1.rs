use std::io::{self, BufRead};

use log::debug;

type Int = i32;

fn bin_to_dec(bin: &str) -> Int {
    let mut dec = 0;
    for (pow, char) in bin.chars().rev().enumerate() {
        if char == '1' {
            dec += 1 << pow;
        }
    }

    dec
}

fn main() {
    env_logger::init();

    let mut gamma_zeros: Vec<Int> = Vec::with_capacity(12);
    let mut gamma_ones: Vec<Int> = Vec::with_capacity(12);

    for line in io::stdin().lock().lines() {
        let line = line.expect("Failed to read stdin");
        let width = line.chars().count();
        if gamma_zeros.len() <= width {
            gamma_zeros.resize(width, 0);
        }
        if gamma_ones.len() <= width {
            gamma_ones.resize(width, 0);
        }
        debug!("Gamma zero: {:?}", gamma_zeros);
        debug!("Gamma ones: {:?}", gamma_ones);

        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                gamma_ones[i] += 1;
            } else if char == '0' {
                gamma_zeros[i] += 1;
            } else {
                panic!("not a valid digit!");
            }
        }
    }

    let mut gamma = String::with_capacity(gamma_ones.len());
    let mut epsilon = String::with_capacity(gamma_ones.len());
    for (one_count, zero_count) in gamma_ones.iter().zip(gamma_zeros.iter()) {
        if one_count >= zero_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);

    let gamma_dec = bin_to_dec(&gamma);
    let epsilon_dec = bin_to_dec(&epsilon);
    println!("Gamma_10: {}, Epsilon_10: {}", gamma_dec, epsilon_dec);
    println!("Multiplied: {}", gamma_dec * epsilon_dec);
}
