use crate::utils::read_lines;
use regex::Regex;

pub fn run(fname: &str, win_len: usize) {
    let part1_answer = part1(fname, win_len);
    println!("day04: part1: answer = {part1_answer}");

    let part2_answer = part2(fname, win_len);
    println!("day04: part2: answer = {part2_answer}");
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

fn win_count(card: &Vec<i32>, win_len: usize) -> usize {
    let mut num_winning_cards: usize = 0;
    let _game = card[0];
    let winners: Vec<i32> = card[1 .. (1+win_len)].into();
    let players: Vec<i32> = card[(1+win_len).. ].into();
    for player in players {
        if winners.contains(&player) {
            num_winning_cards += 1;
        }
    }
    return num_winning_cards;
}

fn score_card(card: &Vec<i32>, win_len: usize) -> i32 {
    let mut score: i32 = 0;
    let num_winning_cards = win_count(&card, win_len);
    if num_winning_cards > 0 {
        score += 2_i32.pow(num_winning_cards as u32 -1);
    }
    return score;
}

pub fn part1(fname: &str, win_len: usize) -> i32 {
    let mut answer: i32 = 0;
    let lines = read_lines(fname);
    let cards = parse_cards(&lines);
    for card in cards {
        answer += score_card(&card, win_len);
    }
    return answer;
}

//   310459 is too low
// 18133159 is too high
pub fn part2(fname: &str, win_len: usize) -> i32 {
    let lines = read_lines(fname);
    let cards = parse_cards(&lines);

    let win_counts_1x: Vec<usize> = cards.iter().map(
        |card| win_count(card, win_len)
    ).collect();

    // we'll fill these values in below
    let mut copy_counts: Vec<i32> = vec![1; cards.len()];
    // let scores_1x: Vec<i32> =
    //     cards.iter()
    //     .map(|card| score_card(card, win_len))
    //     .collect();

    // iterate through each game and count copies of cards
    for ix in 0..cards.len() {
        let wc1x = win_counts_1x[ix];
        // 'play' each copy of the card
        for _copyix in 0..copy_counts[ix] {
            for ix2 in ix+1 .. ix+1+wc1x {
                if ix2 < copy_counts.len() {
                    copy_counts[ix2] += 1;
                }
            }
        }
    }

    // for ix in 0..scores_1x.len() {
    //     answer += scores_1x[ix] * copy_counts[ix];
    // }

    return copy_counts.iter().sum();
}

