use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std:: path::Path;

trait MyContains<T> where T: PartialOrd<T> {
    // naming this the same as the existing contains method doesn't work!
    // it just thinks there's no specialization for the Range type so it expects only a u8
    fn containss(&self, other: &Range<T>) -> bool;
    fn contains_single(&self, other: &T) -> bool;

    fn overlaps(&self, other: &Range<T>) -> bool;
}

impl<Idx> MyContains<Idx> for Range<Idx> where Idx: PartialOrd<Idx> {
    fn containss(&self, other: &Range<Idx>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn contains_single(&self, other: &Idx) -> bool {
        self.start <= *other && self.end >= *other
    }

    fn overlaps(&self, other: &Range<Idx>) -> bool {
        self.contains_single(&other.start) || self.contains_single(&other.end)
    }
}

fn main() {
    let mut count = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let times: Vec<u8> = line.split(",").flat_map(|l| {
                l.split("-").collect::<Vec<_>>()
            }).map(|s| {s.parse::<u8>().unwrap()}).collect();
            
            let range1: Range<u8> = times[0]..times[1];
            let range2: Range<u8> = times[2]..times[3];
            
            if range1.overlaps(&range2) || range2.overlaps(&range1) {
                count += 1;
                println!("It works! {:?}, {:?}", range1, range2);
            } else {
                println!("..........{:?}, {:?}", range1, range2);
            }
        }
    }

    println!("{}", count)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}