use std::iter::zip;
use crate::utils::{parse_digs, read_lines};
// use std::collections::HashMap; // , io::Read};

// type IntType = i128;
type IntType = i128;

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    info!("day09: part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    info!("day09: part2: answer = {part2_answer}");
}

fn parse_input(fname: &str) -> Vec<Vec<IntType>> {
    // pub fn read_lines(filename: &str) -> Vec<String>
    let lines: Vec<String> = read_lines(fname);
    let line_digs: Vec<Vec<IntType>> = lines.iter().map(|x| parse_digs::<IntType>(x).unwrap()).collect();
    for (line, digs) in zip(lines, line_digs.clone()) {
        trace!("line = {line}");
        trace!("digs = {digs:?}");
    }
    line_digs
}

fn extrapolate(invec: &Vec<IntType>, front: bool) -> IntType {
    trace!("invec = {invec:?}");
    // make scratch vec of vecs
    let mut s: Vec<Vec<IntType>> = vec![];
    s.push((*invec).clone());
    loop {
        // check if ultimate scratch vec is all zeros
        let u = s[s.len() - 1].clone();
        let zval: IntType = 0;
        trace!("u = {u:?}");
        if u.iter().all(|x| *x == zval) {
            break;
        }
        // calculate diff of u
        let diff: Vec<IntType> = (1..u.len()).map(|ix| u[ix] - u[ix - 1]).collect();
        // push diff to scratch vec of vecs
        s.push(diff)
    }
    
    let mut answer: IntType = 0;
    if front {
        for ix in (0..s.len() - 1).rev() {
            let tmps = s[ix].clone();
            answer = tmps[0] - answer;
        }
    }
    else {
        for ix in (0..s.len() - 1).rev() {
            let tmps = s[ix].clone();
            answer = answer + tmps[tmps.len() - 1];
        }
   }
    trace!("extrapolate answer = {answer}");
    answer
}

// 1259104315 is too low
pub fn part1(fname: &str) -> IntType {
    let lines_digs = parse_input(fname);
    debug!("lines_digs = {lines_digs:?}");
    let ex_vals: Vec<IntType> = lines_digs.iter().map(|x| extrapolate(x, false)).collect();
    debug!("ex_vals = {ex_vals:?}");
    let answer = ex_vals.iter().sum();
    debug!("answer = {answer:?}");
    answer
}

pub fn part2(fname: &str) -> IntType {
    let lines_digs = parse_input(fname);
    debug!("lines_digs = {lines_digs:?}");
    let ex_vals: Vec<IntType> = lines_digs.iter().map(|x| extrapolate(x, true)).collect();
    debug!("ex_vals = {ex_vals:?}");
    let answer = ex_vals.iter().sum();
    debug!("answer = {answer:?}");
    answer
}

