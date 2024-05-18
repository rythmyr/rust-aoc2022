use std::fs::File;
use std::io::{self, BufRead};
use std:: path::Path;

fn main() {
    let mut stacks: Vec<Vec<u8>> = vec!();

    if let Ok(lines) = read_lines("simple.txt") {
        let sections: Vec<_> = lines.split(|line| line.is_empty()).collect();

        if let Some(lines) = sections.get(0) {
            for line in *lines {
                println!("{}", line);
                let chars: Vec<_> = line.split(|char| {char == ' '}).collect();
            }
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
