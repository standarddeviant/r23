use std::{collections::HashMap, io::Read};
use std::cmp::Ordering;
use crate::utils::{read_lines, parse_digs};


pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    println!("day07: part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    println!("day07: part2: answer = {part2_answer}");
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    HighCard=0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAkind
}

fn determine_hand_type(cards: &Vec<u8>, jval: u8) -> Hand {
    let mut counts: [u8; 15] = [0; 15];

    for c in cards {
        let ix = *c as usize;
        counts[ix] += 1;
    }
    counts.sort();

    // let keys: Vec<&u8> = counts.keys().collect();
    // let mut vals: Vec<u8> = counts.values().collect();
    // vals.sort_by(|a, b| b.cmp(a));

    // handle part2
    // hacky, special case flagging (jval==1 for part2)
    let mut jcount: u8 = 0;
    if jval == 1 && counts[jval as usize] > 0 {
        jcount = counts[jval as usize];
    }

    let hand: Hand = 
    match counts[0] + jcount {
        5 => Hand::FiveOfAkind,
        4 => Hand::FourOfAKind,
        3 => {
            match counts[1] { 
                2 => Hand::FullHouse,
                _ => Hand::ThreeOfAKind
            }
        }
        2 => {
            match counts[1] {
                2 => Hand::TwoPair,
                _ => Hand::OnePair
            }
        }
        _ => Hand::HighCard
    };

    // println!("keys = {keys:?}");
    // println!("vals = {vals:?}");

    return hand;
}

fn parse_hands(lines: &Vec<String>, jval: u8) -> Vec<(Hand, Vec<u8>, i32)> {
    let mut hands: Vec<(Hand, Vec<u8>, i32)> = vec![];
    for line in lines {
        // let line
        // println!("line = {line}");
        let parts: Vec<String> = 
            line.split(" ").map(
                |x|x.to_string()
            ).collect();
        // println!("parts = {parts:?}");
        let cards: Vec<u8> = parts[0].chars().map(
            |x| match x {
                'J'         => jval,
                '2' ..= '9' => x as u8 - '0' as u8,
                'T'         => 10_u8,
                'Q'         => 12_u8,
                'K'         => 13_u8,
                'A'         => 14_u8,
                _ => 0
            }
        ).collect();

        let bid: Vec<i32> = parse_digs::<i32>(&parts[1]).unwrap();
        let bid = bid[0];
        let hand = determine_hand_type(&cards, jval);
        hands.push((hand, cards, bid));
    }

    return hands;
}

fn sort_hands(hands: &mut Vec<(Hand, Vec<u8>, i32)>) {
    hands.sort_by(
        |a, b| -> Ordering {
            // let a8 = a.0 as u8;
            // let b8 = b.0 as u8;
            if (a.0 as u8) > (b.0 as u8) { return Ordering::Greater; }
            if (a.0 as u8) < (b.0 as u8) { return Ordering::Less; }
            for ix in 0 .. a.1.len()  {
                if a.1[ix] > b.1[ix] { return Ordering::Greater; }
                if a.1[ix] < b.1[ix] { return Ordering::Less; }
            }
            return Ordering::Equal;
        }
    );
}

// fn determine_hand(input: &(Vec<u8>, i32)) -> (Hand, Vec<u8>, i32) {
//     // let (cards, )
//     return (Hand::HighCard, vec![0; 5], 0);
// }

pub fn part1(fname: &str) -> i32 {
    let mut answer: i32 = 0;
    let lines = read_lines(fname);
    let mut hands = parse_hands(&lines, 11);
    sort_hands(&mut hands);

    for (rank_ix, hand) in hands.iter().enumerate() {
        let rank_val = (rank_ix+1) as i32;
        let bid = hand.2;
        answer += rank_val * bid;
        // println!("hand = {hand:?}");
    }

    return answer;
}

pub fn part2(fname: &str) -> i32 {
    let mut answer: i32 = 0;
    let mut line_num: i32 = 1;
    let lines = read_lines(fname);
    let mut hands = parse_hands(&lines, 1);
    sort_hands(&mut hands);

    for (rank_ix, hand) in hands.iter().enumerate() {
        let rank_val = (rank_ix+1) as i32;
        let bid = hand.2;
        answer += rank_val * bid;
        // println!("hand = {hand:?}");
    }
    return answer;
}

