use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut parsed_input = Vec::new();
    input.iter().for_each(|line| {
        parsed_input.push(
            line.split(" ")
                .filter(|c| *c != "")
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
        );
    });

    parsed_input
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let parsed_input = parse_input(input);
    let mut total = 0;
    for row in 0..parsed_input[0].len() {
        let mut math_row = parsed_input[0][row].parse::<i64>().unwrap();
        let operator = &parsed_input[parsed_input.len() - 1][row];
        for col in 1..parsed_input.len() - 1 {
            match operator.as_str() {
                "+" => math_row += parsed_input[col][row].parse::<i64>().unwrap(),
                "*" => math_row *= parsed_input[col][row].parse::<i64>().unwrap(),
                _ => (),
            }
        }
        total += math_row;
    }
    total
}

fn parse_input_b(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut parsed_input = Vec::new();

    let last_line = &input[input.len() - 1];
    let last_line_vec = last_line
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let mut positions = Vec::new();
    for i in 0..last_line.len() {
        if last_line_vec[i] == "+" || last_line_vec[i] == "*" {
            positions.push(i);
        }
    }

    input.iter().for_each(|line| {});

    parsed_input
}

fn build_numbers(column: &Vec<String>) -> Vec<i64> {
    let mut numbers = Vec::new();
    let max_length = column
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();
    for i in 0..max_length {
        let mut number = "".to_string();
        for item in &column[..column.len() - 1] {
            match item.chars().nth(i) {
                Some(num) => number = number + &num.to_string(),
                None => (),
            }
        }
        println!("{}", number);
        numbers.push(number.parse::<i64>().unwrap());
    }

    numbers
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let parsed_input = parse_input(input);
    let mut total = 0;
    for row in 0..parsed_input[0].len() {
        let mut math_row = parsed_input[0][row].parse::<i64>().unwrap();
        let operator = &parsed_input[parsed_input.len() - 1][row];
        let numbers = build_numbers(
            &parsed_input
                .iter()
                .map(|c| c[row].clone())
                .collect::<Vec<String>>(),
        );
        match operator.as_str() {
            "+" => math_row = numbers.iter().sum(),
            "*" => math_row = numbers.iter().product(),
            _ => (),
        }

        total += math_row;
    }
    total
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
        assert_eq!(4277556, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(3263827, solve_part_b(&example));
    }
}
