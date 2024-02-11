use regex::Regex;
use std::{fs::read_to_string, str::FromStr};

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn parse_digs<T>(line: &String) -> Option<Vec<T>>
where
    T: FromStr,
{
    let mut parsed_digs: Vec<T> = vec![];
    let re_digs: Regex = Regex::new(r"-?\d+").unwrap();
    // let mut parsed_digs: Vec<T> = vec![];

    // let parsed_digs =
    //     |m| m.as_str().parse::<T>()?
    // )
    // .collect();

    for m in re_digs.find_iter(&line) {
        if let Ok(value) = m.as_str().parse::<T>() {
            parsed_digs.push(value);
        } else {
            return None;
        }
    }

    return Some(parsed_digs);
}
