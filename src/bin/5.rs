use std::{collections::HashSet, fs::read_to_string};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (rules, updates) = read_file("inputs\\5");
    let mut sum = 0;
    'next_update: for update in updates.iter() {
        for rule in rules.iter() {
            if update.contains(&rule[0])
                && update.contains(&rule[1])
                && !update_is_valid_for_rule(rule, update)
            {
                // print_invalid_update(update, rule);
                continue 'next_update;
            }
        }
        // If the code reaches here, none of the rules invalidated the update
        sum += middle_value(update)
    }
    println!("{}", sum);
}

fn part2() {
    let (rules, updates) = read_file("inputs\\5");
    let invalid_updates: Vec<Vec<usize>> = updates
        .into_iter()
        .filter(|update| !update_is_valid(&rules, update))
        .collect();
    let mut sum = 0;
    println!("Number of invalid updates: {}", invalid_updates.len());
    for mut update in invalid_updates {
        let relevant_rules = get_relevant_rules(&rules, &update);
        let mut i = 1;
        while i < update.len() {
            let num = update[i];
            let numbers_after = get_numbers_after(&rules, num);
            let mut j = 0;
            while j < update.len() {
                if numbers_after.contains(&update[j]) {
                    insert_before(&mut update, i, j);
                    break;
                } else {
                    j = j + 1;
                }
            }
            i = i + 1;
        }
        let middle = middle_value(&update);
        println!("{}", middle);
        sum += middle;
    }
    println!("{}", sum);
}

fn insert_before(vector: &mut Vec<usize>, take_index: usize, insert_index: usize) {
    let value = vector.swap_remove(take_index);
    vector.insert(insert_index, value);
}

/**
 * Given a set of rules an a number, return a set of all the numbers that must be after the given number
 */
fn get_numbers_after(rules: &Vec<Vec<usize>>, num: usize) -> HashSet<usize> {
    rules
        .into_iter()
        .filter(|rule| rule[0] == num)
        .map(|rule| rule[1])
        .collect()
}

fn get_relevant_rules<'a>(
    all_rules: &'a Vec<Vec<usize>>,
    update: &Vec<usize>,
) -> Vec<&'a Vec<usize>> {
    return all_rules
        .into_iter()
        .filter(|rule| update.contains(&rule[0]) && update.contains(&rule[1]))
        .collect();
}

fn update_is_valid_for_rule(rule: &Vec<usize>, update: &Vec<usize>) -> bool {
    let rule: [usize; 2] = rule.clone().try_into().expect("Rule is not of length 2");
    let (before, after) = (rule[0], rule[1]);
    for page in update {
        if page == &before {
            return true;
        } else if page == &after {
            return false;
        }
    }
    return true;
}

fn update_is_valid(rules: &Vec<Vec<usize>>, update: &Vec<usize>) -> bool {
    return rules
        .into_iter()
        .all(|rule| update_is_valid_for_rule(rule, update));
}

fn middle_value(vec: &Vec<usize>) -> usize {
    return vec[vec.len() / 2];
}

fn read_file(path: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let lines: Vec<String> = read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect();
    let mut rules: Vec<Vec<usize>> = Vec::new();
    let mut i = 0;
    loop {
        let line: &String = &lines[i];
        if line == &String::from("") {
            i = i + 1; // Go to beginning of first update
            break;
        }
        rules.push(
            line.split('|')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        );
        i = i + 1;
    }

    let mut updates: Vec<Vec<usize>> = Vec::new();
    loop {
        let line: &String = &lines[i];
        updates.push(
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        );
        i = i + 1;
        if i == lines.len() {
            break;
        }
    }
    (rules, updates)
}

fn print_invalid_update(update: &Vec<usize>, rule: &Vec<usize>) {
    println!("Update: {:?}\nInvalidated due to rule {:?}", update, rule)
}

fn get_rules_with_first_num(rules: &Vec<Vec<usize>>, num: usize) -> Vec<[usize; 2]> {
    let mut return_vec: Vec<[usize; 2]> = Vec::new();
    for rule in rules {
        if rule[0] == num {
            let new_rule: [usize; 2] = [rule[0], rule[1]];
            return_vec.push(new_rule);
        }
    }
    return_vec
}
