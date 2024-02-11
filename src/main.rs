extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod utils;
// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
mod day09;

fn main() {
    // set the log level explicitly - remove this when debugging to set log level on the command line via
    // RUST_LOG=trace cargo run
    // std::env::set_var("RUST_LOG", "debug");
    match std::env::var("RUST_LOG") {
        Ok(log_level) => println!("INFO: RUST_LOG = {log_level}"),
        Err(_e) =>  std::env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();
    // env_logger::init();
    // day01::run("day01/input.txt");
    // day02::run("day02/input.txt");
    // day03::run("day03/input.txt");
    // day04::run("day04/test1.txt", 5);
    // day04::run("day04/input.txt", 10);
    // day05::run("day05/test1.txt");
    // day05::run("day05/input.txt");
    // day06::run("day06/test1.txt");
    // day06::run("day06/input.txt");
    // day07::run("day07/test1.txt");
    // day07::run("day07/test2.txt");
    // day07::run("day07/input.txt");
    // day08::run("day08/test2.txt");
    // day08::run("day08/input.txt");
    // day09::run("day09/test1.txt");
    day09::run("day09/input.txt");
}
