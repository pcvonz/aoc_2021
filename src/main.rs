use std::io::Error;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
use std::env;

// Yes I know this is silly
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Ok(c) = config {
        let puzzle = c.puzzle.as_str();
        println!("{}", puzzle);
        match puzzle {
            "day_1_part_1" => day_01::day_1_part_1()?,
            "day_1_part_2" => day_01::day_1_part_2()?,
            "day_2_part_1" => day_02::part_1()?,
            "day_2_part_2" => day_02::part_2()?,
            "day_3_part_1" => day_03::part_1()?,
            "day_3_part_2" => day_03::part_2()?,
            "day_4_part_1" => day_04::part_1_main()?,
            "day_4_part_2" => day_04::part_2_main()?,
            "day_5_part_1" => day_05::part_1()?,
            "day_5_part_2" => day_05::part_2()?,
            "day_6_part_1" => day_06::part_1()?,
            "day_6_part_2" => day_06::part_2()?,
            "day_7_part_1" => day_07::part_1()?,
            "day_7_part_2" => day_07::part_2()?,
            "day_8_part_1" => day_08::part_1()?,
            "day_8_part_2" => day_08::part_2()?,
            "day_9_part_1" => day_09::part_1()?,
            "day_9_part_2" => day_09::part_2()?,
            "day_10_part_1" => day_10::part_1()?,
            "day_10_part_2" => day_10::part_2()?,
            "day_11_part_1" => day_11::part_1()?,
            "day_11_part_2" => day_11::part_2()?,
            "day_12_part_1" => day_12::part_1()?,
            "day_12_part_2" => day_12::part_2()?,
            "day_13_part_1" => day_13::part_1()?,
            "day_13_part_2" => day_13::part_2()?,
            "day_14_part_1" => day_14::part_1()?,
            "day_14_part_2" => day_14::part_2()?,
            "day_15_part_1" => day_15::part_1()?,
            "day_15_part_2" => day_15::part_2()?,
            "day_16_part_1" => day_16::part_1()?,
            "day_16_part_2" => day_16::part_2()?,
            "day_17_part_1" => day_17::part_1()?,
            "day_17_part_2" => day_17::part_2()?,
            "day_18_part_1" => day_18::part_1()?,
            "day_18_part_2" => day_18::part_2()?,
            "day_19_part_1" => day_19::part_1()?,
            "day_19_part_2" => day_19::part_2()?,
            "day_20_part_1" => day_20::part_1()?,
            "day_20_part_2" => day_20::part_2()?,
            "day_21_part_1" => day_21::part_1()?,
            "day_21_part_2" => day_21::part_2()?,
            "day_22_part_1" => day_22::part_1()?,
            "day_22_part_2" => day_22::part_2()?,
            "day_23_part_1" => day_23::part_1()?,
            "day_23_part_2" => day_23::part_2()?,
            "day_24_part_1" => day_24::part_1()?,
            "day_24_part_2" => day_24::part_2()?,
            "day_25_part_1" => day_25::part_1()?,
            "day_25_part_2" => day_25::part_2()?,
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
        let puzzles = vec![
            "day_1_part_1",
            "day_1_part_2", 
            "day_2_part_1", 
            "day_2_part_2",
            "day_3_part_2",
            "day_3_part_1",
            "day_4_part_2",
            "day_4_part_1",
            "day_5_part_1",
            "day_5_part_2",
            "day_6_part_1",
            "day_6_part_2",
            "day_7_part_1",
            "day_7_part_2",
            "day_8_part_1",
            "day_8_part_2",
            "day_9_part_1",
            "day_9_part_2",
            "day_10_part_1",
            "day_10_part_2",
            "day_11_part_1",
            "day_11_part_2",
            "day_12_part_1",
            "day_12_part_2",
            "day_13_part_1",
            "day_13_part_2",
            "day_14_part_1",
            "day_14_part_2",
            "day_15_part_1",
            "day_15_part_2",
            "day_16_part_1",
            "day_16_part_2",
            "day_17_part_1",
            "day_17_part_2",
            "day_18_part_1",
            "day_18_part_2",
            "day_19_part_1",
            "day_19_part_2",
            "day_20_part_1",
            "day_20_part_2",
            "day_21_part_1",
            "day_21_part_2",
            "day_22_part_1",
            "day_22_part_2",
            "day_23_part_1",
            "day_23_part_2",
            "day_24_part_1",
            "day_24_part_2",
            "day_25_part_1",
            "day_25_part_2",
        ];

        let puzzle = args.get(1);

        if let Some(puz) = puzzle {
            if !puzzles.iter().any(|p| {
                *p == puz.as_str()
            }) {
                println!("No puzzle found that matches");
                println!("Possible puzzles: \n{}", puzzles.join("\n"));
                return Err("No puzzle found that matches");
            };

            Ok(Config { puzzle: puz.to_string() })
        } else {
            println!("Possible puzzles: \n{}", puzzles.join("\n"));
            Err("No input!")
        }

    }
}
