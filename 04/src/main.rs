use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Write;
use std::io::stdout;
use std::path::Path;
use std::{thread, time};

const GRID_SIZE: usize = 139;
const INPUT_FILE: &str = "input.txt";
const ADJACENT: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

type Grid = [[bool; GRID_SIZE]; GRID_SIZE];

fn print_grid(grid: &Grid) {
    let mut cgrid: [[usize; 70]; 70] = [[0; 70]; 70];
    print!("{}[2J", 27 as char); //clear screen
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if grid[row][col] {
                cgrid[row / 2][col / 2] |= match (row % 2, col % 2) {
                    (0, 0) => 1,
                    (0, 1) => 2,
                    (1, 0) => 4,
                    (1, 1) => 8,
                    _ => panic!("that shouldn't happen"),
                };
            }
        }
    }
    let mut shape = String::from("");
    for row in 0..cgrid.len() {
        for col in 0..cgrid[row].len() {
            shape.push(match cgrid[row][col] {
                00 => ' ',
                01 => '▘',
                02 => '▝',
                03 => '▀',
                04 => '▖',
                05 => '▌',
                06 => '▞',
                07 => '▛',
                08 => '▗',
                09 => '▚',
                10 => '▜',
                11 => '▐',
                12 => '▄',
                13 => '▙',
                14 => '▟',
                15 => '█',
                _ => panic!("that shouldn't happen"),
            });
        }
        shape.push('\n');
    }
    print!("{shape}");
}

fn main() {
    if let Ok(lines) = read_input(INPUT_FILE) {
        let mut grid: Grid = [[false; GRID_SIZE]; GRID_SIZE];
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
        while !finished {
            finished = true; // unless we find more accessible rolls
            // find accessible rolls
            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    if !grid[row][col] {
                        continue; // not a roll
                    }
                    //println!("Checking roll at ({}, {})", row, col);
                    let mut blocking = 0;
                    for dir in ADJACENT {
                        let cell = (row as isize + dir.0, col as isize + dir.1);
                        if cell.0 < 0
                            || cell.0 >= GRID_SIZE as isize
                            || cell.1 < 0
                            || cell.1 >= GRID_SIZE as isize
                        {
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
            print_grid(&grid);
            stdout().flush();
            thread::sleep(time::Duration::from_millis(300));
        }
        println!("Accessible: {}", accessible);
    }
}

fn read_input<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
