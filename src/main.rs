use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let day1_1 = day01::problem_1(read_input(1));
    let day1_2 = day01::problem_2(read_input(1));
    let day2_1 = day02::problem_1(read_input(2));
    let day2_2 = day02::problem_2(read_input(2));
    // let day3_1 = day03::problem_1(&read_input(3));
    let day4_1 = day04::problem_1(&read_input(4));

    println!("Day 1, problem 1: {day1_1}");
    println!("Day 1, problem 2: {day1_2}");
    println!("Day 2, problem 1: {day2_1}");
    println!("Day 1, problem 2: {day2_2}");
    // println!("{day3_1}");
    println!("Day 4, problem 1: {day4_1}");
}

fn read_input(day: i32) -> String {
    fs::read_to_string(format!("./data/input/{:0>2}.txt", day)).expect("error loading input")
}
