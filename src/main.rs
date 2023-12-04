use std::fs;

mod day01;
mod day02;
mod day03;

fn main() {
    let day1_1 = day01::problem_1(read_input(1));
    let day1_2 = day01::problem_2(read_input(1));
    let day2_1 = day02::problem_1(read_input(2));
    let day2_2 = day02::problem_2(read_input(2));
    let day3_1 = day03::problem_1(&read_input(3));
    // println!("{day1_1}");
    // println!("{day1_2}");
    // println!("{day2_1}");
    // println!("{day2_2}");
    println!("{day3_1}");
}

fn read_input(day: i32) -> String {
    fs::read_to_string(format!("./data/input/{:0>2}.txt", day)).expect("error loading input")
}
