use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::io;
use std::path::Path;


#[derive(Clone, Copy, Debug, PartialEq)]
struct Counts {
    position: i64,
    landed_on_zero: i64,
    pointed_at_zero: i64,
}

impl Counts {
    fn new(position: i64) -> Self {
        Self {
            position: position,
            pointed_at_zero: 0,
            landed_on_zero: 0,
        }
    }
    fn rotate(self: Self, rotation: Rotation) -> Self {
        let mut next = self;
        // I don't like that I had to loop this.
        match rotation.direction {
                Direction::Left  => {
                    next.position -= rotation.distance;
                    next.pointed_at_zero += match (self.position, next.position) {
                        (_, 1..) => 0,
                        (1.., 0) => 1,
                        (0, _) => -next.position / 100,
                        _ => -next.position / 100 + 1,
                    };
                    next.position = next.position.rem_euclid(100);
                },
                Direction::Right => {
                    next.position += rotation.distance;
                    next.pointed_at_zero += next.position / 100;
                    next.position = next.position % 100;
                }
        };
        if next.position == 0 {
            next.landed_on_zero += 1;
        }
        next
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}

struct Rotation {
    direction: Direction,
    distance: i64
}

impl Rotation {
    fn from(line: &str) -> Self {
        Self {
            direction: match &line[0..1] {
                "L" => Direction::Left,
                "R" => Direction::Right,
                &_ => todo!(),
            },
            distance: line[1..].parse::<i64>().expect("Failed to parse line")
        }
    }
}


fn main() {
    if let Ok(lines) = read_input("input.txt") {
        let mut current = Counts::new(50);
        for line in lines {
            if let Ok(line_str) = line {
                let rotation = Rotation::from(&line_str);
                current = current.rotate(rotation);
                println!("{} -> {} (landed on 0: {}. pointed at 0: {})", line_str, current.position, current.landed_on_zero, current.pointed_at_zero);
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
    fn test_rotation_from() {
        assert_eq!(1 / -100, 0);
        assert_eq!(-1 / -100, 0);
        assert_eq!(-101 / -100, 1);
        let rotation = Rotation::from("L1");
        assert_eq!(rotation.direction, Direction::Left);
        assert_eq!(rotation.distance, 1);

        assert_eq!(Counts::new(50).rotate(Rotation::from("L25")), Counts{position: 25, landed_on_zero: 0, pointed_at_zero: 0}, "50 -> L25");
        assert_eq!(Counts::new(50).rotate(Rotation::from("R25")), Counts{position: 75, landed_on_zero: 0, pointed_at_zero: 0}, "50 -> R25");
        assert_eq!(Counts::new(50).rotate(Rotation::from("R50")), Counts{position: 0, landed_on_zero: 1, pointed_at_zero: 1}, "50 -> R50");
        assert_eq!(Counts::new(50).rotate(Rotation::from("L50")), Counts{position: 0, landed_on_zero: 1, pointed_at_zero: 1}, "50 -> L50");
        assert_eq!(Counts::new(50).rotate(Rotation::from("L68")), Counts{position: 82, landed_on_zero: 0, pointed_at_zero: 1}, "50 -> L68");
        assert_eq!(Counts::new(50).rotate(Rotation::from("L100")), Counts{position: 50, landed_on_zero: 0, pointed_at_zero: 1}, "50 -> L100");
        assert_eq!(Counts::new(0).rotate(Rotation::from("L99")), Counts{position: 1, landed_on_zero: 0, pointed_at_zero: 0}, "0 -> L99");
        assert_eq!(
            Counts::new(50)
                .rotate(Rotation::from("L68"))
                .rotate(Rotation::from("L30"))
                .rotate(Rotation::from("R48"))
                .rotate(Rotation::from("L5"))
                .rotate(Rotation::from("R60"))
                .rotate(Rotation::from("L55"))
                .rotate(Rotation::from("L101"))
                .rotate(Rotation::from("L99"))
                .rotate(Rotation::from("R14"))
                .rotate(Rotation::from("L82"))
                ,
            Counts {
                position: 32,
                landed_on_zero: 3,
                pointed_at_zero: 7
            }
        );
    }
}
