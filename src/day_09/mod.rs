use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;

fn parse_text(path: &str) -> Result<Vec<Vec<i32>>, Error> {
    let input = File::open(path)?;

    let all_data: Vec<Vec<i32>> = BufReader::new(input).lines().map(|line| {
        if let Ok(l) = line {
            l.chars().map(|point| {
                point.to_digit(10).expect("Not a number!") as i32
            }).collect()
        } else {
            panic!("Couldn't read input file")
        }
    }).collect();
  Ok(all_data)
}
pub fn find_low_points(map: Vec<Vec<i32>>) -> Vec<(i32, (usize, usize))> {
    let mut low_points = Vec::new();
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, point) in row.iter().enumerate() {
            let up_is_greater = if let Some(previous_row) = map.get(row_index - 1) {
                previous_row[col_index] > *point
            } else {
               true 
            };
            let below_is_greater = if let Some(previous_row) = map.get(row_index + 1) {
                previous_row[col_index] > *point
            } else {
                true
            };
            let left_is_greater = if let Some(previous_point) = row.get(col_index - 1) {
                 previous_point > point
            } else {
                true
            };
            let right_is_greater = if let Some(previous_point) = row.get(col_index + 1) {
                 previous_point > point
            } else {
                true
            };
            if up_is_greater && right_is_greater && left_is_greater && right_is_greater && below_is_greater {
                low_points.push((*point, (row_index, col_index)))
            }
        }
    };
    low_points
}

#[derive(Debug)]
pub struct Location {
    visited: bool,
    value: i32
}

impl Location {
    pub fn new(value: i32) -> Location {
        Location {
            visited: false,
            value
        }
    }
}

pub fn find_basins(map: &mut Vec<Vec<Location>>, basin: &mut Vec<i32>, location: (usize, usize)) {
    let (y, x) = location;
    map[y][x].visited = true;
    basin.push(map[y][x].value);

    if (x + 1) < map[0].len() && !map[y][x + 1].visited && map[y][x + 1].value != 9{
        find_basins(map, basin, (y, x + 1))
    } 
    if (x as i32 ) > 0 && !map[y][x - 1].visited && map[y][x -1].value != 9 {
        find_basins(map, basin, (y, x - 1))
    }
    if  (y + 1) < map.len() && !map[y + 1][x].visited && map[y + 1][x].value != 9{
        find_basins(map, basin, (y + 1, x))
    } if (y as i32 ) > 0 && !map[y - 1][x].visited && map[y - 1][x].value != 9 {
        find_basins(map, basin, (y - 1, x))
    } 
}

pub fn part_1() -> Result<(), Error> {
  let map = parse_text("input/day_09/input")?;
  let low_points = find_low_points(map);
  let res: i32 = low_points.iter().map(|(val, _)| {
      val
  }).sum();
  println!("{:?}", res + low_points.len() as i32);
  Ok(())
}


pub fn part_2() -> Result<(), Error> {
  let parsed = parse_text("input/day_09/input")?;
  let mut map: Vec<Vec<Location>> = parsed.iter().map(|row| {
      row.iter().map(|location| {
          Location::new(*location)
      }).collect()
  }).collect();
  let low_points = find_low_points(parsed);
  let mut basins: Vec<Vec<i32>> = Vec::new();
  for (_, location) in low_points {
    let mut basin: Vec<i32> = vec![];
    find_basins(&mut map, &mut basin, location);
    basins.push(basin.to_vec());
  }
  basins.sort_by(|a, b| {
      if a.len() > b.len() {
          Ordering::Less
      } else {
          Ordering::Greater
      }
  });
    println!("{:?}", basins[0].len() * basins[1].len() * basins[2].len());

  Ok(())
}
