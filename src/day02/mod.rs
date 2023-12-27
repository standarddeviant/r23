use crate::utils::read_lines;
// use regex::Regex;
use std::collections::HashMap;

pub fn run(fname: &str) {
    part1(fname);
    part2(fname);
}

fn parse_game(_game_num: i32, line: &String) -> Vec<HashMap<String, i32>> {
    let mut out: Vec<HashMap<String, i32>> = vec![];
    let colix = 2 + line.find(": ").unwrap();
    let gstr: String = line[colix..].to_string();
    // println!("\n{game_num}--gstr = {:?}",  gstr);
    let grab_strings: Vec<&str> = gstr.split("; ").collect();
    // println!("{game_num}--grab_strings = {:?}", grab_strings);
    for gs in grab_strings {
        // println!("{game_num}----{:?}", gs);
        let mut hm: HashMap<String, i32> = HashMap::new();
        let color_strings: Vec<&str> = gs.split(", ").collect();
        for cs in color_strings {
            // println!("{game_num}------{:?}", cs);
            let tmp: Vec<&str> = cs.split(" ").collect();
            hm.insert(
                tmp[1].to_string(),
                tmp[0].to_string().parse::<i32>().unwrap()
            );
        }
        out.push(hm.clone());
    }
    return out;
}

fn valid_game(lims: &HashMap<String, i32>, game: Vec<HashMap<String, i32>>) -> bool {
    for grab in game {
        for (k,v) in grab {
            if v > lims[&k] {
                return false
            }
        }
    }
    return true;
}

fn min_lims(game: Vec<HashMap<String, i32>>) -> HashMap<String, i32> {
    let mut out: HashMap<String, i32> = HashMap::from([
        ("red".to_string()  , i32::MIN),
        ("green".to_string(), i32::MIN),
        ("blue".to_string() , i32::MIN)
    ]);
    for grab in game {
        for (k, v) in grab {
            if v > out[&k] {
                out.insert(k, v);
            }
        }
    }
    return out;
}

pub fn part1(fname: &str) -> i32 {
    let mut out: i32 = 0;
    let mut game_num: i32 = 1;
    // let lims: HashMap<&str, i32, _>::new();
    let lims: HashMap<String, i32> = HashMap::from([
        ("red".to_string()  , 12),
        ("green".to_string(), 13),
        ("blue".to_string() , 14)
    ]);
    // let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
    let lines = read_lines(fname);
    for line in lines {
        let game = parse_game(game_num, &line);
        // println!("{game_num}: {:?}", game);
        if valid_game(&lims, game) {
            out += game_num;
        }
        game_num += 1;
    }
    println!("day02: part1: sum = {out}");
    return out;
}

pub fn part2(fname: &str) -> i32 {
    let mut out: i32 = 0;
    let mut game_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        let game = parse_game(game_num, &line);
        println!("\ngame = {game:?}");
        let mlim = min_lims(game);
        println!("mlim = {mlim:?}");
        let pow: i32 = mlim.values().product();
        out += pow;
        println!("{game_num}--{line}");
        game_num += 1;
    }
    println!("day02: part2: answer = {out}");
    return out;
}

