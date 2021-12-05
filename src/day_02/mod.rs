use std::fs::File;
use std::io::{BufReader, BufRead, Error};

enum Direction {
    Forward,
    Down,
    Up
}

struct Movement {
    pub direction: Direction,
    pub amount: i32
}

#[derive(Debug)]
struct SubmarinePosition {
    pub horizontal: i32,
    pub depth: i32,
}

impl SubmarinePosition {
    pub fn new() -> SubmarinePosition {
        SubmarinePosition {
            horizontal: 0,
            depth: 0,
        }
    }
    pub fn rise(&mut self, amount: i32) {
        self.depth -= amount;
    }

    pub fn dive(&mut self, amount: i32) {
        self.depth += amount;
    }

    pub fn go_forward(&mut self, amount: i32) {
        self.horizontal += amount;
    }
}

#[derive(Debug)]
struct SubmarinePositionWithAim {
    pub horizontal: i32,
    pub aim: i32,
    pub depth: i32,
}

impl SubmarinePositionWithAim {
    pub fn new() -> SubmarinePositionWithAim {
        SubmarinePositionWithAim {
            horizontal: 0,
            aim: 0,
            depth: 0,
        }
    }
    pub fn rise(&mut self, amount: i32) {
        self.aim -= amount;
    }

    pub fn dive(&mut self, amount: i32) {
        self.aim += amount;
    }

    pub fn go_forward(&mut self, amount: i32) {
        self.horizontal += amount;
        self.depth += self.aim * amount;
    }
}

fn parse_text<'a>() -> Result<Vec<Option<Movement>>, Error> {
    let path = "input/day_2";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    return Ok(buffered.lines().map(|line| {
        if let Ok(l) = line {
            let s: Vec<&str> =  l.split(" ").collect();
            if let [dir, am] = s.as_slice() {
                let amount = am.parse().unwrap();
                match *dir {
                    "forward" => Some(Movement {
                        amount,
                        direction: Direction::Forward,
                    }),
                    "down" => Some(Movement {
                        amount,
                        direction: Direction::Down,
                    }),
                    "up" => Some(Movement {
                        amount,
                        direction: Direction::Up
                    }),
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    }).collect());
}

pub fn part_1() -> Result<(), Error> {
    let movements = parse_text()?;
    let mut pos = SubmarinePosition::new();
    for movement in movements {
        match movement {
            Some(Movement { direction: Direction::Up, amount: a} ) => pos.rise(a),
            Some(Movement { direction: Direction::Down, amount: a} ) => pos.dive(a),
            Some(Movement { direction: Direction::Forward, amount: a} ) => pos.go_forward(a),
            _ => ()
        }
    }

    println!("{:?}", pos.depth * pos.horizontal);

    Ok(())
}

pub fn part_2() -> Result<(), Error> {
    let movements = parse_text()?;
    let mut pos = SubmarinePositionWithAim::new();
    for movement in movements {
        match movement {
            Some(Movement { direction: Direction::Up, amount: a} ) => pos.rise(a),
            Some(Movement { direction: Direction::Down, amount: a} ) => pos.dive(a),
            Some(Movement { direction: Direction::Forward, amount: a} ) => pos.go_forward(a),
            _ => ()
        }
    }

    println!("{:?}", pos.depth * pos.horizontal);

    Ok(())
}

