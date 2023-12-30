use std::ops::Range;
use regex::Regex;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    // local_debug(fname);

    let part1_answer = part1(fname);
    println!("day05: part1: answer = {part1_answer}");
}

// fn local_debug(fname: &str) {
//     let lines = read_lines(fname);
//     let (seeds, maps) = parse_maps(&lines);
//     println!("seeds = {seeds:?}");
//     let mut ix = 0;
//     for map in maps {
//         println!("map {ix}:");
//         for line in map {
//             println!("{line:?}");
//         }
//         ix += 1;
//     }
// }

fn parse_maps(lines: &Vec<String>) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let re_digs: Regex = Regex::new(r"\d+").unwrap();
    // x.as_str().parse::<i32>().unwrap()
    let mut seeds: Vec<i64> = vec![];
    let mut maps: Vec<Vec<Vec<i64>>> = vec![];
    let mut this_map: Vec<Vec<i64>> = vec![];
    for line in lines {
        if line.starts_with("seeds") {
            let seed_values: Vec<_> = re_digs.find_iter(&line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            seeds.extend(seed_values);
        }
        else if line.is_empty() && this_map.len() > 0 {
            maps.push(this_map.clone());
            this_map.clear();
        }
        else {
            let digs: Vec<i64> = re_digs.find_iter(&line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            if digs.len() == 3 {
                this_map.push(digs.clone());
            }
        }
    }
    if this_map.len() > 0 {
        maps.push(this_map);
    }
    return (seeds, maps);
}

// 130120695 is too low
pub fn part1(fname: &str) -> i64 {
    let mut answer: i64 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    let (seeds, maps) = parse_maps(&lines);

    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut xfer_count = 0;
        let mut src_val = seed;
        // println!("\nseed = {src_val}");

        'maploop: for map in &maps {
            let mut found_lookup = false;
            for r in map {
                let dst_rng = r[0] .. r[0]+r[2];
                let src_rng = r[1] .. r[1]+r[2];
                if src_rng.contains(&src_val) {
                    let new_src_val = dst_rng.start + (src_val - src_rng.start);
                    // println!("{new_src_val} <- {dst_rng:?} <<<<<< {src_rng:?} <- {src_val}");
                    // println!("{src_val} -> {src_rng:?} >>>>>> {dst_rng:?} -> {new_src_val}");
                    src_val = new_src_val;
                    xfer_count += 1;
                    found_lookup = true;
                    continue 'maploop;
                }
            }
            // if we get here, and found_lookup is false, then dst = src
            // but... this just means src_val = src_val...
            if !found_lookup {
                xfer_count += 1;
                // println!("{src_val}  >>>>>>  {src_val}");
            }
        }
        // after the 'maploop, src_val should actually be the location we're looking for
        let loc = src_val;
        // println!("loc = {loc}, xfer_count = {xfer_count}, {:?}", maps.len());
        // if xfer_count == 6 {
        locations.push(loc);
    }

    // println!("locations = {locations:?}");

    answer = *locations.iter().min().unwrap();

    return answer;
}

pub fn part2(fname: &str) -> i32 {
    let mut answer = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    let (seeds, orig_maps) = parse_maps(&lines);

    // convert seeds to vector of Range<i64>

    println!("dayXX: part2: answer = {answer}");
    return answer;
}

