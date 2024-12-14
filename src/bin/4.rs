use core::array::from_fn;
use std::fs::read_to_string;

const WORD: &str = "XMAS";

fn read_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn reverse_string(string: &String) -> String {
    let mut output: String = String::new();
    for character in string.chars().rev() {
        output.push(character);
    }
    output
}

fn string_from_indices<const N: usize>(
    data: &Vec<String>,
    i_indices: [usize; N],
    j_indices: [usize; N],
) -> Option<String> {
    let mut output = String::new();
    for i in 0..N {
        output.push(data.get(i_indices[i])?.chars().nth(j_indices[i])?);
    }
    Some(output)
}

fn check_horizontals(data: &Vec<String>) -> usize {
    let mut total_count: usize = 0;
    let word: String = String::from(WORD);
    for string in data {
        for i in 0..=string.len() - word.len() {
            let puzzle_word: String = String::from(&string[i..i + word.len()]);
            let reversed_puzzle_word: String = reverse_string(&puzzle_word);
            if puzzle_word == word || reversed_puzzle_word == word {
                total_count += 1;
            }
        }
    }
    total_count
}

fn check_verticles(data: &Vec<String>) -> usize {
    let mut total_count: usize = 0;
    let word: String = String::from(WORD);
    for i in 0..=data.len() - word.len() {
        for j in 0..data[0].len() {
            let i_indices: [usize; WORD.len()] = from_fn(|index| i + index);
            let j_indices: [usize; WORD.len()] = [j; WORD.len()];
            let puzzle_word = string_from_indices(data, i_indices, j_indices).unwrap();
            let reversed_puzzle_word = reverse_string(&puzzle_word);
            if puzzle_word == word || reversed_puzzle_word == word {
                total_count += 1;
            }
        }
    }
    total_count
}

fn check_rising_diagonals(data: &Vec<String>) -> usize {
    let mut total_count: usize = 0;
    let word: String = String::from(WORD);
    for i in word.len() - 1..data.len() {
        for j in 0..=data[0].len() - word.len() {
            let i_indices: [usize; WORD.len()] = from_fn(|index| i - index);
            let j_indices: [usize; WORD.len()] = from_fn(|index| j + index);
            let puzzle_word = string_from_indices(data, i_indices, j_indices).unwrap();
            let reversed_puzzle_word = reverse_string(&puzzle_word);
            if puzzle_word == word || reversed_puzzle_word == word {
                total_count += 1;
            }
        }
    }
    total_count
}

fn check_falling_diagonals(data: &Vec<String>) -> usize {
    let mut total_count: usize = 0;
    let word: String = String::from(WORD);
    for i in 0..=data.len() - word.len() {
        for j in 0..=data[0].len() - word.len() {
            let i_indices: [usize; WORD.len()] = from_fn(|index| i + index);
            let j_indices: [usize; WORD.len()] = from_fn(|index| j + index);
            let puzzle_word = string_from_indices(data, i_indices, j_indices).unwrap();
            let reversed_puzzle_word = reverse_string(&puzzle_word);
            if puzzle_word == word || reversed_puzzle_word == word {
                total_count += 1;
            }
        }
    }
    total_count
}

fn check_x(data: &Vec<String>) -> u32 {
    let mut total_count = 0;
    let word: String = String::from("MAS");
    for i in 1..data.len() - 1 {
        for j in 1..data[0].len() - 1 {
            let falling_diagonal: String =
                string_from_indices(data, [i - 1, i, i + 1], [j - 1, j, j + 1]).unwrap();
            let rising_diagonal: String =
                string_from_indices(data, [i + 1, i, i - 1], [j - 1, j, j + 1]).unwrap();
            if (falling_diagonal == word || reverse_string(&falling_diagonal) == word)
                && (rising_diagonal == word || reverse_string(&rising_diagonal) == word)
            {
                total_count += 1
            }
        }
    }
    total_count
}

fn part_1() {
    let data: Vec<String> = read_file("inputs\\4");
    let mut total_count = 0;
    total_count += check_horizontals(&data);
    total_count += check_verticles(&data);
    total_count += check_falling_diagonals(&data);
    total_count += check_rising_diagonals(&data);
    println!("{}", total_count);
}

fn part_2() {
    let data: Vec<String> = read_file("inputs\\4");
    println!("{}", check_x(&data))
}

fn main() {
    part_1();
    part_2();
}
