use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::io;
use std::path::Path;

fn parse_ranges(line: &str) -> Vec<Vec<usize>> {
    line.split(",")
        .map(|s| s.to_string()
            .splitn(2, "-")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<usize>().expect("Could not parse range bound"))
            .collect()
        )
        .collect()
        //.map(|s| s.to_string().parse::<usize>())
        //.map(|r| (r.next(), r.next()))
}

fn number_is_valid(num: usize) -> bool {
    let s = num.to_string();
    let len = s.chars().count();
    //if len % 2 == 1 {
    //    return true;
    //}
    let half = len / 2;
    for size in 1..=half {
        if s.chars()
            .collect::<Vec<char>>()
            .chunks(size)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .windows(2)
            .all(|c| c[0] == c[1]) {
                return false

        }
    }
    true
}

fn main() {
    if let Ok(lines) = read_input("input.txt") {
        let mut invalid_sum = 0;
        for line in lines {
            if let Ok(line_str) = line {
                for range in parse_ranges(&line_str) {
                    println!("{:?}", range);
                    for i in range[0]..=range[1] {
                        if ! number_is_valid(i) {
                            invalid_sum += i;
                            println!("{}, {}", i, invalid_sum);
                        }
                    }
                }
            }
        }
        println!("Sum of invalid: {}", invalid_sum);
    }
}

fn read_input<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_is_valid() {
        assert_eq!(number_is_valid(11), false);
        assert_eq!(number_is_valid(12), true);
        assert_eq!(number_is_valid(100), true);
        assert_eq!(number_is_valid(1188511885), false);
        assert_eq!(number_is_valid(151515), false);
        assert_eq!(number_is_valid(151516), true);
    }
}
