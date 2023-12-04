pub fn problem_1(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_1() {
        let example =
            fs::read_to_string("./data/examples/04/problem1.txt").expect("error loading input");
        assert_eq!(problem_1(&example), 13);
    }
}
