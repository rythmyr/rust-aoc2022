use std::fs::File;
use std::io::{self, BufRead};
use std:: path::Path;

fn main() {
    let mut total = 0;
    let lower_a: i32 = Into::<i32>::into("a".as_bytes()[0]);
    let upper_a: i32 = Into::<i32>::into("A".as_bytes()[0]);

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let full_size = line.len();
            let compartment_size = full_size/2;
            let first_compartment = line[0..compartment_size].as_bytes();
            let second_compartment = line[compartment_size..full_size].as_bytes();

            'outer: for first_item_byte in first_compartment {
                let first_item: i32 = Into::<i32>::into(*first_item_byte);
                for second_item_byte in second_compartment {
                    let second_item: i32 = Into::<i32>::into(*second_item_byte);
                    if first_item == second_item {
                        let mut score = first_item - upper_a;
                        if score >= 26 {
                            score = first_item - lower_a;
                        }
                        else {
                            score += 26;
                        }

                        total += score + 1;
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("part 1: {}", total);
    total = 0;

    if let Ok(lines) = read_lines("input.txt") {
        let mut iter = lines.into_iter();
        
        loop {
            let line1 = iter.next();

            if let None = line1 {
                break;
            }

            let line1 = line1.unwrap();
            let line2 = iter.next().unwrap();
            let line3 = iter.next().unwrap();

            let line1_bytes = line1.as_bytes();
            let line2_bytes = line2.as_bytes();
            let line3_bytes = line3.as_bytes();

            'outer: for byte1 in line1_bytes {
                for byte2 in line2_bytes {
                    if byte1 == byte2 {
                        for byte3 in line3_bytes {
                            if byte1 == byte3 {
                                let mut score = i32::from(*byte1) - upper_a;
                                if score >= 26 {
                                    score = i32::from(*byte1) - lower_a;
                                }
                                else {
                                    score += 26;
                                }

                                total += score + 1;
                                
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 2: {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}