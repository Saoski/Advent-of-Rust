use std::cmp::Ordering;
use std::fs::read_to_string;

fn read_file(filename: &str) -> Vec<Vec<usize>> {
    read_to_string(filename)
        .expect("Error opening file")
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<usize>) -> bool {
    // Determine if data is increasing or decreasing
    let is_increasing: bool = match report[0].cmp(&report[1]) {
        Ordering::Less => true,
        Ordering::Greater => false,
        _ => false,
    };
    let mut num1: usize = report[0];
    let mut difference: usize;
    for num2 in report[1..].iter() {
        match num1.cmp(num2) {
            Ordering::Less => {
                if is_increasing {
                    difference = num2 - num1;
                } else {
                    return false;
                }
            }
            Ordering::Equal => return false,
            Ordering::Greater => {
                if !is_increasing {
                    difference = num1 - num2;
                } else {
                    return false;
                }
            }
        }
        if !(1..=3).contains(&difference) {
            return false;
        }
        num1 = *num2;
    }
    true
}

fn is_safe2(report: &Vec<usize>) -> bool {
    if is_safe(report) {
        return true;
    }
    let local_report = report.to_owned();
    for i in 0..local_report.len() {
        let mut popped_report = local_report.clone();
        popped_report.remove(i);
        if is_safe(&popped_report) {
            return true;
        }
    }
    false
}

fn part1() {
    let data: Vec<Vec<usize>> = read_file("inputs\\2");
    let total_safe: usize = data.into_iter().filter(|x| is_safe(x)).count();
    println!("{}", total_safe)
}

fn part2() {
    let data: Vec<Vec<usize>> = read_file("inputs\\2");
    let total_safe: usize = data.into_iter().filter(|x| is_safe2(x)).count();
    println!("{}", total_safe)
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&vec![0, 1, 3, 4, 7]));
    }

    #[test]
    fn test_is_safe2() {
        assert!(is_safe2(&vec![0, 1, 3, 4, 7]));
        assert!(is_safe2(&vec![80, 83, 84, 86, 87, 90, 91, 92]))
    }
}
