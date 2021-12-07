use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;
use raylib::prelude::*;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    begin: Point,
    end: Point,
}

fn find_max_x_max_y(lines: &Vec<Line>)-> (i32, i32) {
    lines.iter().fold((0, 0), |(mut max_x, mut max_y), line| {
        if line.end.x > max_x {
            max_x = line.end.x;
        }
        if line.begin.x > max_x {
            max_x = line.begin.x;
        }
        if line.end.y > max_y {
            max_y = line.end.y;
        }
        if line.begin.y > max_y {
            max_y = line.begin.y;
        }
        
        (max_x, max_y)
    })
}

impl Grid {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        let mut grid: Vec<Vec<i32>> = Vec::new();
       for _ in 0..max_y {
           let mut row: Vec<i32> = Vec::new();
           for _ in 0..max_x {
               row.push(0);
           }
           grid.push(row);
       }
       Grid {
           grid 
       }
    }

    fn calculate_overlaps(self, threshold: i32) -> i32 {
        self.grid.iter().fold(0, |mut total, row| {
            total += row.iter().fold(0, |mut total_in_row, cell| {
                if *cell > threshold {
                    total_in_row += 1;
                } 
                total_in_row
            });
            total
        })
    }

    fn draw_horizontal_line(&mut self, line: Line) {
        let (begin,end) = if line.begin.x < line.end.x {
            (line.begin.x, line.end.x) 
        } else {
            (line.end.x, line.begin.x) 
        };

        for col in begin..end +1{
            self.grid[line.begin.y as usize][col as usize] += 1 ;
        }
    }

    fn draw_vertical_line(&mut self, line: Line) {
        let (begin,end) = if line.begin.y < line.end.y {
            (line.begin.y, line.end.y) 
        } else {
            (line.end.y, line.begin.y) 
        };

        for col in begin..end + 1 {
            self.grid[col as usize][line.begin.x as usize] += 1;
        }
    }

    pub fn draw_line(&mut self, line: Line) {
        if line.begin.x == line.end.x {
            self.draw_vertical_line(line);
        } else if line.begin.y == line.end.y {
            self.draw_horizontal_line(line);
        }
    }

    fn draw_diagonal_positive(&mut self, line: Line) {
        let (begin, end) =  if line.begin.x < line.end.x {
            (line.begin, line.end) 
        } else {
            (line.end, line.begin) 
        };

        let test = (begin.x..end.x+1).into_iter().zip((begin.y..end.y+1).into_iter());

        for ( row, col ) in test {
            println!("{}, {}", row, col);
            self.grid[col as usize][row as usize] += 1;
        };
    }

    fn draw_diagonal_negative(&mut self, line: Line) {
        let (begin, end) =  if line.begin.x > line.end.x {
            (line.begin, line.end) 
        } else {
            (line.end, line.begin) 
        };

        let test = (end.x..begin.x+1).into_iter().zip((begin.y..end.y+1).rev());
        for (row, col) in test {
            // println!("{}, {}", row, col);
            self.grid[col as usize][row as usize] += 1;
        };


    }

    pub fn draw_line_diagonal(&mut self, line: Line) {
        if line.begin.x == line.end.x {
            self.draw_vertical_line(line);
        } else if line.begin.y == line.end.y {
            self.draw_horizontal_line(line);
        } else if (line.begin.x + line.begin.y) == (line.end.x + line.end.y) {
            self.draw_diagonal_negative(line);
        } else {
            self.draw_diagonal_positive(line);
        }
    }
}

fn parse_text(path: String) -> Result<Vec<Line>, Error> {
    let input = File::open(path)?;
    let lines: Vec<Line> = BufReader::new(input).lines().map(|line| {
        if let Ok(l) = line {
            let points: Vec<i32> = l.split(" -> ").map(|point| {
                let x_y = point.split(",");
                let x_y: Vec<i32> = x_y.map(|x_y| {
                    let point: i32 = x_y.parse().expect("Couldn't convert point to i32");
                    point
                }).collect();

                x_y
            }).flatten().collect();

            if let [begin_x, begin_y, end_x, end_y] = points.as_slice() {
                Line {
                    begin: Point::new(*begin_x, *begin_y),
                    end: Point::new(*end_x, *end_y),
                }
            } else {
                Line {
                    begin: Point::new(0, 0),
                    end: Point::new(0, 0),
                }
            }

        } else {
            Line {
                begin: Point::new(0, 0),
                end: Point::new(0, 0),
            }
        }

    }).collect();
    Ok(lines)
}

pub fn part_1() -> Result<(), Error> {
  let lines = parse_text(String::from("input/day_5/example"))?;
  let (max_x, max_y) = find_max_x_max_y(&lines);
  let mut grid = Grid::new(max_x, max_y);

  for line in lines {
      grid.draw_line(line);
  }

  println!("{}", grid.calculate_overlaps(1));
  
  Ok(())
}

pub fn part_2() -> Result<(), Error> {
  let lines = parse_text(String::from("input/day_5/input"))?;
  let (max_x, max_y) = find_max_x_max_y(&lines);
  let mut grid = Grid::new(max_x+2, max_y+2);

  for line in lines {
      grid.draw_line_diagonal(line);
  }


  let (mut rl, thread) = raylib::init()
        .size(1000, 1000)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for (row_index, row) in grid.grid.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                let opacity = if *col > (u8::max_value() as i32) {
                    u8::max_value() as i32
                } else {
                    *col
                } * 70;
                let red = ( opacity * 30 ) as u8;
                let blue = ( opacity * 40 ) as u8;
                let green = (opacity * 60) as u8;
                d.draw_pixel(row_index as i32, col_index as i32, Color::new(red, green, blue, opacity as u8));
            }
        }
        unsafe {
            let c_name = std::ffi::CString::new("test.png").unwrap();
            raylib::ffi::TakeScreenshot(c_name.as_ptr());
        }

    }
  for grid in &grid.grid.clone() {
      println!("{:?}", grid);
  }


  println!("{}", &grid.clone().calculate_overlaps(1));
  
  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max() {
        let lines = vec![
            Line {
                begin: Point::new(0, 0), 
                end: Point::new(10, 10),
            },
            Line {
                begin: Point::new(5, 5), 
                end: Point::new(6, 6),
            }
        ];
        let (max_x, max_y) = find_max_x_max_y(&lines);
        assert_eq!((max_x, max_y), (10, 10))
        
    }

    #[test]
    fn parses() {
          let test = parse_text(String::from( "input/day_5/test_input" )).unwrap();
          assert_eq!(test[0].begin.x, 0);
          assert_eq!(test[0].begin.y, 9);
          assert_eq!(test[0].end.x, 5);
          assert_eq!(test[0].end.y, 9);
        
    }
    #[test]
    fn draws_horizontal_line() {
        let mut grid = Grid::new(11, 11);
        let line = Line {
            begin: Point::new(0, 0),
            end: Point::new(0, 10),
        };

        grid.draw_line(line);
        let mut count = 0;
        for row in 0..10 {
            count += grid.grid[row as usize][0];
        }
        println!("{:?}", grid);
        assert_eq!(count, 10);
    }
    #[test]
    fn draws_vertical_line() {
        let mut grid = Grid::new(11, 11);
        let line = Line {
            begin: Point::new(10, 0),
            end: Point::new(0, 0),
        };

        grid.draw_line(line);
        let mut count = 0;
        for row in 0..10 {
            count += grid.grid[0][row as usize];
        }
        println!("{:?}", grid);
        assert_eq!(count, 10);
    }
    #[test]
    fn calculates_overlaps() {
        let mut grid = Grid::new(11, 11);
        let line = Line {
            begin: Point::new(10, 0),
            end: Point::new(0, 0),
        };

        grid.draw_line(line.clone());
        grid.draw_line(line.clone());
        grid.draw_line(line.clone());
        assert_eq!(grid.calculate_overlaps(2), 11);
    }
    #[test]
    fn draws_diagonal_positive() {
        let mut grid = Grid::new(11, 11);
        let line = Line {
            begin: Point::new(1, 1),
            end: Point::new(4, 4),
        };

        grid.draw_line_diagonal(line.clone());
        grid.draw_line_diagonal(line.clone());
        grid.draw_line_diagonal(line.clone());
        grid.draw_line_diagonal(line.clone());
        assert_eq!(grid.calculate_overlaps(0), 4);
    }
    #[test]
    fn draws_diagonal_negative() {
        let mut grid = Grid::new(10, 10);
        let line = Line {
            begin: Point::new(7, 9),
            end: Point::new(9, 7),
        };

        grid.draw_line_diagonal(line);
        assert_eq!(grid.calculate_overlaps(0), 3);
    }
}
