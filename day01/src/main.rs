use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Something went wrong with this file");

    let split: Vec<i32> = contents
        .split_terminator("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("First part: {}", first(split.clone()));
    println!("Second part: {}", second(split.clone()));
}
fn first(input: Vec<i32>) -> i32 {
    let ans = input.iter().fold((input[0], 0), |acc, &x| {
        (x, if acc.0 < x { acc.1 + 1 } else { acc.1 })
    });
    ans.1
}
fn second(input: Vec<i32>) -> i32 {
    let zipped = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((x, y), z)| [*x, *y, *z])
        .collect::<Vec<_>>();

    let init = zipped[0].iter().sum::<i32>();

    let ans = zipped.iter().fold((init, 0), |acc, &x| {
        (
            x.iter().sum::<i32>(),
            if acc.0 < x.iter().sum::<i32>() {
                acc.1 + 1
            } else {
                acc.1
            },
        )
    });
    ans.1
}
