use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i64 {
    let mut joltage_output = 0;

    input.iter().for_each(|line| {
        let batteries = line
            .chars()
            .map(|battery| battery.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let first_high = batteries[..batteries.len() - 1].iter().max().unwrap();

        let first_high_pos = batteries.iter().position(|x| x == first_high).unwrap();

        let second_high = batteries[first_high_pos + 1..].iter().max().unwrap();

        joltage_output += (first_high.to_string() + &second_high.to_string())
            .parse::<i64>()
            .unwrap();
    });

    joltage_output
}

fn solve_part_b(input: &Vec<String>) -> i64 {
    let mut joltage_output = 0;

    input.iter().for_each(|line| {
        let batteries = line
            .chars()
            .map(|battery| battery.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut joltage = "".to_string();
        let mut previous_high_pos = 0;

        for i in 0..12 {
            let next_possible_high = batteries[previous_high_pos..batteries.len() - 11 + i]
                .iter()
                .max()
                .unwrap();
            previous_high_pos = batteries[previous_high_pos..batteries.len() - 11 + i]
                .iter()
                .position(|x| x == next_possible_high)
                .unwrap()
                + 1
                + previous_high_pos;

            joltage = joltage.to_string() + &next_possible_high.to_string();
        }
        joltage_output += joltage.parse::<i64>().unwrap();
    });

    joltage_output
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
        assert_eq!(357, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(3121910778619, solve_part_b(&example));
    }
}
