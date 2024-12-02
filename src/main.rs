use std::collections::HashMap;
use std::env::args;
use std::io::stdin;
use std::ops::Not;

fn main() {
    let arg_error_msg = "Pass valid day and part numbers, e.g. 1.1";
    match args().nth(1).expect(arg_error_msg).as_str() {
        "1.1" => d1_1(),
        "1.2" => d1_2(),
        "2.1" => d2_1(),
        "2.2" => d2_2(),
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
        let a = left_side[i];
        let b = right_side[i];
        sum += if a > b { a - b } else { b - a }
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
        .map(|l| {
            let mut l = l.split("   ");
            let left = l.next().unwrap().parse::<u128>().unwrap();
            let right = l.next().unwrap().parse::<u128>().unwrap();
            (left, right)
        })
        .for_each(|(left, right)| {
            left_side.push(left);
            *right_side.entry(right).or_default() += 1;
        });
    let sum = left_side
        .iter()
        .fold(0, |acc, x| acc + x * right_side.get(&x).unwrap_or(&0));
    println!("{}", sum);
}

fn d2_1() {
    let sum = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| d2_take_report(l))
        .filter(|r| d2_is_safe_report(r))
        .count();
    println!("{}", sum);
}

fn d2_take_report(l: String) -> Vec<u128> {
    l.split(' ').map(|l| l.parse::<u128>().unwrap()).collect()
}

fn d2_is_safe_report(report: &Vec<u128>) -> bool {
    let inc_dec_state = d2_increase_decrease_state(report[0], report[1]);
    let mut prev = report[0];
    for &current in report.iter().skip(1) {
        if d2_increase_decrease_state(prev, current) != inc_dec_state {
            return false;
        }
        let diff = prev.abs_diff(current);
        if d2_is_normal_diff(diff).not() {
            return false;
        }
        prev = current;
    }
    true
}

fn d2_is_normal_diff(d: u128) -> bool {
    match d {
        1..=3 => true,
        _ => false,
    }
}

fn d2_increase_decrease_state(a: u128, b: u128) -> (bool, bool) {
    let is_inc = a < b;
    let is_dec = a > b;
    (is_inc, is_dec)
}

fn d2_is_safe_report_with_df(report: &Vec<u128>) -> bool {
    if d2_is_safe_report(report) {
        return true;
    }
    for x in 0..report.len() {
        let mut safe_copy = report.clone();
        safe_copy.remove(x);
        if d2_is_safe_report(&safe_copy) {
            return true;
        }
    }
    return false;
}

fn d2_2() {
    let sum = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| d2_take_report(l))
        .filter(|r| d2_is_safe_report_with_df(r))
        .count();
    println!("{}", sum);
}
