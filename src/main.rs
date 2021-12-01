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

fn main() -> Result<(), Error> {
    let path = "input/day_1/part_1";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let d = parse_text(buffered.lines());
    let mut depths = d.iter().peekable();
    

    let mut first: i32 = *depths.next().unwrap();
    let mut second: i32 = *depths.next().unwrap();
    let mut third: i32 = *depths.next().unwrap();
    while let Some(fourth) = depths.next() {
        if  (first + second + third) < (second + third + fourth) {
            println!("{:?} {:?} Increase!", (first + second + third), ( second + third + fourth));
        } else {
            println!("{:?} {:?} Decrease", (first + second + third), (second + third + fourth));
        }
        first = second;
        second = third;
        third = *fourth;
    }

    println!("");

    Ok(())
}
