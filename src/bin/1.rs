use std::cmp::Ordering;
use std::fs::read_to_string;

fn read_file(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let data: Vec<Vec<i32>> = read_to_string(filename)
        .expect("Error reading file")
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for line in data {
        vec1.push(line[0]);
        vec2.push(line[1]);
    }

    (vec1, vec2)
}

fn quick_sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }
    let pivot: i32 = list[list.len() / 2];
    let mut less: Vec<i32> = Vec::new();
    let mut equal: Vec<i32> = Vec::new();
    let mut greater: Vec<i32> = Vec::new();
    for int in list {
        match int.cmp(&pivot) {
            Ordering::Equal => equal.push(int),
            Ordering::Greater => greater.push(int),
            Ordering::Less => less.push(int),
        };
    }
    [quick_sort(less), equal, quick_sort(greater)].concat()
}

fn main() {
    let (left_list, right_list) = read_file("inputs\\1");
    for i in 0..left_list.len() {
        println!("{} {}", left_list[i], right_list[i]);
    }
    let sorted_left_list = quick_sort(left_list);
    let sorted_right_list = quick_sort(right_list);
    let mut sum: i32 = 0;
    for i in 0..sorted_left_list.len() {
        sum += (sorted_left_list[i] - sorted_right_list[i]).abs();
    }
    println!("{}", sum);
}
