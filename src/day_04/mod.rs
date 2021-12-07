use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use regex::{ Regex, RegexBuilder };
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Visited,
    NotVisted
}

#[derive(Debug, Clone, PartialEq)]
enum BoardState {
    Winner,
    Loser
}

#[derive(Debug, Clone)]
pub struct Cell {
    value: i32,
    state: State,
    pos: Position
}

#[derive(Debug, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn new(value: i32, pos: Position) -> Cell {
        Cell {
            value,
            state: State::NotVisted,
            pos
        }
    }
}

#[derive(Clone)]
pub struct Board {
    cell: Vec<Vec<Cell>>,
    board_state: BoardState,
    round: usize,
    last_num: i32,
}

impl Board {
    pub fn find_marker(&mut self, value: i32) {
        let new_cells: Vec<Vec<Cell>> = self.cell.iter().map(|row| {
            row.iter().map(|c| {
                if c.value == value {
                    Cell {
                        value: c.value,
                        pos: Position {
                            x: c.pos.x,
                            y: c.pos.y
                        },
                        state: State::Visited
                    }
                } else {
                    c.clone()
                }
            }).collect()
        }).collect();

        self.last_num = value;
        self.cell = new_cells;
        
    }

    pub fn has_filled_column_or_row(&mut self, round: usize) -> bool {
        let filled_row = self.cell.iter().any(|row| {
            let has_unvisited = row.iter().any(|c| {
                c.state == State::NotVisted
            });
            !has_unvisited
        });

        let mut filled_column = false;
        let mut score_count = 0;
        for column in 0..5 {
            for row in 0..5 {
                if self.cell[row][column].state == State::Visited {
                    score_count += 1;
                }
            }
            if score_count == 5 {
                filled_column = true;
                break
            }
            score_count = 0;
        };

        if filled_column || filled_row {
            self.round = round;
            self.board_state = BoardState::Winner;
        }

        filled_column || filled_row
    }

    pub fn score(self) -> i32 {
        self.cell.iter().fold(0, |mut score, row| {
            let row_score = row.iter().fold(0, |mut s, c| {
                if c.state == State::NotVisted {
                    s += c.value;
                }
                s
            });
            score += row_score;
            score
        })
    }

    pub fn build_board_from_vec(cells: Vec<Vec<Cell>>) -> Board {
        // Set first value
        Board {
            cell: cells,
            board_state: BoardState::Loser,
            round: 0,
            last_num: 0
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("");
       let board: Vec<Vec<String>> = self.cell.iter().map(|lines| {
             lines.iter().map(|c| {
                 let cell_state: String;
                 if c.state == State::Visited {
                    cell_state = String::from("v");
                 } else {
                    cell_state = String::from("n");
                 }
                 let row = format!("{:2} {}", c.value, cell_state);
                 row
             }).collect()
         }).collect();
       let mut board_str = String::from("");

       for l in &board {
           let r = l.join(" ");
           board_str.push('\n');
           board_str.push_str(r.as_str());
       }
       board_str.push('\n');
        f.debug_struct("Board")
         .field("board:", &print!("{}", board_str))
         .field("round:", &self.round)
         .field("last_num:", &self.last_num)
         .finish()
    }
}


fn parse_text<'a>(path: String) -> Result<(Vec<i32>, Vec<Board>), Error> {
    let input = File::open(path)?;
    let mut lines = BufReader::new(input).lines();
    let re = Regex::new(r"\d+").expect("Incorrect regex!");
    let re_board_split = RegexBuilder::new(r"^\n")
        .multi_line(true)
        .build()
        .expect("Could not compile board split regex");

    // Blanke line
    let input_nums: Vec<i32> = lines
        .next()
        .expect("Could not get input lines")?
        .split(",").map(|num| {
            num.parse().expect("Could not parse input to number")
        }).collect();

    lines.next();
    let test: Vec<String> = lines.map(|l| {
        l.unwrap()
    }).collect();

    let full_string = &test.join("\n");

    let boards: Vec<Board> = re_board_split.split(&full_string).map(|board| {
        let cells: Vec<Vec<Cell>> = board.split("\n").enumerate().map(|(row, line)| {
            let beeop_boop: Vec<Cell> = re.find_iter(line).enumerate().map(|(column, c)| {
                let a: i32 = c.as_str().parse().unwrap();
                    Cell::new(a, Position {
                        x: column,
                        y: row
                    })
            }).collect();
            beeop_boop
        }).filter(|l| {
            l.len() > 0
        }).collect();

        Board::build_board_from_vec(cells)
    }).collect();

    Ok((input_nums, boards))
}

pub fn part_1_main() -> Result<(), Error> {
    part_1(String::from("input/day_4"))?;
    Ok(())
}

pub fn part_1(path: String) -> Result<i32, Error> {
    let (input, mut boards) = parse_text(path)?;
    let mut inputs = input.iter().peekable();

    for ( round, num ) in input.iter().enumerate() {
        for board in &mut boards {
            board.find_marker(*num);
        }

        let any_filled = boards.iter_mut().any(|b| {
            b.has_filled_column_or_row(round)
        });
        if any_filled {
            break
        }
        inputs.next();
    }
    

    let winning_boards: Vec<&Board> = boards.iter().filter(|b| {
        b.board_state == BoardState::Winner
    }).collect();

    println!("Winner! \n{:?}", winning_boards);

    let last_num = inputs.next().expect("No more inputs!");
    let score = winning_boards[0].clone().score();
    println!("Answer: {}", last_num * score);

    Ok(score)
}



pub fn part_2_main() -> Result<(), Error> {
    part_2(String::from("input/day_4"))?;
    Ok(())
}

pub fn part_2(path: String) -> Result<Board, Error> {
    let (input, mut boards) = parse_text(path)?;

    for (round, num ) in input.iter().enumerate() {
        let mut losing_boards: Vec<&mut Board> = boards.iter_mut().filter(|b| {
            b.board_state != BoardState::Winner
        }).collect();
        
        for board in losing_boards.iter_mut() {
            board.find_marker(*num);
        }

        for b in losing_boards {
            b.has_filled_column_or_row(round);
        };
    }
    

    let winning_boards: Vec<&Board> = boards.iter().filter(|b| {
        b.board_state == BoardState::Winner
    }).collect();

    let winning_board = winning_boards.iter().fold(winning_boards[0], |max, b| {
        if b.round > max.round {
            b
        } else {
            max
        }
    });
    let score = winning_board.clone().score();
    println!("winning_board: {:?}", winning_board);
    println!("Answer: {}", winning_board.last_num * score);

    Ok(winning_board.clone())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_part_1_row() {
        let value = part_1(String::from("input/day_4_test_row")).unwrap();
        assert_eq!(40, value)
    }
    #[test]
    fn passes_part_1_column() {
        let value = part_1(String::from("input/day_4_test_column")).unwrap();
        assert_eq!(40, value)
    }
    #[test]
    fn passes_part_2() {
        let value = part_2(String::from("input/day_4_example")).unwrap();
        let last_num = value.last_num;
        assert_eq!(13, last_num);
        assert_eq!(1924, last_num * value.score())
    }
}

