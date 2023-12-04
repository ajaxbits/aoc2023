use rayon::prelude::*;
pub fn problem_1(input: String) -> u32 {
    input
        .par_lines()
        .map(|s| {
            let digits = s.chars().map(|c: char| c.to_digit(10));
            let first: u32 = digits.clone()
                .find(|d: &Option<u32>| d.is_some())
                .expect("There will always be one digit in a scrambled string.")
                .unwrap();
            let last: u32 = digits.clone()
                .rfind(|d: &Option<u32>| d.is_some())
                .expect("There will always be one digit in a scrambled string.")
                .unwrap();
            let number: u32 = format!("{first}{last}").parse().expect("the first and last digit will always be positive numbers, meaning that they can only combine to a positive number");
            number
        })
        .sum()
}

const DIGITS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn str_to_int(s: String) -> u32 {
    match s.parse::<u32>() {
        Ok(n) => n,
        Err(_) => match &*s {
            "zero" => 0_u32,
            "one" => 1_u32,
            "two" => 2_u32,
            "three" => 3_u32,
            "four" => 4_u32,
            "five" => 5_u32,
            "six" => 6_u32,
            "seven" => 7_u32,
            "eight" => 8_u32,
            "nine" => 9_u32,
            _ => {
                panic!("{s}")
            }
        },
    }
}

pub fn problem_2(input: String) -> u32 {
    let scrambled_coordinates: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;
    for scrambled_coord in scrambled_coordinates {
        let mut first: &str = "";
        let mut last: &str = "";
        let mut first_index: usize = scrambled_coord.len();
        let mut last_index: usize = 0;

        for n in DIGITS {
            if let (Some(i), Some(j)) = (scrambled_coord.find(n), scrambled_coord.rfind(n)) {
                if i < first_index {
                    first = n;
                    first_index = i;
                }
                if j >= last_index {
                    last = n;
                    last_index = j;
                }
            }
        }

        let first: u32 = str_to_int(first.to_string());
        let last: u32 = str_to_int(last.to_string());

        let final_number: u32 = format!("{first}{last}").parse().unwrap();
        sum += final_number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_1() {
        let input =
            fs::read_to_string("./data/examples/01/problem1Test.txt").expect("error loading input");
        let result = problem_1(input);
        assert_eq!(result, 142 as u32);
    }

    #[test]
    fn example_2() {
        let input = fs::read_to_string("./data/examples/01/problem1Test2.txt")
            .expect("error loading input");
        let result = problem_2(input);
        assert_eq!(result, 281 as u32);
    }
    #[test]
    fn edgecases() {
        let edges =
            fs::read_to_string("./data/examples/01/edgecases.txt").expect("error loading input");
        let result = problem_2(edges);
        assert_eq!(result, 143 as u32);
    }
}
