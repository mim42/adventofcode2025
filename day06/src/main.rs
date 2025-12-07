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

fn apply_operation(op: &String, numbers: &Vec<String>) -> i64 {
    let mut answer = 0;
    match op.as_str() {
        "+" => {
            let num: i64 = numbers.iter().map(|c| c.parse::<i64>().unwrap()).sum();
            answer += num;
        }
        "*" => {
            let num: i64 = numbers.iter().map(|c| c.parse::<i64>().unwrap()).product();
            answer += num;
        }
        _ => (),
    }
    answer
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let reversed_input = input
        .iter()
        .rev()
        .map(|s| s.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let col_len = input.len();
    let row_len = input[0].len();

    let mut current_operator = &reversed_input[0][0];
    let mut answer: i64 = 0;
    let mut current_numbers: Vec<String> = Vec::new();
    for row in 0..row_len {
        // after every operation change we apply the operations
        // (thats why we need to call this one more time at the end of the loop)
        if (reversed_input[0][row] == "*" || reversed_input[0][row] == "+")
            && current_numbers.len() > 0
        {
            answer += apply_operation(current_operator, &current_numbers);
            current_operator = &reversed_input[0][row];
            current_numbers.clear();
        }

        // building the numbers ignoring spaces and add them to a temp vec to calcualte their product or sum later
        let mut current_number = "".to_string();
        for col in 1..col_len {
            if reversed_input[col][row] != " " {
                current_number = reversed_input[col][row].clone() + &current_number;
            }
        }
        if current_number != "" {
            current_numbers.push(current_number);
        }
    }
    //apply one last time at the end
    answer += apply_operation(current_operator, &current_numbers);
    answer
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
