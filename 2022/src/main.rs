#![allow(dead_code)]
#![allow(unused_variables)]
use took::Timer;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

mod days;

fn main() {
    days::all().iter().for_each(|j| {
        let timer = Timer::new();
        let ouput = j();
        let description = &format!("{: <38}\t", ouput);

        timer.took().describe(description);
    });
}
