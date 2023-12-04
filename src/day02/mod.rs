use rayon::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Set(i32, i32, i32);
impl TryFrom<&str> for Set {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn get_number(input: &str, color: &str) -> i32 {
            input
                .replace(color, "")
                .trim()
                .parse()
                .expect("everything after the color name will be a valid i32")
        }

        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        for amount in value.trim().split(',') {
            if amount.ends_with("red") {
                r = get_number(amount, "red");
            } else if amount.ends_with("green") {
                g = get_number(amount, "green");
            } else if amount.ends_with("blue") {
                b = get_number(amount, "blue");
            } else {
                return Err(amount.to_string());
            }
        }

        Ok(Set(r, g, b))
    }
}
const BAG_1: Set = Set(12, 13, 14);

fn get_game_id(input: String) -> i32 {
    let id: i32 = input
        .split(':')
        .next()
        .expect("all strings have at least one colon, and a game id before it")
        .strip_prefix("Game ")
        .expect("all strings have a Game substring before the colon")
        .parse()
        .expect("everything after Game, but before the colon will be a i32");
    id
}

fn get_sets(input: String) -> Vec<Set> {
    input
        .split(':')
        .nth(1)
        .expect("all strings have exactly one colon, with sets following it")
        .split(';')
        .map(|s| s.try_into().expect("error parsing sets"))
        .collect()
}

fn is_possible(input: String) -> bool {
    let bag = BAG_1;
    let sets: Vec<Set> = get_sets(input);
    let invalid_reds: Vec<i32> = sets
        .iter()
        .filter_map(|s: &Set| match s.0 > bag.0 {
            true => Some(s.0),
            false => None,
        })
        .collect();
    let invalid_greens: Vec<i32> = sets
        .iter()
        .filter_map(|s: &Set| match s.1 > bag.1 {
            true => Some(s.1),
            false => None,
        })
        .collect();
    let invalid_blues: Vec<i32> = sets
        .iter()
        .filter_map(|s: &Set| match s.2 > bag.2 {
            true => Some(s.2),
            false => None,
        })
        .collect();

    invalid_reds.is_empty() && invalid_greens.is_empty() && invalid_blues.is_empty()
}

fn get_min_bag(game: String) -> Set {
    let sets: Vec<Set> = get_sets(game);
    let min_r: i32 = sets
        .iter()
        .map(|s| s.0)
        .max()
        .expect("cannot have a game with no sets");
    let min_g: i32 = sets
        .iter()
        .map(|s| s.1)
        .max()
        .expect("cannot have a game with no sets");
    let min_b: i32 = sets
        .iter()
        .map(|s| s.2)
        .max()
        .expect("cannot have a game with no sets");
    Set(min_r, min_g, min_b)
}

pub fn problem_1(input: String) -> i32 {
    input
        .par_lines()
        .filter(|game| is_possible(game.to_string()))
        .map(|game| get_game_id(game.to_string()))
        .sum()
}

pub fn problem_2(input: String) -> i32 {
    input
        .par_lines()
        .map(|game| get_min_bag(game.to_string()))
        .map(|bag: Set| bag.0 * bag.1 * bag.2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_game_id() {
        let games: Vec<i32> = fs::read_to_string("./data/examples/02/problem1.txt")
            .expect("error loading input")
            .lines()
            .map(|g: &str| get_game_id(g.to_string()))
            .collect();
        assert_eq!(games[0], 1);
        assert_eq!(games[1], 2);
        assert_eq!(games[2], 3);
        assert_eq!(games[3], 4);
        assert_eq!(games[4], 5);
        assert_eq!(games[5], 6);
    }

    #[test]
    fn test_sets() {
        let games: Vec<Vec<Set>> = fs::read_to_string("./data/examples/02/problem1.txt")
            .expect("error loading input")
            .lines()
            .map(|g: &str| get_sets(g.to_string()))
            .collect();
        assert_eq!(games[0], vec![Set(4, 0, 3), Set(1, 2, 6), Set(0, 2, 0)]);
        assert_eq!(games[1], vec![Set(0, 2, 1), Set(1, 3, 4), Set(0, 1, 1)]);
        assert_eq!(games[2], vec![Set(20, 8, 6), Set(4, 13, 5), Set(1, 5, 0)]);
        assert_eq!(games[3], vec![Set(3, 1, 6), Set(6, 3, 0), Set(14, 3, 15)]);
        assert_eq!(games[4], vec![Set(6, 3, 1), Set(1, 2, 2)]);
        assert_eq!(games[5], vec![Set(12, 0, 0), Set(1, 0, 0)]);
    }

    #[test]
    fn test_possible() {
        let games: Vec<bool> = fs::read_to_string("./data/examples/02/problem1.txt")
            .expect("error loading input")
            .lines()
            .map(|g: &str| is_possible(g.to_string()))
            .collect();
        assert_eq!(games[0], true);
        assert_eq!(games[1], true);
        assert_eq!(games[2], false);
        assert_eq!(games[3], false);
        assert_eq!(games[4], true);
        assert_eq!(games[5], true);
    }
    #[test]
    fn test_min_bag() {
        let bags: Vec<Set> = fs::read_to_string("./data/examples/02/problem1.txt")
            .expect("error loading input")
            .lines()
            .map(|b: &str| get_min_bag(b.to_string()))
            .collect();
        assert_eq!(bags[0], Set(4, 2, 6));
        assert_eq!(bags[1], Set(1, 3, 4));
        assert_eq!(bags[2], Set(20, 13, 6));
        assert_eq!(bags[3], Set(14, 3, 15));
        assert_eq!(bags[4], Set(6, 3, 2));
    }

    #[test]
    fn test_example_1() {
        let example =
            fs::read_to_string("./data/examples/02/problem1.txt").expect("error loading input");
        assert_eq!(problem_1(example), 14);
    }
    #[test]
    fn test_example_2() {
        let example =
            fs::read_to_string("./data/examples/02/problem2.txt").expect("error loading input");
        assert_eq!(problem_2(example), 2286);
    }
}
