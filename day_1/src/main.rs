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

    fn find_similarity(&self) -> i32 {
        let mut similarity = 0;
        let mut i = 0;
        let mut j = 0;

        while i < self.left_note.len() {
            let current = self.left_note[i];

            let mut count_left = 0;
            while i < self.left_note.len() && self.left_note[i] == current {
                count_left += 1;
                i += 1;
            }

            let mut count_right = 0;
            while j < self.right_note.len() && self.right_note[j] < current {
                j += 1;
            }
            while j < self.right_note.len() && self.right_note[j] == current {
                count_right += 1;
                j += 1;
            }

            similarity += current * count_left * count_right;
        }

        similarity
    }
    fn solve_part_1(&mut self) -> i32 {
        self.sort_notes();
        self.get_distances()
    }

    fn solve_part_2(&mut self) -> i32 {
        self.sort_notes();
        self.find_similarity()
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
    println!("Part 1 solution: {}", notes.solve_part_1());
    println!("Part 2 solution: {}", notes.solve_part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        let input = read_input("test_case.txt").unwrap();
        let mut notes = TwoNotes::new(input);
        assert_eq!(11, notes.solve_part_1());
    }

    #[test]
    fn test_solution_part_2() {
        let input = read_input("test_case.txt").unwrap();
        let mut notes = TwoNotes::new(input);
        assert_eq!(31, notes.solve_part_2());
    }
}
