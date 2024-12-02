use std::collections::HashMap;
use std::env::args;
use std::io::stdin;

fn main() {
    let arg_error_msg = "Pass valid day and part numbers, e.g. 1.1";
    match args().nth(1).expect(arg_error_msg).as_str() {
        "1.1" => d1_1(),
        "1.2" => d1_2(),
        _ => panic!("{}", arg_error_msg),
    }
}

fn d1_1() {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();
    for l in stdin()
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
    {
        let mut l = l.split("   ");
        let a = l.next().unwrap().parse::<u128>().unwrap();
        left_side.push(a);
        let b = l.next().unwrap().parse::<u128>().unwrap();
        right_side.push(b);
    }
    let mut sum = 0;
    left_side.sort();
    right_side.sort();
    for i in 0..left_side.len() {
        sum += d1_distance(left_side[i], right_side[i]);
    }
    println!("{}", sum);
}

fn d1_2() {
    let mut left_side = Vec::new();
    let mut right_side = HashMap::<u128, u128>::new();
    stdin()
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| d1_tuple(l))
        .for_each(|(left, right)| {
            left_side.push(left);
            *right_side.entry(right).or_default() += 1;
        });
    let sum = left_side
        .iter()
        .fold(0, |acc, x| acc + x * right_side.get(&x).unwrap_or(&0));
    println!("{}", sum);
}

fn d1_tuple(l: String) -> (u128, u128) {
    let mut l = l.split("   ");
    let left = l.next().unwrap().parse::<u128>().unwrap();
    let right = l.next().unwrap().parse::<u128>().unwrap();
    (left, right)
}

fn d1_distance(a: u128, b: u128) -> u128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
