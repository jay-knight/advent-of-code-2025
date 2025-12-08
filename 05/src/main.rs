use std::cmp;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;
use std::ops::RangeInclusive;
use range_overlap;

fn main() {
    // read ranges
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    if let Ok(lines) = read_input("ranges.txt") {
        for line in lines {
            if let Ok(line_str) = line {
                let mut bounds = line_str.splitn(2, '-');
                ranges.push(RangeInclusive::new(
                        bounds.next().expect("bad split").parse::<u64>().expect("can't parse"),
                        bounds.next().expect("bad split").parse::<u64>().expect("can't parse"),
                ));
            }
        }
    }
    //println!("{:?}", ranges);

    // read ids
    let mut ids: Vec<u64> = Vec::new();
    if let Ok(lines) = read_input("ids.txt") {
        for line in lines {
            if let Ok(line_str) = line {
                ids.push(line_str.parse::<u64>().expect("can't parse"));
            }
        }
    }
    //println!("{:?}", ids);
    let count = ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();
    println!("First star count: {count}");

    // combine overlapping ranges
    ranges.sort_by(|a,b| a.start().cmp(b.start()));
    let mut curr = 0;
    while curr < ranges.len() {
        //println!("Len: {}", ranges.len());
        let mut other = curr + 1;
        while other < ranges.len() {
            if range_overlap::has_incl_overlap(
                ranges[curr].start(),
                ranges[curr].end(),
                ranges[other].start(),
                ranges[other].end(),
            ) {
                println!("{curr} and {other} overlap");
                //println!("{:?}", ranges[curr]);
                //println!("{:?}", ranges[other]);
                ranges[curr] = RangeInclusive::new(
                    *cmp::min(ranges[curr].start(), ranges[other].start()),
                    *cmp::max(ranges[curr].end(), ranges[other].end()),
                );
                //println!("{:?}", ranges[curr]);
                //println!("{}", ranges[curr].end() - ranges[curr].start());
                ranges.remove(other);
            } else {
                other += 1;
            }
        }
        println!("{:?}", ranges[curr]);
        curr += 1;
    }
    let all_count: u64 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("Second star count: {all_count}");
}

fn read_input<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_things() {
        let ri = RangeInclusive::new(1, 3);
        assert_eq!(ri.end() - ri.start() + 1, 3);
    }
}
