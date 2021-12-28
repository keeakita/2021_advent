use std::io::{self, BufRead};

type Int = i32;

enum Direction {
    UP,
    DOWN,
    FORWARD,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            "forward" => Ok(Direction::FORWARD),
            _ => Err("Invalid enum".into()),
        }
    }
}

struct Instruction {
    direction: Direction,
    magnitude: Int,
}

impl TryFrom<&str> for Instruction {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir_str, mag_str) = value.split_once(" ").ok_or("line has no space???")?;
        let magnitude = match mag_str.parse::<Int>() {
            Ok(i) => i,
            Err(e) => return Err(format!("{}", e)),
        };
        Ok(Instruction {
            direction: Direction::try_from(dir_str)?,
            magnitude: magnitude,
        })
    }
}

fn main() {
    let mut depth: Int = 0;
    let mut forward: Int = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Failed to read stdin");
        let inst = Instruction::try_from(line.as_str()).expect("Failed to parse line");

        match inst.direction {
            Direction::DOWN => depth += inst.magnitude,
            Direction::UP => depth -= inst.magnitude,
            Direction::FORWARD => forward += inst.magnitude,
        };
    }

    println!("Depth: {}, Forward: {}", depth, forward);
    println!("Multiplied: {}", depth * forward);
}
