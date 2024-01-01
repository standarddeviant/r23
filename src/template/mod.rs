use crate::utils::read_lines;

pub fn run(fname: &str) {
    part1(fname);
    println!("");
    part2(fname);
}

fn parse_hands(lines: &Vec<String>) {
    let hands: Vec<(Vec<u8>, i32)> = vec![];
    for line in lines {
        // let line
        println!("line = {line}");
    }
    // let parts = 
    
}

pub fn part1(fname: &str) -> i32 {
    let mut answer: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        println!("{line_num}--{line}");
        line_num += 1;
    }
    answer = 42;
    println!("dayXX: part1: answer = {answer}");
    return answer;
}

pub fn part2(fname: &str) -> i32 {
    let mut answer: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        println!("{line_num}--{line}");
        line_num += 1;
    }
    answer = 42;
    println!("dayXX: part2: answer = {answer}");
    return answer;
}

