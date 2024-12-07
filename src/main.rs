use crate::D3FuncResult::{D3EmptyResult, D3NumberResult};
use itertools::Itertools;
use std::collections::HashMap;
use std::env::args;
use std::io::stdin;
use std::ops::Not;
use D3FuncResult::{D3Disable, D3Enable};

fn main() {
    let arg_error_msg = "Pass valid day and part numbers, e.g. 1.1";
    match args().nth(1).expect(arg_error_msg).as_str() {
        "1.1" => d1_1(),
        "1.2" => d1_2(),
        "2.1" => d2_1(),
        "2.2" => d2_2(),
        "3.1" => d3_1(),
        "3.2" => d3_2(),
        "4.1" => d4_1(),
        "4.2" => d4_2(),
        "5.1" => d5_1(),
        "5.2" => d5_2(),
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
    false
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

fn d3_1() {
    let sum: u128 = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| d3_interpreter(l, false))
        .sum();
    println!("{}", sum);
}

fn d3_interpreter(line: String, conditions_enabled: bool) -> u128 {
    let mut func = String::new();
    let mut params = String::new();
    let mut sum = 0u128;
    let mut enabled = true;
    for c in line.chars() {
        if c == '(' {
            if func.is_empty() {
                continue;
            }
            if params.is_empty() {
                params.push(c);
                continue;
            }
            func.clear();
            params.clear();
            continue;
        }
        if c.is_numeric() {
            if params.is_empty() {
                func.push(c);
            } else {
                params.push(c);
            }
            continue;
        }
        if c == ',' {
            if func.is_empty() {
                continue;
            }
            if params.is_empty() {
                continue;
            }
            if params.ends_with(',') {
                func.clear();
                params.clear();
                continue;
            }
            params.push(c);
            continue;
        }
        if c == ')' {
            if func.is_empty() {
                continue;
            }
            params.push(c);
            match d3_try_execute_function(&func, &params) {
                D3Enable => enabled = true,
                D3Disable => enabled = false,
                D3NumberResult(number) => {
                    if conditions_enabled.not() || enabled {
                        sum += number;
                    }
                }
                _ => {}
            }
            func.clear();
            params.clear();
            continue;
        }
        if params.is_empty() {
            func.push(c);
        } else {
            func.clear();
            params.clear();
            func.push(c);
        }
    }
    if func.is_empty().not() && params.is_empty().not() {
        if let D3NumberResult(number) = d3_try_execute_function(&func, &params) {
            if conditions_enabled.not() || enabled {
                sum += number;
            }
        }
    }
    sum
}

enum D3FuncResult {
    D3Enable,
    D3Disable,
    D3EmptyResult,
    D3NumberResult(u128),
}

fn d3_try_execute_function(func: &str, params: &str) -> D3FuncResult {
    if params == "()" {
        if func.ends_with("do") {
            return D3Enable;
        }
        if func.ends_with("don't") {
            return D3Disable;
        }
        return D3EmptyResult;
    }
    if func.ends_with("mul") {
        let params = params
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>();
        if params.len() != 2 {
            return D3EmptyResult;
        }
        let mut mul = 1u128;
        for p in params {
            if p.len() > 3 {
                return D3EmptyResult;
            }
            if let Ok(p) = p.parse::<u128>() {
                mul *= p;
            } else {
                return D3EmptyResult;
            }
        }
        return D3NumberResult(mul);
    }
    D3EmptyResult
}

fn d3_2() {
    let line = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", d3_interpreter(line, true));
}

fn d4_1() {
    let mut horizontal_lines: Vec<String> = vec![];
    let mut vertical_lines: HashMap<i32, String> = HashMap::new();
    let mut left_diagonal_lines: HashMap<i32, String> = HashMap::new();
    let mut right_diagonal_lines: HashMap<i32, String> = HashMap::new();
    let unsorted_lines = stdin().lines().filter_map(|l| l.ok());
    let mut line_idx = 0;
    for l in unsorted_lines {
        let mut char_idx = 0;
        for c in l.chars() {
            vertical_lines.entry(char_idx).or_default().push(c);
            left_diagonal_lines
                .entry(line_idx + char_idx)
                .or_default()
                .push(c);
            right_diagonal_lines
                .entry(char_idx - line_idx)
                .or_default()
                .push(c);
            char_idx += 1;
        }
        horizontal_lines.push(l);
        line_idx += 1;
    }
    let xmas = horizontal_lines
        .iter()
        .map(|l| d4_1_count_xmas(l))
        .sum::<u128>()
        + vertical_lines
            .values()
            .map(|l| d4_1_count_xmas(l))
            .sum::<u128>()
        + left_diagonal_lines
            .values()
            .map(|l| d4_1_count_xmas(l))
            .sum::<u128>()
        + right_diagonal_lines
            .values()
            .map(|l| d4_1_count_xmas(l))
            .sum::<u128>();
    println!("{}", xmas);
}

fn d4_1_count_xmas(l: &String) -> u128 {
    l.chars()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|t| d4_1_is_xmas(t))
        .count() as u128
        + l.chars()
            .rev()
            .tuple_windows::<(_, _, _, _)>()
            .filter(|t| d4_1_is_xmas(t))
            .count() as u128
}

fn d4_1_is_xmas(t: &(char, char, char, char)) -> bool {
    t == &('X', 'M', 'A', 'S')
}

fn d4_2() {
    todo!()
}

fn d5_1() {
    todo!()
}

fn d5_2() {
    todo!()
}
