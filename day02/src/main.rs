use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let mut invalid_numbers = 0;
    input.first().unwrap().split(',').for_each(|range| {
        let (lower, upper) = range.split_once('-').unwrap();
        let lower = lower.parse::<i64>().unwrap();
        let upper = upper.parse::<i64>().unwrap();

        for id in lower..=upper {
            let id_str = id.to_string();
            let id_str_len = id_str.len();

            if id_str_len % 2 == 0 {
                let first_pattern = &id_str[0..id_str_len / 2];
                if first_pattern.repeat(2) == id_str {
                    invalid_numbers += id;
                }
            }
        }
    });
    invalid_numbers
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let mut invalid_numbers = 0;
    input.first().unwrap().split(',').for_each(|range| {
        let (lower, upper) = range.split_once('-').unwrap();
        let lower = lower.parse::<i64>().unwrap();
        let upper = upper.parse::<i64>().unwrap();

        for id in lower..=upper {
            let id_str = id.to_string();
            let id_str_len = id_str.len();
            for pattern_length in 1..=id_str_len / 2 {
                if id_str_len % pattern_length == 0 {
                    let first_pattern = &id_str[0..pattern_length];
                    if first_pattern.repeat(id_str_len / pattern_length) == id_str {
                        invalid_numbers += id;
                        break;
                    }
                }
            }
        }
    });

    invalid_numbers
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
        assert_eq!(1227775554, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(4174379265, solve_part_b(&example));
    }
}
