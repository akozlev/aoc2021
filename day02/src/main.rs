use std::fs;
fn main() {
    let contents = fs::read_to_string("./input").expect("Something went wrong with this file");

    let split: Vec<(&str, i32)> = contents
        .split_terminator("\n")
        .map(|x| x.split_terminator(" ").collect::<Vec<&str>>())
        .map(|x| (x[0], x[1].parse().unwrap()))
        .collect();

    println!("First part: {}", first(&split));
    println!("Second part: {}", second(&split));
}

fn first(input: &Vec<(&str, i32)>) -> i32 {
    let ans = input.iter().fold((0, 0), |acc, &(op, am)| match op {
        "forward" => (acc.0 + am, acc.1),
        "down" => (acc.0, acc.1 + am),
        "up" => (acc.0, acc.1 - am),
        _ => acc,
    });
    ans.0 * ans.1
}
fn second(input: &Vec<(&str, i32)>) -> i32 {
    let ans = input.iter().fold((0, 0, 0), |acc, &(op, am)| match op {
        "forward" => (acc.0 + am, acc.1 + acc.2 * am, acc.2),
        "down" => (acc.0, acc.1, acc.2 + am),
        "up" => (acc.0, acc.1, acc.2 - am),
        _ => acc,
    });
    ans.0 * ans.1
}
