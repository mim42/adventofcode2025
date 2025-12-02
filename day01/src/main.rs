use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let mut position = 50;
    let mut password = 0;
    input.iter().for_each(|f| {
        let sign = match f.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };
        position = (position + sign * f[1..].parse::<i32>().unwrap()) % 100;
        if position <= 0 {
            position = (position % 100 + 100) % 100;
        }
        if position == 0 {
            password += 1;
        }
    });
    password
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let mut position = 50;
    let mut password = 0;
    input.iter().for_each(|f| {
        let sign = match f.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };
        let previously_zero = position == 0;

        position = position + sign * f[1..].parse::<i32>().unwrap();
        if position > 99 {
            let number = position / 100;
            position = position % 100;
            password += number;
        } else if position < 0 {
            let number = position.abs() / 100;
            position = (position % 100 + 100) % 100;
            if !previously_zero {
                password += 1;
            }
            password += number;
        } else if position == 0 {
            password += 1;
        }
    });
    password
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i32 = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
    let result_part_b: i32 = solve_part_b(&input);
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
        assert_eq!(6, solve_part_b(&example));
    }
}
