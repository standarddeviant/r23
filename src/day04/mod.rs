use std::collections::HashSet;

use crate::utils::read_lines;
use regex::{Regex, Match};

pub fn run(fname: &str, winners_len: usize) {
    let part1_answer = part1(fname, winners_len);
    println!("day04: part1: answer = {part1_answer}");
    // part2(fname);
}

fn parse_cards(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut cards: Vec<Vec<i32>> = vec![];
    let re_digs: Regex = Regex::new(r"\d+").unwrap();
    for line in lines {
        // let card: Vec<Match> = re_digs.find_iter(&line).collect();
        // println!("card = {card:?}");
        let card: Vec<i32> = re_digs.find_iter(&line)
            .map(
                |x| 
                x.as_str().parse::<i32>().unwrap()
            )
            .collect();
        cards.push(card);
    }
    return cards;
}

pub fn part1(fname: &str, winners_len: usize) -> i32 {
    let mut answer: i32 = 0;
    let lines = read_lines(fname);
    let cards = parse_cards(&lines);
    for card in cards {
        let mut num_winning_cards: u32 = 0;
        let _game = card[0];
        let winners: Vec<i32> = card[1 .. (1+winners_len)].into();
        let players: Vec<i32> = card[(1+winners_len).. ].into();
        for player in players {
            if winners.contains(&player) {
                num_winning_cards += 1;
            }
        }
        if num_winning_cards > 0 {
            answer += 2_i32.pow(num_winning_cards-1);
        }
    }
    return answer;
}

pub fn part2(fname: &str) -> i32 {
    let mut out: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    for line in lines {
        println!("{line_num}--{line}");
        line_num += 1;
    }
    println!("dayXX: part2: sum = {out}");
    return out;
}

