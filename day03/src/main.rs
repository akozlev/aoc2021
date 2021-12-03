use std::fs;
fn main() {
    let contents = fs::read_to_string("./input").expect("Something went wrong with this file");
    let bits: u32 = contents.split("\n").collect::<Vec<&str>>()[0].len() as u32;
    let split: Vec<Vec<char>> = contents
        .split_terminator("\n")
        .map(|x| x.chars().collect())
        .collect();

    let split1: Vec<u16> = contents
        .split_terminator("\n")
        .map(|x| to_decimal(x).expect("Not binary"))
        .collect();

    first_u16(&split1, bits);
    second_u16(&split1, bits);
}
fn first_u16(input: &Vec<u16>, bits: u32) {
    let res: Vec<u16> = input.clone();
    let mut gamma: u16 = 0;
    let leading = 8 * std::mem::size_of::<u16>() as u32 - bits;
    for i in (0..bits).rev() {
        let count = res.iter().fold((0, 0), |acc, &x| {
            if x & (1 << i) != 0 {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + 1, acc.1)
            }
        });
        if count.0 < count.1 {
            gamma += 2u16.pow(i);
        }
    }
    let ans: u64 = gamma as u64 * (!(gamma << leading) >> leading) as u64;
    println!("Answer one: {}", ans);
}
fn second_u16(input: &Vec<u16>, bits: u32) {
    let mut res: (Vec<u16>, Vec<u16>) = (input.clone(), input.clone());
    for i in (0..bits).rev() {
        if res.0.len() > 1 {
            let count = res.0.iter().fold((0, 0), |acc, &x| {
                if x & (1 << i) != 0 {
                    (acc.0, acc.1 + 1)
                } else {
                    (acc.0 + 1, acc.1)
                }
            });
            let common = count.0 > count.1;
            res.0 = res
                .0
                .into_iter()
                .filter(|&x| (x & (1 << i) != 0) == common)
                .collect();
        }
        if res.1.len() > 1 {
            let count = res.1.iter().fold((0, 0), |acc, &x| {
                if x & (1 << i) != 0 {
                    (acc.0, acc.1 + 1)
                } else {
                    (acc.0 + 1, acc.1)
                }
            });
            let common = count.0 <= count.1;
            res.1 = res
                .1
                .into_iter()
                .filter(|&x| (x & (1 << i) != 0) == common)
                .collect();
        }
    }
    println!("Answer two: {:?}", res.0[0] as u64 * res.1[0] as u64);
}
