use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;
use std::collections::HashMap;

fn parse_text(path: &str) -> Result<Vec<(Vec<String>, Vec<String>)>, Error> {
    let input = File::open(path)?;

    let all_data: Vec<(Vec<String>, Vec<String>)> = BufReader::new(input).lines().map(|line| {
        if let Ok(l) = line {
            let data: Vec<&str> = l.split(" | ").collect();
            if let [all_input, all_output] = *data.as_slice() {
                let all_input: Vec<String> = all_input.split(" ").map(|s| {
                    String::from(s)
                }).collect();
                let all_output: Vec<String> = all_output.split(" ").map(|s| {
                    String::from(s)
                }).collect();

                (all_input, all_output)
            } else {
            (vec![String::from("")], vec![String::from("")])
            }

        } else {
            (vec![String::from("")], vec![String::from("")])
        }
    }).collect();
    Ok(all_data)
}

pub fn part_1() -> Result<(), Error> {
  let text = parse_text("input/day_08/input")?;
  let count = text.iter().fold(0, |total_count, (_, output)| {
      total_count + output.iter().fold(0, |count, segment| {
          count + match segment.len() {
            2 => { 
                println!("{}", segment);
                1
            },
            3 => { 
                println!("{}", segment);
                1 },
            4 => { 
                println!("{}", segment);
                1 },
            5 => { 
                println!("{}", segment);
                1 },
            7 => { 
                println!("{}", segment);
                1 },
            _ => 0,
          }
      })
  });
  println!("{}", count);
  Ok(())
}

pub fn get_scrambled_value(output: &String, map: &HashMap<char, i32>) -> i32 {
    output.chars().fold(0, |accum, ch| {
        let e = map.get(&ch);
        accum + e.unwrap()
    })
}

pub fn part_2() -> Result<(), Error> {
  let text = parse_text("input/day_08/input")?;
  let result = text.iter().fold(0, |total_count, (input, output)| {
      let map = input.iter().fold(HashMap::new() as HashMap<char, i32>, |mut map, segment| {
          for segment in segment.chars() {
              *map.entry(segment).or_insert(0) += 1;
          }
          map
      });

      let mut value = String::new();
      for o in output {
          match get_scrambled_value(o, &map) {
            42 => value.push('0'),
            17 => value.push('1'),
            34 => value.push('2'),
            39 => value.push('3'),
            30 => value.push('4'),
            37 => value.push('5'),
            41 => value.push('6'),
            25 => value.push('7'),
            49 => value.push('8'),
            45 => value.push('9'),
            _ => {
                println!("{}", get_scrambled_value(&o, &map));
                  }
          }
      }
      let int_value: i32 = value.parse().unwrap();
      total_count + int_value
  });
  println!("{}", result);
  Ok(())
}
