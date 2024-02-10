// use std::ops::Deref;
// use std::str::FromStr;
use crate::utils::read_lines;
use num::integer::lcm;
// use std::cmp::Ordering;
use std::collections::HashMap; // , io::Read};

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    info!("day08: part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    info!("day08: part2: answer = {part2_answer}");
}

fn parse_input(fname: &str) -> (Vec<u8>, HashMap<String, [String; 2]>) {
    let lines = read_lines(fname);
    // assume rigid, perfect format for simplicity
    let keys: Vec<u8> = lines[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for line in lines[2..].iter() {
        // 0..|...7..|.C..
        // AAA = (BBB, BBB)
        let k: String = line[0..3].into();
        let v: [String; 2] = [line[7..10].into(), line[12..15].into()];
        map.insert(k, v);
    }

    (keys, map)
}

pub fn part1(fname: &str) -> i32 {
    let mut kix = 0;
    let (keys, map) = parse_input(fname);
    debug!("keys = {keys:?}");
    debug!("map = {map:?}");

    let start: String = String::from("AAA");
    let finish: String = String::from("ZZZ");
    let mut current: String = start.clone();
    while !current.eq(&finish) {
        let kval = keys[kix % keys.len()] as usize;
        kix += 1;
        let branch = map.get(&current).unwrap();
        branch[kval].clone_into(&mut current);
    }

    kix as i32
}

// it's one of those ones where brute force isn't sufficient
// 1857219991 is too low
pub fn part2(fname: &str) -> i128 {
    let (keys, map) = parse_input(fname);
    debug!("keys = {keys:?}");
    debug!("map = {map:?}");

    // let starts: Vec<String> = map.keys()
    let mut starts: Vec<String> = map
        .keys()
        .filter_map(|k| {
            if (*k).ends_with('A') {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect();
    starts.sort();
    debug!("starts = {starts:?}");

    let mut offsets: Vec<i128> = vec![];
    let mut periods: Vec<i128> = vec![];
    for (startix, s) in starts.iter().enumerate() {
        let mut zcount = 0;
        let mut c = s.clone();
        let mut kix = 0;
        let mut off: i128 = 0;
        loop {
            trace!("s={s}, kix={kix}, c={c}");
            let kval = keys[kix % keys.len()] as usize;
            kix += 1;
            let branch = map.get(&c).unwrap();
            branch[kval].clone_into(&mut c);
            if c.ends_with('Z') {
                zcount += 1;
                match zcount {
                    1 => {
                        // first z = offset
                        off = kix as i128;
                    }
                    2 => {
                        // second z = period
                        let per = kix as i128 - off;
                        // trace!("uwuwuwuwuwuw!, off = {off}, per = {per}");
                        debug!("startix={startix},kix={kix},off={off},per={per},c={c},branch={branch:?}");
                        offsets.push(off);
                        periods.push(per);
                        break;
                    }
                    _ => {
                        panic!("unexpected: zcount = {zcount}");
                    }
                }
            } // end if c.ends_with("Z")
              // trace!("startix={startix},kix={kix},off={off},per={per},c={c},branch={branch:?}");
              // if kix > 10 { break; }
        } // end loop: offset/period
          // break;
    } // for (ix, s) in starts.iter().enumerate()

    // now what to do w/ offsets and periods?
    debug!("offsets = {offsets:?}");
    debug!("periods = {periods:?}");

    let mut per_lcm: i128 = 1;
    for (per_ix, per_val) in periods.clone().iter().enumerate() {
        per_lcm = lcm(per_lcm, *per_val);
        debug!("{per_ix} : per_val = {per_val}, per_lcm = {per_lcm}");
    }
    debug!("per_lcm = {per_lcm}");

    per_lcm
}
