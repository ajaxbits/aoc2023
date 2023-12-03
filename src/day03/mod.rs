use rayon::prelude::*;

enum Color {
    Red,
    Blue,
}

fn get_symbol_indices(row: &str) -> Option<Vec<i32>> {
    let indices: Vec<i32> = row
        .par_char_indices()
        .filter_map(|(i, c)| match !c.is_alphanumeric() && c != '.' {
            true => Some(i as i32),
            false => None,
        })
        .collect();

    match !indices.is_empty() {
        true => Some(indices),
        false => None,
    }
}

fn color(row: &str) -> Option<Vec<(i32, i32)>> {
    let indices = get_symbol_indices(&row);
    indices.map(|indices: Vec<i32>| {
        indices
            .into_iter()
            .map(|i| {
                let max_len = row.len() as i32;
                let poss_low = i - 1;
                let poss_high = i + 1;
                match (poss_low >= 0, poss_high <= max_len) {
                    (true, true) => (poss_low, poss_high),
                    (false, true) => (i, poss_high),
                    (true, false) => (poss_low, i),
                    (false, false) => (i, i),
                }
            })
            .collect::<Vec<(i32, i32)>>()
    })
}

pub fn problem_1(input: &str) -> i32 {
    todo!()
}

pub fn problem_2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_symbol_indices() {
        let example =
            fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        assert_eq!(get_symbol_indices(example[0]), None);
        assert_eq!(get_symbol_indices(example[1]), Some(vec![3]));
        assert_eq!(get_symbol_indices(example[8]), Some(vec![3, 5]));
    }

    #[test]
    fn test_color() {
        let example =
            fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        assert_eq!(color(example[0]), None);
        assert_eq!(color(example[1]), Some(vec![(2, 4)]));
        assert_eq!(color(example[8]), Some(vec![(2, 6)]));
        assert_eq!(color(example[10]), Some(vec![(2, 6), (8, 10)]));
        assert_eq!(color(example[11]), Some(vec![(0, 1), (2, 6), (8, 10)]));
    }

    // #[test]
    // fn test_example_1() {
    //     let example =
    //         fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
    //     assert_eq!(problem_1(&example), 4361);
    // }
}
