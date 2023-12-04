#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Point(i32, i32);

fn find_symbols(rows: Vec<&str>) -> Vec<Point> {
    let mut symbols: Vec<Point> = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.char_indices() {
            if !c.is_alphanumeric() && c != '.' {
                symbols.push(Point(x.try_into().unwrap(), y.try_into().unwrap()))
            }
        }
    }
    symbols
}

fn find_targets(schematic: &str) -> Vec<Point> {
    let rows: Vec<&str> = schematic.lines().collect();
    let max_x = rows[0].len() as i32 - 1;
    let max_y = rows.len() as i32 - 1;

    let symbols = find_symbols(rows);

    let mut points = Vec::new();
    for p in symbols {
        let x = p.0;
        let y = p.1;

        #[rustfmt::skip]
        let possibilities = vec![
            Point(x-1, y+1), Point(x, y+1), Point(x + 1, y+1),
            Point(x-1, y), Point(x, y), Point(x + 1, y),
            Point(x-1, y-1), Point(x, y-1), Point(x + 1, y-1),
        ];

        for point in possibilities {
            if (0..=max_x).contains(&point.0)
                && (0..=max_y).contains(&point.1)
                && !points.contains(&point)
            {
                points.push(point)
            }
        }
    }
    points.sort();
    points
}

fn get_num_points(schematic: &str) -> Vec<Vec<(i32, Vec<Point>)>> {
    let mut number_loc: Vec<Vec<(i32, Vec<Point>)>> = Vec::new();
    for (y, line) in schematic.lines().enumerate() {
        let numbers: Vec<&str> = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|i| !i.is_empty())
            .collect();

        let mut line_number_loc: Vec<(i32, Vec<Point>)> = Vec::new();
        for n in numbers.iter() {
            let mut points: Vec<Point> = Vec::new();

            let start = line.find(n).expect("we know n is in line");
            let end = start + n.len() - 1;
            let n: i32 = n.parse().expect("all numbers will be in the i32 range");

            for x in start..=end {
                points.push(Point(x as i32, y as i32))
            }

            line_number_loc.push((n, points));
        }

        number_loc.push(line_number_loc);
    }

    number_loc
}

pub fn problem_1(schematic: &str) -> i32 {
    let numbers: Vec<Vec<(i32, Vec<Point>)>> = get_num_points(schematic);
    let targets = find_targets(schematic);

    let mut final_numbers: Vec<i32> = Vec::new();
    for entry in numbers.into_iter() {
        for (n, points) in entry.into_iter() {
            for point in points {
                if targets.contains(&point) {
                    final_numbers.push(n)
                }
            }
        }
    }

    final_numbers.iter().sum()
}

// pub fn problem_2(input: &str) -> i32 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_symbols() {
        let example =
            fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
        let example: Vec<&str> = example.lines().collect();
        assert_eq!(
            find_symbols(example),
            vec![
                Point(3, 1),
                Point(6, 3),
                Point(3, 4),
                Point(5, 5),
                Point(3, 8),
                Point(5, 8),
                Point(3, 10),
                Point(5, 10),
                Point(8, 10),
                Point(9, 10),
                Point(0, 11),
                Point(3, 11),
                Point(5, 11),
                Point(8, 11),
                Point(9, 11)
            ]
        );
    }

    #[test]
    fn test_find_targets() {
        let example1 = r#"
            467..114..
            ...*......
            ..35..633.
        "#;
        let example1: String = example1
            .trim()
            .lines()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        #[rustfmt::skip]
        let mut expected1 = vec![
            Point(2, 0), Point(3, 0), Point(4, 0),
            Point(2, 1), Point(3, 1), Point(4, 1),
            Point(2, 2), Point(3, 2), Point(4, 2),
        ];
        expected1.sort();

        let example2 = r#"
            ...$.*....
            .664.598..
            ...$.*..*#
        "#;
        let example2: String = example2
            .trim()
            .lines()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        #[rustfmt::skip]
        let mut expected2 = vec![
            Point(2, 0), Point(3, 0), Point(4, 0), Point(5, 0), Point(6, 0),
            Point(2, 1), Point(3, 1), Point(4, 1), Point(5, 1), Point(6, 1),
                                                                             Point(7, 1), Point(8, 1), Point(9, 1),
            Point(2, 2), Point(3, 2), Point(4, 2), Point(5, 2), Point(6, 2), Point(7, 2), Point(8, 2), Point(9, 2),
        ];
        expected2.sort();

        assert_eq!(find_targets(&example1), expected1);
        assert_eq!(find_targets(&example2), expected2);
    }

    #[test]
    fn test_get_num_points() {
        let example1 = r#"
            467..114..
        "#;
        let example1: String = example1
            .trim()
            .lines()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        let expected1 = vec![
            (467, vec![Point(0, 0), Point(1, 0), Point(2, 0)]),
            (114, vec![Point(5, 0), Point(6, 0), Point(7, 0)]),
        ];

        let example2 = r#"
            ..35..633.
        "#;
        let example2: String = example2
            .trim()
            .lines()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        let expected2 = vec![
            (35, vec![Point(2, 0), Point(3, 0)]),
            (633, vec![Point(6, 0), Point(7, 0), Point(8, 0)]),
        ];

        assert_eq!(get_num_points(&example1), vec![expected1]);
        assert_eq!(get_num_points(&example2), vec![expected2]);
    }

    // #[test]
    // fn test_example_1() {
    //     let example =
    //         fs::read_to_string("./data/examples/03/problem1.txt").expect("error loading input");
    //     assert_eq!(problem_1(&example), 4499);
    // }
}
