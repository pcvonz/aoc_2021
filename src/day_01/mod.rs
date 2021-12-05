use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn parse_text<'a>(buffer: std::io::Lines<BufReader<File>>) -> Vec<i32>{
    let mut depths: Vec<i32> = Vec::new();

    for depth in buffer {
        let d: i32 = depth.unwrap().parse().unwrap();
        depths.push(d);
    }
    depths
}

pub fn day_1_part_1() -> Result<(), Error> {
    let path = "input/day_1/part_1";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let d = parse_text(buffered.lines());
    let depths = d.as_slice();

    for window in depths.windows(2) {
        if let [first, second] = window {
            if  first < second {
                println!("{:?} {:?} Increase!", first, second);
            } else {
                println!("{:?} {:?} Decrease", first , second);
            }
        }
    }

    println!("");

    Ok(())
}

pub fn day_1_part_2() -> Result<(), Error> {
    let path = "input/day_1/part_1";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let d = parse_text(buffered.lines());
    let depths = d.as_slice();

    for window in depths.windows(4) {
        if let [first, second, third, fourth] = window {
            if  (first + second + third) < (second + third + fourth) {
                println!("{:?} {:?} Increase!", (first + second + third), ( second + third + fourth));
            } else {
                println!("{:?} {:?} Decrease", (first + second + third), (second + third + fourth));
            }
        }
    }

    println!("");

    Ok(())
}
