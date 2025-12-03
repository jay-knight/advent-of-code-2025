use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::io;
use std::path::Path;

fn find_first_largest(digits: &str, leave: usize) -> (usize, usize) {
    let mut result = (0, 0);
    for (pos, char) in digits[..digits.len()-leave].chars().into_iter().enumerate() {
        let digit: usize = char.to_digit(10).expect("Could not parse digit") as usize;
        if digit > result.0 {
            result = (digit, pos);
        }
    }
    result
}

fn find_jolts(line: &str, batteries: usize) -> usize {
    let mut jolts = 0;
    let mut pos = 0;
    for i in 0..batteries {
        let (digit, rpos) = find_first_largest(&line[pos..], batteries-1-i);
        jolts = jolts * 10 + digit;
        pos += rpos + 1;
    }
    jolts
}

fn main() {
    let batteries = 12; // 2 for 1st star
    if let Ok(lines) = read_input("input.txt") {
        let mut jolt_sum = 0;
        for line in lines {
            if let Ok(line_str) = line {
                let jolt = find_jolts(&line_str, batteries);
                jolt_sum += jolt;
                println!("{} {} {}", line_str, jolt, jolt_sum);
            }
        }
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
    fn test_find_first_largest() {
        assert_eq!(find_first_largest("987654321111111", 1), (9, 0));
        assert_eq!(find_first_largest("12345", 0), (5, 4));
        assert_eq!(find_first_largest("12345", 1), (4, 3));
    }

    #[test]
    fn test_find_jolts() {
        assert_eq!(find_jolts("987654321111111", 2), 98);
        assert_eq!(find_jolts("811111111111119", 2), 89);
        assert_eq!(find_jolts("234234234234278", 2), 78);
        assert_eq!(find_jolts("818181911112111", 2), 92);

        assert_eq!(find_jolts("987654321111111", 12), 987654321111);
        assert_eq!(find_jolts("811111111111119", 12), 811111111119);
        assert_eq!(find_jolts("234234234234278", 12), 434234234278);
        assert_eq!(find_jolts("818181911112111", 12), 888911112111);
    }
}
