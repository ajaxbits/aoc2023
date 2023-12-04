use rayon::{collections::linked_list, prelude::*};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

fn color_row(row: &str) -> Vec<Color> {
    let len = row.len();
    let indices: Option<Vec<i32>> = get_symbol_indices(&row);
    let mut colors: Vec<Color> = vec![Color::Red; len];
    if let Some(indices) = indices {
        for i in indices {
            colors[i as usize] = Color::Blue;
            if i > 0 {
                colors[(i - 1) as usize] = Color::Blue;
            }
            if i + 1 < len as i32 {
                colors[(i + 1) as usize] = Color::Blue;
            }
        }
    }
    colors
}

fn color_plane(schematic: &str) -> Vec<Vec<Color>> {
    let rows: Vec<&str> = schematic.lines().collect();
    let colored_rows: Vec<Vec<Color>> = rows.iter().map(|row| color_row(row)).collect();
    let line_len = rows[0].len();

    let prepared_rows: Vec<Vec<char>> = rows
        .into_iter()
        .map(|row: &str| row.chars().collect::<Vec<char>>())
        .zip(colored_rows.into_iter())
        .map(|(row, colors): (Vec<char>, Vec<Color>)| {
            row.into_iter()
                .zip(colors.into_iter())
                .map(|(c, color): (char, Color)| -> char {
                    if color == Color::Red {
                        c
                    } else {
                        '*'
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let mut cols: Vec<String> = Vec::new();
    for i in 1..line_len {
        let mut col: Vec<char> = Vec::new();
        for row in prepared_rows.iter() {
            col.push(row[i]);
        }
        let col: String = col.iter().collect();
        cols.push(col)
    }
    let colored_cols: Vec<Vec<Color>> = cols.iter().map(|s| color_row(s)).collect();

    todo!()
}

fn find_numbers(row: &str) -> Option<Vec<i32>> {
    todo!()
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
        use Color::{Blue, Red};

        let example =
            fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        assert_eq!(color_row(example[0]), vec![Red; 10]);
        assert_eq!(
            color_row(example[1]),
            vec![Red, Red, Blue, Blue, Blue, Red, Red, Red, Red, Red]
        );
        assert_eq!(
            color_row(example[10]),
            vec![Red, Red, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue]
        );
    }

    // #[test]
    // fn test_example_1() {
    //     let example =
    //         fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
    //     assert_eq!(problem_1(&example), 4361);
    // }
}
