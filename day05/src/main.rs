use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();
    for line in input {
        if line.contains("-") {
            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            ranges.push((start, end));
        } else if line.len() > 0 {
            numbers.push(line.parse::<i64>().unwrap());
        }
    }
    (ranges, numbers)
}

fn overlapping_range(first: (i64, i64), second: (i64, i64)) -> Option<(i64, i64)> {
    if first.0 <= second.0 && first.1 >= second.0 {
        if first.1 >= second.1 {
            return Some((first.0, first.1));
        } else {
            return Some((first.0, second.1));
        }
    } else if second.1 >= first.0 && second.1 <= first.1 {
        return Some((second.0, first.1));
    }

    None
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let (ranges, numbers) = parse_input(input);
    let mut fresh_count = 0;
    for number in numbers {
        for range in &ranges {
            if number >= range.0 && number <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let (mut ranges, _) = parse_input(input);
    let mut index = 0;
    loop {
        let mut overlapping_ranges = Vec::new();
        if index == ranges.len() - 1 {
            break;
        }

        let range = ranges[index];
        let mut flag = false;
        for next_range in ranges {
            if next_range == range {
                continue;
            }
            match overlapping_range(range, next_range) {
                Some(overlap) => {
                    overlapping_ranges.push(overlap);
                    index = 0;
                    flag = true;
                }
                None => {
                    overlapping_ranges.push(next_range);
                }
            }
        }
        if !flag {
            overlapping_ranges.push(range);
            index += 1;
        }

        ranges = overlapping_ranges.clone();
    }

    ranges
        .iter()
        .map(|(a, b)| b - a + 1)
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>() as i64
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
        assert_eq!(3, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(14, solve_part_b(&example));
    }
}
