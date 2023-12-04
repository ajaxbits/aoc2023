use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    id: i32,
    chosen: Vec<i32>,
    winning: Vec<i32>,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let id: &str = line
            .split(":")
            .nth(0)
            .expect("there is always at aleast one colon");
        let numbers: &str = line
            .split(":")
            .nth(1)
            .expect("there is always at aleast one colon");
        let id: i32 = id
            .strip_prefix("Card")
            .expect("id is always prefixed by Card and some whitespace")
            .trim()
            .parse()
            .expect("line can always be parsed as i32");
        let numbers: Vec<Vec<i32>> = numbers
            .split('|')
            .map(|s| s.trim())
            .map(|s| {
                let mut nums = s
                    .split_whitespace()
                    .map(|n: &str| {
                        n.parse::<i32>()
                            .expect("all entries will be parseable as i32")
                    })
                    .collect::<Vec<i32>>();
                nums.sort();
                nums
            })
            .collect();
        let (chosen, winning) = match &numbers[..] {
            [first, second, ..] => (first.to_owned(), second.to_owned()),
            _ => return Err("Could not split numbers into two groups.".to_string()),
        };

        let card = Card {
            id,
            chosen,
            winning,
        };

        Ok(card)
    }
}

fn get_winners(chosen: Vec<i32>, winning: Vec<i32>) -> Option<Vec<i32>> {
    let mut winners: Vec<i32> = Vec::new();
    for n in chosen.iter() {
        if winning.contains(n) {
            winners.push(*n)
        }
    }
    match winners.len() {
        0 => None,
        _ => Some(winners),
    }
}

pub fn problem_1(cards: &str) -> i32 {
    let cards: Vec<Card> = cards
        .par_lines()
        .map(|c: &str| -> Card { c.try_into().expect("could not parse line as Card") })
        .collect();

    let scores: Vec<Option<i32>> = cards
        .into_iter()
        .map(|c| get_winners(c.chosen, c.winning))
        .map(|poss_winners| {
            poss_winners.map(|winners| {
                let num_winners = winners.len() as u32;
                2i32.pow(num_winners - 1)
            })
        })
        .collect();

    scores
        .into_iter()
        .filter_map(|score: Option<i32>| score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_card() {
        let example =
            fs::read_to_string("./data/examples/04/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        let example1: Card = example[0].try_into().unwrap();
        let example2: Card = example[2].try_into().unwrap();
        let expected1 = Card {
            id: 1,
            chosen: vec![17, 41, 48, 83, 86],
            winning: vec![6, 9, 17, 31, 48, 53, 83, 86],
        };
        let expected2 = Card {
            id: 3,
            chosen: vec![1, 21, 44, 53, 59],
            winning: vec![1, 14, 16, 21, 63, 69, 72, 82],
        };
        assert_eq!((example1), expected1);
        assert_eq!((example2), expected2);
    }

    #[test]
    fn test_winners() {
        let example =
            fs::read_to_string("./data/examples/04/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        let example1: Card = example[0].try_into().unwrap();
        let example2: Card = example[1].try_into().unwrap();
        let example3: Card = example[2].try_into().unwrap();
        let example4: Card = example[3].try_into().unwrap();
        let example5: Card = example[4].try_into().unwrap();

        let example1: Vec<i32> = get_winners(example1.chosen, example1.winning).unwrap();
        let example2: Vec<i32> = get_winners(example2.chosen, example2.winning).unwrap();
        let example3: Vec<i32> = get_winners(example3.chosen, example3.winning).unwrap();
        let example4: Vec<i32> = get_winners(example4.chosen, example4.winning).unwrap();
        let example5: Option<Vec<i32>> = get_winners(example5.chosen, example5.winning);

        let expected1 = vec![17, 48, 83, 86];
        let expected2 = vec![32, 61];
        let expected3 = vec![1, 21];
        let expected4 = vec![84];
        let expected5 = None;

        assert_eq!(example1, expected1);
        assert_eq!(example2, expected2);
        assert_eq!(example3, expected3);
        assert_eq!(example4, expected4);
        assert_eq!(example5, expected5);
    }

    #[test]
    fn test_example_1() {
        let example =
            fs::read_to_string("./data/examples/04/problem1.txt").expect("error loading input");
        assert_eq!(problem_1(&example), 13);
    }
}
