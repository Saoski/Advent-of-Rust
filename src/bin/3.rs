use regex::Regex;
use std::fs::read_to_string;

fn read_file(path: &str) -> String {
    read_to_string(path).expect("Failed to read file")
}

fn part_1() {
    let binding = read_file("inputs\\3");
    let hay: &str = binding.as_str();
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: i32 = 0;
    for (_, [num1, num2]) in re.captures_iter(hay).map(|c| c.extract()) {
        sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

fn part_2() {
    let binding = read_file("inputs\\3");
    let hay = binding.as_str();
    let re: Regex = Regex::new(r"(mul\((\d+),(\d+)\)|do(|n't)()\(\))").unwrap();
    let mut sum = 0;
    let mut is_active = true;
    for (_, [capture, num1, num2]) in re.captures_iter(hay).map(|c| c.extract()) {
        if capture == "do()" {
            is_active = true;
        } else if capture == "don't()" {
            is_active = false;
        } else {
            if is_active {
                sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", sum)
}

fn main() {
    part_1();
    part_2();
}
