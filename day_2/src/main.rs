use std::{fs::File, io::Read};

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
}

impl Report {
        fn new(input: &str) -> Self {
        let reports = input
            .lines()
            .map(|line| line.split_whitespace()
                            .map(|num| num.parse::<i32>().expect("Input is always valid"))
                            .collect())
            .collect();
        Self { reports }
    }
    fn solve_part_1(&self) -> u16 {
        let mut number_of_safe_reports: u16 = 0;

        for report in &self.reports {
            let mut trend = Trend::Empty;
            number_of_safe_reports += 1;
            for i in 0..report.len() - 1 {
                let diff = report[i] - report[i + 1];
                if !(1 <= diff.abs() && diff.abs() <= 3) {
                    number_of_safe_reports -= 1;
                    break;
                }

                trend = trend.check_trend(diff);
                match trend {
                    Trend::Empty => {
                        number_of_safe_reports -= 1;
                        break;
                    }
                    _ => continue,
                }
            }
        }

        number_of_safe_reports
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
    let report = Report::new(&input);
    println!("{}", report.solve_part_1());
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = read_input("test_case.txt").unwrap();

        let report = Report::new(&input);
        assert_eq!(2, report.solve_part_1());
    }
}
