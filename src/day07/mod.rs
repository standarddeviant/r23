use std::cmp::Ordering;
use crate::utils::{read_lines, parse_digs, argsort};

pub fn run(fname: &str) {
    let part1_answer = part1(fname);
    info!("day07: part1: answer = {part1_answer}");

    let part2_answer = part2(fname);
    info!("day07: part2: answer = {part2_answer}");
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    HighCard=0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAkind
}

#[derive(Debug, Clone)]
struct Hand {
    hand_type: HandType,
    test_cards: Vec<u8>,
    // counts: Vec<u8>,
    // vals: Vec<u8>,
    // jcount: u8,
    bid: i32
}

fn determine_hand(cards: &Vec<u8>, jval: u8, bid: i32) -> Hand {
    let mut counts: Vec<u8> = Vec::from([0; 15]); // count of each card value, using value as key/idx
    let test_cards = cards.clone(); // a copy of the card for tie-breaking
    let mut jix: usize = 0; // index of potential joker

    for c in cards {
        let ix = *c as usize;
        counts[ix] += 1;
    }
    
    let n: usize = counts.clone().iter().map(
        |x| if *x > 0 {1} else {0}
    ).sum();
    debug!("n = {n}");
    
    let mut ixs: Vec<usize> = argsort(&counts.clone().to_vec());
    ixs.reverse();
    ixs.resize(n, 0);
    let ixs: Vec<u8> = ixs.iter().map(|x|*x as u8).collect();
    let vals = ixs;
    debug!("vals = {vals:?}");
    
    // handle part2
    // hacky, special case flagging (jval==1 for part2)
    let mut jcount: u8 = 0;
    if jval == 1 && counts[jval as usize] > 0 {
        trace!("test!");
        jcount = counts[jval as usize];
        jix = vals.iter().position(|&r| r == jval).unwrap();
    }
    
    // sort the counts - not needed for this problem, but nice for card problems generally
    counts.sort();
    counts.reverse();
    counts.resize(n, 0);
    debug!("counts = {counts:?}");
 
    let mut c0 = counts[0];
    let mut c1 = if counts.len() > 1 { counts[1] } else { 0 };
    trace!("counts[0]  = {}, jcount = {jcount}", counts[0]);
    
    if jval==1 && vals.contains(&jval) {
        // this jix==0 special case exists because [1, 1, 5, 4, 3] needs to evaluate to a ThreeOfAKind
        if jix == 0 {
            if vals.len() >= 2 { c0 += counts[1]; }
            if vals.len() >= 3 { c1  = counts[2]; }
        }
        else { // i.e. jix != 0
            c0 += jcount;
            for tmpix in 1..5 {
                if jix != tmpix && counts.len() > tmpix {
                    c1 = counts[tmpix];
                    break; 
                }
            }
        }
    }
            
    let hand_type: HandType = 
    match c0 {
        5 => HandType::FiveOfAkind,
        4 => HandType::FourOfAKind,
        3 => {
            match c1 {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind
            }
        }
        2 => {
            match c1 {
                2 => HandType::TwoPair,
                _ => HandType::OnePair
            }
        }
        _ => HandType::HighCard
    };
    
    return Hand{
        hand_type: hand_type,
        test_cards: test_cards,
        // counts: counts,
        // vals: vals,
        // jcount: jcount,
        bid: bid
    };
}

fn parse_hands(lines: &Vec<String>, jval: u8) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        // let line
        trace!("line = {line}");
        let parts: Vec<String> = 
            line.split(" ").map(
                |x|x.to_string()
            ).collect();
        // trace!("parts = {parts:?}");
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
        let hand = determine_hand(&cards, jval, bid);
        trace!("hand = {hand:?}\n");
        hands.push(hand)
    }

    return hands;
}

// fn sort_hands(hands: &mut Vec<(Hand, Vec<u8>, i32)>) {
fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(
        |a, b| -> Ordering {
            // let a8 = a.0 as u8;
            // let b8 = b.0 as u8;
            if (a.hand_type as u8) > (b.hand_type as u8) { return Ordering::Greater; }
            if (a.hand_type as u8) < (b.hand_type as u8) { return Ordering::Less; }
            for ix in 0 .. 5 {
                if a.test_cards[ix] > b.test_cards[ix] { return Ordering::Greater; }
                if a.test_cards[ix] < b.test_cards[ix] { return Ordering::Less; }
            }
                
            return Ordering::Equal;
        }
    );
}

pub fn part1(fname: &str) -> i32 {
    let mut answer: i32 = 0;
    let lines = read_lines(fname);
    let mut hands = parse_hands(&lines, 11);
    sort_hands(&mut hands);

    for (rank_ix, hand) in hands.iter().enumerate() {
        let rank_val = (rank_ix+1) as i32;
        let bid = hand.bid;
        answer += rank_val * bid;
        // trace!("hand = {hand:?}");
    }

    return answer;
}

// 244274116 is too high
// 243791898 is too high
// 243662438 is too high
pub fn part2(fname: &str) -> i64 {
    let mut answer: i64 = 0;
    let lines = read_lines(fname);
    let mut hands = parse_hands(&lines, 1);
    sort_hands(&mut hands);
    debug!("part2: sorted hands = ");//"{hands:?}");
    for (ix, hand) in hands.iter().enumerate() {
        debug!("{ix} : {hand:?}");
    }

    for (rank_ix, hand) in hands.iter().enumerate() {
        let rank_val = (rank_ix+1) as i32;
        let bid = hand.bid;
        answer += (rank_val as i64) * (bid as i64);
        // trace!("hand = {hand:?}");
    }
    return answer;
}

