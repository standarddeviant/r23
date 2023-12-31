use std::collections::VecDeque;
use std::ops::{Bound, Range, RangeBounds};
use std::iter::zip;
use regex::Regex;
use ranges::{GenericRange, OperationResult};
use crate::utils::read_lines;

pub fn run(fname: &str) {
    // local_debug(fname);

    let part1_answer = part1(fname);
    println!("day05: part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    println!("day05: part2: answer = {part2_answer}");
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
    // let mut answer: i64 = 0;
    let lines = read_lines(fname);
    let (seeds, maps) = parse_maps(&lines);

    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        // let mut xfer_count = 0;
        let mut src_val = seed;
        // println!("\nseed = {src_val}");

        'maploop: for map in &maps {
            for r in map {
                let dst_rng = r[0] .. r[0]+r[2];
                let src_rng = r[1] .. r[1]+r[2];
                if src_rng.contains(&src_val) {
                    let new_src_val = dst_rng.start + (src_val - src_rng.start);
                    // println!("{new_src_val} <- {dst_rng:?} <<<<<< {src_rng:?} <- {src_val}");
                    // println!("{src_val} -> {src_rng:?} >>>>>> {dst_rng:?} -> {new_src_val}");
                    src_val = new_src_val;
                    // xfer_count += 1;
                    // found_lookup = true;
                    continue 'maploop;
                }
            }
            // if we get here and didn't find a dst/src lookup pair, then
            // src = dst, but dst becomes src for the next loop iteration
            // so, it's non-obvious, but there's correctly nothing to do here.
            // src = dst
        }
        
        // after the 'maploop, src_val should actually be the location we're looking for
        let loc = src_val;
        // println!("loc = {loc}, xfer_count = {xfer_count}, {:?}", maps.len());
        // if xfer_count == 6 {
        locations.push(loc);
    }

    // println!("locations = {locations:?}");

    return *locations.iter().min().unwrap();
}

// TODO: move this to utils.rs w/ a generic arg type
fn grange_to_range(gr: GenericRange<i64>) -> Option<Range<i64>> {
    let mut start: i64 = -1;
    let mut end: i64 = -1;

    if let Bound::Included(s) = gr.start_bound() {
        start = *s;
    }

    if let Bound::Excluded(e) = gr.end_bound() {
        end = *e;
    }

    if start >= 0 && end >= 0 {
        return Some(start .. end);
    }

    return None;
}

fn grange_inter(ga: GenericRange<i64>, gb: GenericRange<i64>) -> Option<Range<i64>> {
    return match ga.intersect(gb) {
        OperationResult::Single(x) => {
            if let Some(r) = grange_to_range(x) {
                Some(r)
            }
            else {
                None
            }
        },
        _ => None
    }
}

// find the intersection and differences of a w.r.t. b
fn rng_inter_diffs(a: &Range<i64>, b: &Range<i64>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
    let mut inter: Vec<Range<i64>> = vec![];
    let mut diffs: Vec<Range<i64>> = vec![];

    // use ranges crate to avoid writing annoying logic
    let ga: GenericRange<i64> = GenericRange::from(a.clone());
    let gb: GenericRange<i64> = GenericRange::from(b.clone());

    // gather intersection of a + b
    if let Some(r) = grange_inter(ga, gb) {
        inter.push(r);
    }

    // gather disjoint parts of of a w.r.t. b
    match ga.difference(gb) {
        OperationResult::Empty => (),
        OperationResult::Single(x) => {
            if let Some(r) = grange_inter(ga, x) {
                diffs.push(r);
            }
        },
        OperationResult::Double(x, y)=> {
            if let Some(r) = grange_inter(ga, x) {
                diffs.push(r);
            }
            if let Some(r) = grange_inter(ga, y) {
                diffs.push(r);
            }
        }
    }

    // return results
    return (inter, diffs);
}

pub fn part2(fname: &str) -> i64 {
    let lines = read_lines(fname);
    let (seeds, orig_maps) = parse_maps(&lines);

    // convert seeds to vector of Range<i64>
    let seed_rngs: Vec<Range<i64>> = 
        zip(
            seeds[0..].iter().step_by(2),
            seeds[1..].iter().step_by(2)
        )
        .map(
            |(start, len)|
            start.clone() .. (start.clone()+len)
        )
        .collect();

    // convert maps to Vec<Vec<(Range<i64>, Range<i64>)>>
    let maps: Vec<Vec<(Range<i64>, Range<i64>)>> = 
        orig_maps.iter().map(
            |map1x|
            map1x.iter().map(
                |v|
                (
                    v[0] .. v[0] + v[2], // dst range
                    v[1] .. v[1] + v[2], // src range
                )
            )
            .collect()
        )
        .collect();

    // TODO - clean up the 'nestiness' of this function

    // iterate over maps
    let _map_ix = 0;
    let mut src: VecDeque<Range<i64>> = VecDeque::from(seed_rngs.clone());
    let mut dst: Vec<Range<i64>> = vec![];
    '_map_loop: for (_map_ix, map) in maps.iter().enumerate() {

        // use map (i.e. dst/src range pairs) to xfer src -> dst
        // println!("{map_ix} : src = {src:?}");
        // println!("map = {map:?}");
        '_map1x_loop: loop {
            let mut inter_count = 0;
            // let mut exclusions: Vec<Range<i64>> = vec![];
            // let mut no_inters: Vec<
            'src_loop: while !src.is_empty() {
                let s = src.pop_front().unwrap();
                // check through all ranges in map vs. s, the bit of src we just popped
                '_map_rng_lu_loop: for (rd, rs) in map {
                    let (inter, diffs) = rng_inter_diffs(&s, rs);
                    if inter.len() > 0 {
                        // count this range intersection
                        inter_count += 1;
                        // mark this src for removal after loop
                        // transfer intersection to dst via rs (rs / rd)
                        // NOTE: rs = range_src
                        // NOTE: rd = range_dst
                        dst.extend(
                            inter.iter().map(
                                // let new_src_val = dst_rng.start + (src_val - src_rng.start);
                                |si| {
                                    let delta = rd.start - rs.start;
                                    let di= si.start + delta .. si.end + delta;
                                    // println!("inter: si = {si:?} -> di = {di:?};     rd.start = {}, rs.start = {}", rd.start, rs.start);
                                    di
                                }
                            )
                        );
                        // transfer exclusions to end of src - this keeps the rmixs above valid
                        src.extend(diffs);
                        continue 'src_loop;
                    } // end if inter.len() > 0

                } // end 'map_rng_lu_loop

                // if we get here, it means s saw no intersections with any part of this map
                // so - we just put s directly into dst, as-is per the rules
                dst.push(s);

            } // 'src_loop

            // if we didn't find any overlaps in a single iteration of 'src_loop, we're done w/ this map
            if inter_count == 0 {
                break;
            }
        } // end 'map1x_loop

        // after a single iteration of map1x_loop, just simply add remaining ranges in src to dst
        // but - dst on this loop needs to become src on the next iteration of 'map_loop
        // so  - just consume dst into src
        // println!("dst = {dst:?}\n");
        src.extend(dst.clone());
        dst.clear();
    } // 'map_loop

    let locs = src;
    // println!("locs = {locs:?}");

    return locs.iter().map(|x| x.start).min().unwrap();
}

