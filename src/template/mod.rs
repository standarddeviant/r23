use crate::utils::read_lines;

pub fn run(fname: &str) {
    part1(fname);
    println!("");
    part2(fname);
}

pub fn part1(fname: &str) -> i32 {
    let mut out: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        println!("{line_num}--{line}");
        line_num += 1;
    }
    out = 42;
    println!("dayXX: part1: answer = {out}");
    return out;
}

pub fn part2(fname: &str) -> i32 {
    let mut out: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        println!("{line_num}--{line}");
        line_num += 1;
    }
    out = 42;
    println!("dayXX: part2: answer = {out}");
    return out;
}

