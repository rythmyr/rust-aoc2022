use std::fs::File;
use std::io::{self, BufRead};
use std:: path::Path;

const NUM_MAX_ELVES: usize = 3;

fn main() {
    let mut elves_list = Vec::<u32>::new();
    let mut current_cals = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line == "".to_string() {
                    elves_list.push(current_cals);
                    current_cals = 0;
                } else {
                    if let Ok(cals) = line.parse::<u32>() {
                        current_cals += cals;
                    }
                }
            }
        }
    }

    elves_list.sort_unstable();
    elves_list.reverse();

    let mut max_total = 0;
    let mut count = 0;

    for i in 0..NUM_MAX_ELVES {
        max_total += elves_list[i];
        count += 1;
    }

    println!("{}", max_total);
    println!("{}", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}