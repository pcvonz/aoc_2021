use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn parse_text() -> Result<Vec<i32>, Error> {
    let path = "input/day_3";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let diagnostic = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];


    return Ok(buffered.lines().fold(diagnostic,
    |mut accum, line| {
        if let Ok(l) = line {
            for (index, bit) in l.chars().enumerate() {
                if bit == '0' {
                    accum[index as usize] -= 1;
                } else {
                    accum[index as usize] += 1;
                }

            }
            accum
        } else {
            accum
        }
    }))
}

fn get_most_common_bit(bits: &Vec<i32>, compare_func: &dyn Fn(i32) -> bool) -> i32 {
    let count = bits.iter().fold(0, |mut accum, bit| { 
        if *bit != 1 {
        accum += 1 
    } else {
        accum -= 1
    }
    accum
    });

    if compare_func(count) {
        0
    } else {
        1
    }
}

fn find_rating(mut lines: Vec<Vec<i32>>, 
    compare_func: &dyn Fn(i32, i32) -> bool,
    compare_common_bit_func: &dyn Fn(i32) -> bool,
    ) -> Vec<String> {
    for idx in 0..12 {
        let bits = lines.iter().map(|l| {
            l[idx]
        }).collect();
        let most_common_bit = get_most_common_bit(&bits, compare_common_bit_func);
        lines = lines.iter().filter(|l| {
            compare_func(l[idx], most_common_bit)
        }).map(|l| {
            l.to_vec()
        }).collect();

        if lines.len() == 1 {
            break
        }
    }

     lines[0].to_vec().iter().map(|b| b.to_string()).collect()
}

fn binary_string_to_deci(binary: String) -> isize {
    let intval = isize::from_str_radix(binary.as_str(), 2).expect("Couldn't convert to decimal");
    intval
}

fn parse_text_part_2(path: String) -> Result<isize, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines: Vec<Vec<i32>> = buffered.lines().map(|line| {
        let l = line.expect("Could not parse line!");
        let test: Vec<i32> = l.chars().map(|c| {
            let i: i32 = c.to_string().parse().expect("Can't parse int");
            i
        }).collect();
        test
    }).collect();

    let oxygen_generator = find_rating(lines.to_vec(), &|bit, most_common| {
        bit == most_common
    },  &|bit| {
        bit > 0
    });

    let co2_scrubber = find_rating(lines, &|bit, most_common| {
        bit != most_common
    },
    &|bit| {
        bit >= 1
    });

    let res = binary_string_to_deci(oxygen_generator.join(""))* &binary_string_to_deci(co2_scrubber.join(""));
    println!("Answer: {}", res);

     Ok(res)
}


pub fn part_1() -> Result<(), Error> {
    if let Ok(res) = parse_text() {
        let gamma = res.iter()
            .fold(String::from(""), |mut accum, bit| {
            if *bit < 0 {
                accum.push_str("0");
            } else {
                accum.push_str("1");
            }
            accum
        });
        let epsilon = res.iter()
            .fold(String::from(""), |mut accum, bit| {
            if *bit < 0 {
                accum.push_str("1");
            } else {
                accum.push_str("0");
            }
            accum
        });
        let intval = isize::from_str_radix(gamma.as_str(), 2).expect("Couldn't convert to decimal");
        let epsilon = isize::from_str_radix(epsilon.as_str(), 2).expect("Couldn't convert to decimal");
        println!("{}", intval * epsilon);
    } else {
        panic!("oh no!");
    }
    Ok(())
}

pub fn part_2() -> Result<(), Error> {
    parse_text_part_2(String::from("input/day_3"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_part_2_with_example_input() {
        let value = parse_text_part_2(String::from("input/test_day_3")).unwrap();
        assert_eq!(230, value)
    }
}

