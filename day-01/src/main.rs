use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn read_input() -> String {
    let path = Path::new("./src/input");
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    return contents;
}

fn process_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut lists: (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    let re = Regex::new(r"\s+")
        .expect("Regex should be valid");

    for line in input.split('\n') {
        let numbers: Vec<&str> = re.split(line).collect();
        if numbers.len() != 2 { continue; }
        lists.0.push(numbers[0].parse().expect("All entries should be a valid integer"));
        lists.1.push(numbers[1].parse().expect("All entries should be a valid integer"));
    }

    return lists;
}

fn solve_a(list_a: &mut Vec<i32>, list_b: &mut Vec<i32>) -> i32 {
    // Sort both lists, determine the distance between each pair of numbers POST sort,
    //  and then sum the distances.
    list_a.sort();
    list_b.sort();

    let mut sum: i32 = 0;

    for (i, val) in list_a.iter().enumerate() {
        sum += (val - list_b[i]).abs();
    }

    return sum;
}

fn occurrences_to_list(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut occurrences = HashMap::new();

    for item in list {
        occurrences.entry(*item).and_modify(|count| *count += 1).or_insert(1);
    }

    return occurrences;
}

fn solve_b(list_a: &Vec<i32>, list_b: &Vec<i32>) -> i32 {
    let mut similarity: i32 = 0;
    let occurrences = occurrences_to_list(list_b);

    for item in list_a {
        similarity += item * match occurrences.get(item) {
            Some(number) => number,
            None => &0,
        }
    }

    return similarity;
}

fn main() {
    let contents = read_input();

    let mut lists = process_input(contents);

    let result_a = solve_a(&mut lists.0, &mut lists.1);
    println!("{result_a}");

    let result_b = solve_b(&lists.0, &lists.1);
    println!("{result_b}");
}
