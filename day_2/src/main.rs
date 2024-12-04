use std::{fs::File, io::Read};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Trend {
    Increasing,
    Descreasing,
    Empty,
}

impl Trend {
    fn check_trend(&self, diff: i32) -> Trend {
        match self {
            Trend::Empty => {
                if diff > 0 {
                    Trend::Increasing
                } else {
                    Trend::Descreasing
                }
            }
            Trend::Increasing => {
                if diff > 0 {
                    Trend::Increasing
                } else {
                    Trend::Empty
                }
            }
            Trend::Descreasing => {
                if diff < 0 {
                    Trend::Descreasing
                } else {
                    Trend::Empty
                }
            }
        }
    }
}
#[derive(Debug)]
struct Report {
    reports: Vec<Vec<i32>>,
    number_of_safe_reports: usize,
}

impl Report {
    fn new(input: &str) -> Self {
        let number_of_safe_reports = 0;
        let reports = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().expect("Input is always valid"))
                    .collect()
            })
            .collect();
        Self {
            reports,
            number_of_safe_reports,
        }
    }

    fn is_safe(level_slice: &[i32], current_trend: &mut Trend) -> bool {
        let diff = level_slice[0] - level_slice[1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        let new_trend = current_trend.check_trend(diff);

        if *current_trend == Trend::Empty {
            *current_trend = new_trend;
            return true;
        }

        match *current_trend == new_trend {
            true => return true,
            false => return false,
        }
    }

    fn solve_part_1(&mut self) {
        for report in &self.reports {
            self.number_of_safe_reports += 1;
            let mut trend = Trend::Empty;
            for i in 0..report.len() - 1 {
                let level_slice = &report[i..i + 2];
                match Report::is_safe(level_slice, &mut trend) {
                    true => continue,
                    false => {
                        self.number_of_safe_reports -= 1;
                        break;
                    }
                };
            }
        }
    }
}
fn read_input(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let input = read_input("puzzle_input.txt").unwrap();
    let mut report = Report::new(&input);
    report.solve_part_1();
    println!("{}", report.number_of_safe_reports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = read_input("test_case.txt").unwrap();

        let mut report = Report::new(&input);
        report.solve_part_1();

        assert_eq!(2, report.number_of_safe_reports);
    }
}
