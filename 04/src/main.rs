use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::io;
use std::path::Path;

const grid_size:usize = 139;
const input_file: &str = "input.txt";
const adjacent: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

type Grid = [[bool; grid_size]; grid_size];

fn main() {
    println!("Hello, world!");
    if let Ok(lines) = read_input(input_file) {
        let mut grid: Grid = [[false; grid_size]; grid_size];
        // Read grid into 2d array
        for (row, line) in lines.enumerate() {
            if let Ok(line_str) = line {
                for (col, ch) in line_str.chars().enumerate() {
                    grid[row][col] = match ch {
                        '.' => false,
                        '@' => true,
                        _ => panic!("Invalid character"),
                    }
                }
                //println!("{:?}", grid);
            }
        }
        let mut finished = false;
        let mut accessible = 0;
        while ! finished {
            finished = true; // unless we find more accessible rolls
            // find accessible rolls
            for row in 0..grid_size {
                for col in 0..grid_size {
                    if ! grid[row][col] {
                        continue; // not a roll
                    }
                    println!("Checking roll at ({}, {})", row, col);
                    let mut blocking = 0;
                    for dir in adjacent {
                        let cell = (row as isize + dir.0, col as isize + dir.1);
                        if cell.0 < 0 || cell.0 >= grid_size as isize ||
                           cell.1 < 0 || cell.1 >= grid_size as isize {
                               continue; // out of bounds
                        }
                        if grid[cell.0 as usize][cell.1 as usize] {
                            blocking += 1;
                        }
                    }
                    if blocking < 4 {
                        accessible += 1; // count it
                        grid[row][col] = false; // remove it
                        finished = false;
                    }
                }
            }
        }
        println!("Accessible: {}", accessible);
    }
}

fn read_input<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
