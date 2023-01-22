use std::fs::File;
use std::io::{self, BufRead};
use std:: path::Path;

fn main() {
    if let Ok(lines) = read_lines("simple.txt") {
        for line in lines {
            
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}
