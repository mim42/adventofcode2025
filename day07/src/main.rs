use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let mut parsed_input = parse_input(input);
    let mut counter = 0;
    for col in 0..parsed_input.len() - 1 {
        for row in 0..parsed_input[0].len() {
            if parsed_input[col][row] == "S" || parsed_input[col][row] == "|" {
                if parsed_input[col + 1][row] == "^" {
                    parsed_input[col + 1][row - 1] = "|".to_string();
                    parsed_input[col + 1][row + 1] = "|".to_string();
                    counter += 1;
                } else {
                    parsed_input[col + 1][row] = "|".to_string()
                }
            }
        }
    }
    counter
}

fn count_timelines(
    map: &Vec<Vec<String>>,
    starting_point: (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let (col, row) = starting_point;
    match cache.get(&starting_point) {
        Some(k) => return *k,
        None => {
            let mut counter;
            if col == map.len() - 1 {
                counter = 1;
            } else {
                if map[col + 1][row] == "." {
                    counter = count_timelines(map, (col + 1, row), cache);
                } else {
                    counter = count_timelines(map, (col + 1, row + 1), cache)
                        + count_timelines(map, (col + 1, row - 1), cache);
                }
            }
            cache.insert(starting_point, counter);
            return counter;
        }
    }
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let parsed_input = parse_input(input);
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let starting_row = parsed_input[0].iter().position(|s| s == "S").unwrap();
    count_timelines(&parsed_input, (0, starting_row), &mut cache) as i64
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i64 = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
    let result_part_b: i64 = solve_part_b(&input);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(21, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(40, solve_part_b(&example));
    }
}
