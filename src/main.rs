use crate::D3FuncResult::{D3EmptyResult, D3NumberResult};
use is_odd::IsOdd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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
    let mut left_diagonal_lines: HashMap<i32, Vec<D4Char>> = HashMap::new();
    let mut right_diagonal_lines: HashMap<i32, Vec<D4Char>> = HashMap::new();
    let mut line_idx = 0;
    for l in stdin().lines().filter_map(|l| l.ok()) {
        let mut char_idx = 0;
        for c in l.chars() {
            left_diagonal_lines
                .entry(line_idx + char_idx)
                .or_default()
                .push(D4Char(line_idx, char_idx, c));
            right_diagonal_lines
                .entry(char_idx - line_idx)
                .or_default()
                .push(D4Char(line_idx, char_idx, c));
            char_idx += 1;
        }
        line_idx += 1;
    }
    let mut left_mas: HashMap<&D4Char, u128> = HashMap::new();
    left_diagonal_lines
        .values()
        .flat_map(|l| d4_2_collect_mas(l))
        .for_each(|mas| *left_mas.entry(mas.1).or_default() += 1);
    let mut right_mas: HashMap<&D4Char, u128> = HashMap::new();
    right_diagonal_lines
        .values()
        .flat_map(|l| d4_2_collect_mas(l))
        .for_each(|mas| *right_mas.entry(mas.1).or_default() += 1);
    let mut xmas_total = 0u128;
    for (p, left_count) in left_mas {
        let right_count = right_mas.get(&p).unwrap_or(&0);
        let xmas = left_count * right_count;
        xmas_total += xmas;
    }
    println!("{}", xmas_total);
}

fn d4_2_collect_mas(l: &Vec<D4Char>) -> Vec<(&D4Char, &D4Char, &D4Char)> {
    let mut vec = l
        .iter()
        .tuple_windows::<(_, _, _)>()
        .filter(|t| d4_2_is_mas(t))
        .collect_vec();
    vec.extend(
        l.iter()
            .rev()
            .tuple_windows::<(_, _, _)>()
            .filter(|t| d4_2_is_mas(t)),
    );
    vec
}

fn d4_2_is_mas(t: &(&D4Char, &D4Char, &D4Char)) -> bool {
    t.0 .2 == 'M' && t.1 .2 == 'A' && t.2 .2 == 'S'
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct D4Char(i32, i32, char);

fn d5_1() {
    let lines = stdin().lines().filter_map(|l| l.ok()).collect_vec();
    let rules = lines.iter().take_while(|l| !l.is_empty()).collect_vec();
    let mut rules_keeper = D5_1RulesKeeper::new();
    rules_keeper.learn(&rules);
    let mut sum = 0u128;
    for update_pages in lines.iter().skip(rules.len() + 1) {
        let update_pages = update_pages
            .split(",")
            .filter_map(|n| n.parse::<u128>().ok())
            .collect_vec();
        if !rules_keeper.is_ok_upd(&update_pages) {
            sum += update_pages[d5_1_middle(update_pages.len())];
        }
    }
    println!("{}", sum);
}

fn d5_1_middle(len: usize) -> usize {
    assert!(len.is_odd());
    (len - 1) / 2
}

struct D5_1RulesKeeper {
    page_to_prev_pages: HashMap<u128, HashSet<u128>>,
    page_to_next_pages: HashMap<u128, HashSet<u128>>,
    empty: HashSet<u128>,
}

impl D5_1RulesKeeper {
    fn new() -> Self {
        Self {
            page_to_prev_pages: HashMap::new(),
            page_to_next_pages: HashMap::new(),
            empty: HashSet::new(),
        }
    }

    fn learn(&mut self, rules: &Vec<&String>) {
        for r in rules {
            let (prev, next) = r
                .split("|")
                .filter_map(|n| n.parse::<u128>().ok())
                .collect_tuple()
                .unwrap();
            self.add_rule(prev, next);
        }
    }

    fn add_rule(&mut self, prev: u128, next: u128) {
        self.page_to_prev_pages
            .entry(next)
            .or_default()
            .insert(prev);
        self.page_to_next_pages
            .entry(prev)
            .or_default()
            .insert(next);
    }

    fn is_ok(&self, curr_page: u128, actual_prev_pages: &HashSet<u128>) -> bool {
        let should_be_next = self
            .page_to_next_pages
            .get(&curr_page)
            .unwrap_or(&self.empty);
        for prev in actual_prev_pages {
            if should_be_next.contains(prev) {
                return false;
            }
        }
        true
    }

    fn is_ok_upd(&self, upd: &Vec<u128>) -> bool {
        let mut actual_prev = HashSet::new();
        for &p in upd {
            if self.is_ok(p, &actual_prev) {
                actual_prev.insert(p);
                continue;
            }
            return false;
        }
        true
    }

    pub fn fix(&self, old_order: &Vec<u128>) -> Vec<u128> {
        let mut new_order = vec![];
        for x in old_order {
            self.insert_to_right_place(&mut new_order, *x);
        }
        new_order
    }

    fn insert_to_right_place(&self, new_order: &mut Vec<u128>, new_element: u128) {
        let should_be_prev = self
            .page_to_prev_pages
            .get(&new_element)
            .unwrap_or(&self.empty);
        let should_be_next = self
            .page_to_next_pages
            .get(&new_element)
            .unwrap_or(&self.empty);
        for index in 0..new_order.len() {
            let current_element = &new_order[index];
            if should_be_prev.contains(current_element) {
                continue;
            }
            if should_be_next.contains(current_element) {
                return new_order.insert(index, new_element);
            }
        }
        new_order.push(new_element)
    }
}

fn d5_2() {
    let lines = stdin().lines().filter_map(|l| l.ok()).collect_vec();
    let rules = lines.iter().take_while(|l| !l.is_empty()).collect_vec();
    let mut rules_keeper = D5_1RulesKeeper::new();
    rules_keeper.learn(&rules);
    let mut sum = 0u128;
    for old_order in lines.iter().skip(rules.len() + 1) {
        let old_order = old_order
            .split(",")
            .filter_map(|n| n.parse::<u128>().ok())
            .collect_vec();
        if rules_keeper.is_ok_upd(&old_order) {
            continue;
        }
        let new_order = rules_keeper.fix(&old_order);
        let middle_element = new_order[d5_1_middle(old_order.len())];
        sum += middle_element;
    }
    println!("{}", sum);
}
