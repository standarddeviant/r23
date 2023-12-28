use regex::{Regex, Match};

use crate::utils::read_lines;

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    println!("day03: part1: answer = {}", part1_answer);
    // part2(fname);
}

pub fn part1(fname: &str) -> i32 {
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    let mut part_nums: Vec<i32> = vec![];
    let mut digs: Vec<Vec<Match>> = vec![];
    let mut syms: Vec<Vec<Match>> = vec![];
    let lines: Vec<String> = read_lines(fname);
    let re_digs= Regex::new(r"\d+").unwrap();
    let re_syms= Regex::new(r"[^0-9\.]").unwrap();

    for (_line_ix, line) in lines.iter().enumerate() {
        let tmp_digs: Vec<Match> = re_digs.find_iter(line).collect();
        let tmp_syms: Vec<Match> = re_syms.find_iter(line).collect();
        digs.push(tmp_digs.clone());
        syms.push(tmp_syms.clone());
    } // end regex SEARCH loop

    for (line_ix, tmp_digs) in digs.iter().enumerate() {
        let mut sym_line_checks: Vec<usize> = vec![];
        if line_ix >             0 { sym_line_checks.push(line_ix - 1); }
        if true                    { sym_line_checks.push(line_ix + 0); }
        if line_ix < lines.len()-1 { sym_line_checks.push(line_ix + 1); }
        'digloop: for d in tmp_digs {
            // println!("{d:?}");
            for chkix in &sym_line_checks {
                let chksyms = &syms[*chkix].clone();
                for s in chksyms {
                    // println!("{s:?}");
                    let digixs = (d.start() as i32 - 1_i32, d.end() as i32 + 1_i32);
                    let symixs = (s.start() as i32        , s.end() as i32        );
                    // println!("{d:?}, {s:?}, {digixs:?}, {symixs:?}");
                    if digixs.0 <= symixs.0 && symixs.1 <= digixs.1 {
                        let val = d.as_str().parse::<i32>().unwrap();
                        part_nums.push(val);
                        continue 'digloop; // continue checking following digits
                    }
                }
            }
        }
    } // end regex CHECK loop

    // println!("{part_nums:?}");
    return part_nums.iter().sum();
}

// pub fn part2(fname: &str) -> i32 {
//     let mut out: i32 = 0;
//     let mut line_num: i32 = 1;
//     let lines = read_lines(fname);
//     for line in lines {
//         println!("{line_num}--{line}");
//         line_num += 1;
//     }
//     println!("dayXX: part2: sum = {out}");
//     return out;
// }

