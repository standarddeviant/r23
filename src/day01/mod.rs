
use std::str::Bytes;

use crate::utils::read_lines;
// use regex::Regex;
// use std::collections::HashSet;

pub fn run(fname: &str) {
    part1(fname);
    part2(fname);
}

// const fwd_digit_strings: [Bytes; 9] = [
//     "one".as_bytes().to_vec(),
//     "two".into(),
//     "three".into(),
//     "four".into(),
//     "five".into(),
//     "six".into(),
//     "seven".into(),
//     "eight".into(),
//     "nine".into()
// ];

// const strings: [&'static str; 9] = 
const FWD_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

fn first_last_digit(s: &String, strings: bool, rev: bool) -> i32 {
    let mut b = s.clone().into_bytes();
    let mut digit_strings: Vec<Vec<u8>> = vec![];
    if rev {
        b.reverse();
        for s in FWD_DIGITS {
            let tmp: String = s.chars().rev().collect();
            digit_strings.push(tmp.into_bytes());
        }
    }
    else {
        for s in FWD_DIGITS {
            let tmp: String = s.chars().collect();
            digit_strings.push(tmp.into_bytes());
        }
    }
        
    let L = b.len();
    for ix in 0..L {
        // if '0' <= (b[ix] as char) && (b[ix] as char) <= '9' {
        if ('0' as u8) <= b[ix] && b[ix] <= ('9' as u8) {
            let rval = (b[ix] as i32) - ('0' as i32);
            // println!("\nreturning {rval} @ ix = {ix}, c = {}", b[ix]);
            return rval;
        }
        if !strings { continue; }

        let mut rval: i32 = 1;
        // for dsix in 0..digit_strings.len() {
        for ds in &digit_strings {
            let dsL = ds.len();
            if L - ix >= dsL {
                // let check = b[ix..ix+dsL];
                if b[ix .. (ix+dsL)].eq(ds) {
                    // println!("\nreturning {rval} @ ix = {ix}, ds = {ds:?}, b[ix..ix+dsL] = ...");
                    // println!("rev={rev}: {}, {}, {}, ... ", b[ix] as char, b[ix+1] as char, b[ix+2] as char);
                    return rval;
                }
                rval += 1;
            }
        }
    }
    return 0;
} // fn first_last_digit(s: &String, strings: bool, rev: bool) -> i32

pub fn part1(fname: &str) -> i32 {
    let mut sum: i32 = 0;
    let lines = read_lines(fname);
    for line in lines {
        sum += 10 * first_last_digit(&line, false, false);
        sum +=  1 * first_last_digit(&line, false, true);
        // println!("{line}");
    }
    println!("day01: part1: sum = {sum}");
    return sum;
}

pub fn part2(fname: &str) -> i32 {
    let mut sum: i32 = 0;
    let lines = read_lines(fname);
    for line in lines {
        let mut cal = 0;
        cal += 10 * first_last_digit(&line, true, false);
        cal +=  1 * first_last_digit(&line, true, true);
        sum += cal;
        println!("{cal} from {line}");
    }
    println!("day01: part2: sum = {sum}");
    return sum;
}
