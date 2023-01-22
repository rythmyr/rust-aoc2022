use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let round_score = match line.as_str() {
                    // "win/loss" + "points for shape"
                    // "A X" => 3 + 1, // rock ties rock
                    // "A Y" => 6 + 2, // paper beats rock
                    // "A Z" => 0 + 3, // scissors loses to rock
                    // "B X" => 0 + 1, // etc.
                    // "B Y" => 3 + 2,
                    // "B Z" => 6 + 3,
                    // "C X" => 6 + 1,
                    // "C Y" => 0 + 2,
                    // "C Z" => 3 + 3,

                    // "win/loss" + "points for shape" (part 2)
                    "A X" => 0 + 3, // X = need loss, Scissors loses to rock
                    "A Y" => 3 + 1, // Y = need draw
                    "A Z" => 6 + 2,
                    "B X" => 0 + 1, // rock loses to paper
                    "B Y" => 3 + 2, // paper draws paper
                    "B Z" => 6 + 3, // scissors beats paper
                    "C X" => 0 + 2, 
                    "C Y" => 3 + 3,
                    "C Z" => 6 + 1,
                    _ => {
                        println!("invalid line, {}", line);
                        0
                    }
                };
                score += round_score;
            }
        }
    }
    println!("score: {}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}