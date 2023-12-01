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

enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example() {
        let input =
            fs::read_to_string("./data/examples/01/problem1Test.txt").expect("error loading input");
        let result = problem_1(input);
        assert_eq!(result, 142 as u32);
    }
}
