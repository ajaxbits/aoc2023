use std::fs;

mod day01;

fn main() {
    let day1_1 = day01::problem_1(read_input(1));
    let day1_2 = day01::problem_2(read_input(1));
    println!("{day1_1}");
    println!("{day1_2}");
}

fn read_input(day: i32) -> String {
    fs::read_to_string(format!("./data/input/{:0>2}.txt", day)).expect("error loading input")
}
