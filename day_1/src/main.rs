use std::{fs::File, io::Read};

#[derive(Debug)]
struct TwoNotes {
    left_note: Vec<i32>,
    right_note: Vec<i32>,
}

impl TwoNotes {
    fn new(input: String) -> Self {
        let mut left_note: Vec<i32> = Vec::new();
        let mut right_note: Vec<i32> = Vec::new();

        for line in input.lines() {
            let mut s = line.split("   ");

            let number: i32 = s
                .next()
                .expect("The input is always 2 items")
                .parse()
                .expect("The input is always a num");

            left_note.push(number);

            let number: i32 = s
                .next()
                .expect("The input is always 2 items")
                .parse()
                .expect("The input is always a num");

            right_note.push(number);
        }
        Self {
            left_note,
            right_note,
        }
    }

    fn sort_notes(&mut self) {
        self.left_note.sort_unstable();
        self.right_note.sort_unstable();
    }

    fn get_distances(&self) -> i32 {
        self.left_note
            .iter()
            .zip(self.right_note.iter())
            .map(|(l, r)| (l - r).abs())
            .sum()
    }

    fn solve(&mut self) -> i32 {
        self.sort_notes();
        self.get_distances()
    }
}

fn read_input(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let input = read_input("puzzle_input.txt").unwrap();
    let mut notes = TwoNotes::new(input);
    println!("{}", notes.solve());
}

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = read_input("test_case.txt").unwrap();
        let mut notes = TwoNotes::new(input);
        assert_eq!(11, notes.solve());
    }
}
