use regex::{Regex, Match};

use crate::utils::read_lines;

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    println!("day03: part1: answer = {}", part1_answer);
    let part2_answer = part2(fname);
    println!("day03: part2: answer = {}", part2_answer);
}

fn parse_digs_syms(lines: &Vec<String>) -> (Vec<Vec<Match>>, Vec<Vec<Match>>) {
    let re_digs= Regex::new(r"\d+").unwrap();
    let re_syms= Regex::new(r"[^0-9\.]").unwrap();
    let mut digs: Vec<Vec<Match>> = vec![];
    let mut syms: Vec<Vec<Match>> = vec![];
    // let lines = read_lines(fname);

    // for (_line_ix, line) in &lines.iter().enumerate() {
    for line in lines {
        let tmp_digs: Vec<Match> = re_digs.find_iter(&line).collect();
        let tmp_syms: Vec<Match> = re_syms.find_iter(&line).collect();
        digs.push(tmp_digs.clone());
        syms.push(tmp_syms.clone());
    } // end regex SEARCH loop
    return (digs, syms);
}

pub fn part1(fname: &str) -> i32 {
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    // NOTE: IMPORTANT!!!!: PART NUMS CAN BE REPEATED AND ARE ***NOT*** UNIQUE!!!
    let mut part_nums: Vec<i32> = vec![];
    let lines = read_lines(fname);
    let (digs, syms) = parse_digs_syms(&lines);

    for (line_ix, tmp_digs) in digs.iter().enumerate() {
        let mut sym_line_checks: Vec<usize> = vec![];
        if line_ix >             0 { sym_line_checks.push(line_ix - 1); }
        if true                    { sym_line_checks.push(line_ix + 0); }
        if line_ix < digs.len()-1  { sym_line_checks.push(line_ix + 1); }
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

pub fn part2(fname: &str) -> i64 {
    // let mut part_nums: Vec<i32> = vec![];
    let mut gears: Vec<Vec<Match>> = vec![];
    let lines = read_lines(fname);
    let (digs, syms) = parse_digs_syms(&lines);

    // find gears, calculate ratios
    for (line_ix, line_syms) in syms.iter().enumerate() {
        let mut line_check_ixs: Vec<usize> = vec![];
        if line_ix >             0 { line_check_ixs.push(line_ix - 1); }
        if true                    { line_check_ixs.push(line_ix + 0); }
        if line_ix < digs.len()-1  { line_check_ixs.push(line_ix + 1); }
        'symloop: for s in line_syms {
            let mut tmp_gear: Vec<Match> = vec![s.clone()];
            // println!("{d:?}");
            for chkix in &line_check_ixs{
                let chkdigs = &digs[*chkix].clone();
                for d in chkdigs {
                    // println!("{s:?}");
                    let digixs = (d.start() as i32 - 1_i32, d.end() as i32 + 1_i32);
                    let symixs = (s.start() as i32        , s.end() as i32        );
                    // println!("{d:?}, {s:?}, {digixs:?}, {symixs:?}");
                    if digixs.0 <= symixs.0 && symixs.1 <= digixs.1 {
                        tmp_gear.push(*d);
                        if tmp_gear.len() >= 3 {
                            gears.push(tmp_gear);
                            continue 'symloop; // continue checking following digits
                        }
                    }
                }
            }
        }
    } // end gear search

    let mut out: i64 = 0;
    for g in gears {
        let d1 = g[1].as_str().parse::<i32>().unwrap() as i64;
        let d2 = g[2].as_str().parse::<i32>().unwrap() as i64;
        // println!("{d1}, {d2}");
        out += d1 * d2;
    }

    return out;
}

