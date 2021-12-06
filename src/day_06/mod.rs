use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;

fn parse_text(path: String) -> Result<Vec<Vec<u64>>, Error> {
    let input = File::open(path)?;
    
    let fish: Vec<Vec<u64>> = BufReader::new(input).lines().map(|line| {
        if let Ok(l) = line {
            l.split(",").map(|fish| {
                fish.parse().expect("Could not parse int")
            }).collect()
        } else {
            vec![0 as u64]
        }
    }).collect();
  Ok(fish)
}

pub fn advance_day(fish: &Vec<u64>) -> Vec<u64> {
    let mut new_fish: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    
    new_fish[0] = fish[1];
    new_fish[1] = fish[2];
    new_fish[2] = fish[3];
    new_fish[3] = fish[4];
    new_fish[4] = fish[5];
    new_fish[5] = fish[6] ;
    new_fish[6] = fish[7]+ fish[0];
    new_fish[7] = fish[8];
    new_fish[8] = fish[0];

    new_fish
}


// Sorts fish by amount of time left
pub fn sort_fish(fish: &Vec<u64>) -> Vec<u64> {
    let new_fish: Vec<u64> = Vec::new();

    (0..9).fold(new_fish, |mut sorted, days_left| {
        let filtered_fish: Vec<u64> = fish.iter().filter(|fish| {
            **fish == days_left
        }).map(|fish| {
            *fish
        }).collect();
        sorted.push(filtered_fish.len() as u64);
        sorted
    })
}

pub fn part_1() -> Result<(), Error> {
    let fish = parse_text(String::from("input/day_6/part_1"))?;
    let mut sorted_fish = sort_fish(&fish[0]);
    for _ in 0..256{
        sorted_fish = advance_day(&sorted_fish);
    }
    let count = sorted_fish.iter().fold(0, |c, f| {
        c + f
    });
    println!("{:?}", count);
  Ok(())
}

pub fn part_2() -> Result<(), Error> {
  Ok(())
}
