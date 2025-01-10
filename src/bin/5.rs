use std::fs::read_to_string;

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
                && !update_is_valid(rule, update)
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

fn part2() {}

fn update_is_valid(rule: &Vec<usize>, update: &Vec<usize>) -> bool {
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
