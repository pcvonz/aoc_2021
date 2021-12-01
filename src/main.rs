use std::io::Error;
mod day_1;
use day_1::{day_1_part_2, day_1_part_1};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Ok(c) = config {
        let puzzle = c.puzzle.as_str();
        println!("{}", puzzle);
        match puzzle {
            "day_1_part_1" => day_1_part_1()?,
            "day_1_part_2" => day_1_part_2()?,
            _ => ()
        };
    }
    Ok(())
}

struct Config {
    puzzle: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let puzzles = vec!["day_1_part_1", "day_1_part_2"];

        let puzzle = args[1].clone();

        if !puzzles.iter().any(|p| {
            *p == puzzle.as_str()
        }) {
            println!("No puzzle found that matches");
            println!("Possible puzzles: \n{}", puzzles.join("\n"));
            return Err("No puzzle found that matches");
        };

        Ok(Config { puzzle })
    }
}
