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
    for l in stdin().lines() {
        let l = l.unwrap();
        if l.is_empty() {
            break;
        }
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
    for l in stdin().lines() {
        let l = l.unwrap();
        if l.is_empty() {
            break;
        }
        let mut l = l.split("   ");
        let a = l.next().unwrap().parse::<u128>().unwrap();
        left_side.push(a);
        let b = l.next().unwrap().parse::<u128>().unwrap();
        *right_side.entry(b).or_default() += 1;
    }
    let mut sum = 0;
    for x in left_side {
        sum += x * right_side.get(&x).unwrap_or(&0);
    }
    println!("{}", sum);
}

fn d1_distance(a: u128, b: u128) -> u128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
