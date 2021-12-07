use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;

fn parse_text(path: String) -> Result<Vec<i64>, Error> {
    let input = File::open(path)?;
    
    let fish: Vec<i64> = BufReader::new(input).lines().map(|line| {
        if let Ok(l) = line {
            l.split(",").map(|fish| {
                fish.parse().expect("Could not parse int")
            }).collect()
        } else {
            vec![0 as i64]
        }
    }).flatten().collect();
  Ok(fish)
}

pub fn find_min_max(crabs: &Vec<i64>) -> (i64, i64) {
    (
        *crabs.iter().max().unwrap(), *crabs.iter().min().unwrap()
    )
}

pub fn part_1() -> Result<(), Error> {
  let crabs = parse_text(String::from("input/day_7/input"))?;
  let (max, min) = find_min_max(&crabs);

  let min = (min..max).into_iter().map(|to_pos| {
      crabs.iter().fold(0, |fuel_used, current_crab_pos| {
          fuel_used + (current_crab_pos - to_pos).abs()
      })
  }).min().unwrap();

  println!("{}", min);

  Ok(())
}


pub fn get_amount_of_fuel_used(to_pos: i64, crab_pos: i64) -> i64 {
  let amount_to_move = (crab_pos - to_pos).abs();
  let current_fuel_used: i64 = (1..amount_to_move + 1).sum();
  current_fuel_used
}

pub fn part_2() -> Result<(), Error> {
  let crabs = parse_text(String::from("input/day_7/input"))?;
  let (max, min) = find_min_max(&crabs);

  let min = (min..max).into_iter().map(|to_pos| {
      crabs.iter().fold(0, |fuel_used, current_crab_pos| {
          let current_fuel_used = get_amount_of_fuel_used(to_pos, *current_crab_pos);
          fuel_used + current_fuel_used

      })
  }).min().unwrap();

  println!("{}", min);

  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crab_test() {
        let test = get_amount_of_fuel_used(16, 5);
        let test_2 = get_amount_of_fuel_used(5, 14);
        let test_3 = get_amount_of_fuel_used(0, 5);
        assert_eq!(66, test);
        assert_eq!(45, test_2);
        assert_eq!(15, test_3);
    }
}
