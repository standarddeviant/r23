use std::iter::zip;
use regex::Regex;
use crate::utils::{read_lines, parse_digs};

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    println!("day06, part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    println!("day06, part2: answer = {part2_answer}");
}

fn parse_races(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    assert!(lines.len() >= 2);
    // let mut time: Vec<i64> = vec![];
    // let mut dist: Vec<i64> = vec![];
    let time = parse_digs::<i64>(&lines[0]).unwrap();
    let dist = parse_digs::<i64>(&lines[1]).unwrap();
    return (time, dist);
}

fn parse_race_part2(lines: &Vec<String>) -> (i64, i64) {
    let re_digs: Regex = Regex::new(r"\d+").unwrap();
    assert!(lines.len() >= 2);
    let mut tstr = String::from("");
    for tm in re_digs.find_iter(&lines[0]) {
        tstr.push_str(tm.as_str());
    }

    let mut dstr = String::from("");
    for dm in re_digs.find_iter(&lines[1]) {
        dstr.push_str(dm.as_str());
    }

    let t = tstr.parse::<i64>().unwrap();
    let d = dstr.parse::<i64>().unwrap();

    return (t, d);
}

fn roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sterm = (b*b - (4_f64*a*c)).sqrt();
    let x1 = (-b + sterm) / (2_f64*a);
    let x2 = (-b - sterm) / (2_f64*a);
    return (x1, x2);
}

fn roots_from_td(t: i64, d: i64) -> (f64, f64) {
    // we have t ms to move d mm
    // the distance covered is (t-x)*x
    // so we need to find where
    // d = (t-x) * x
    // rewriting this in quadratic form
    //      x**2 - t*x + d = 0
    // 0 = -x**2 + t*x - d
    // let (xtmp1, xtmp2)= roots(1_f64, -t as f64, *d as f64);

    let (xtmp1, xtmp2)= roots(-1_f64, t as f64, -d as f64);
    let x1 = xtmp1.min(xtmp2);
    let x2 = xtmp1.max(xtmp2);
    return (x1, x2);
}

fn win_count_from_td(t: i64, d: i64) -> i64 {
    let (x1, x2) = roots_from_td(t, d);
    // let win_count = x2.floor() as i64 - x1.ceil() as i64;
    // why does the below line work? lol
    let win_count = x2.ceil() as i64 - x1.floor() as i64 - 1_i64;
    // println!("win_count = {win_count}");
    return win_count;
}

pub fn part1(fname: &str) -> i64 {
    let lines = read_lines(fname);
    let (time, dist) = parse_races(&lines);
    // println!("time = {time:?}");
    // println!("dist = {dist:?}");
    assert!(time.len() == dist.len());


    let races: Vec<(i64, i64)> = zip(time, dist).into_iter().map(
        |(t, d)|
        (t, d)
    )
    .collect();

    let mut win_counts: Vec<i64> = vec![];

    for (_ix, (t, d)) in races.iter().enumerate() {
        // println!("race {ix}: t = {t}, d = {d}");
        let win_count = win_count_from_td(*t, *d);
        // println!("win_count = {win_count}");
        win_counts.push(win_count);
    }

    return win_counts.iter().product();
}

pub fn part2(fname: &str) -> i64 {
    let lines = read_lines(fname);
    let (t, d) = parse_race_part2(&lines);
    return win_count_from_td(t, d);
}