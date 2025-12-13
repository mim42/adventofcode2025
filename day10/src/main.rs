use std::cmp::{max, min};
use std::usize;
use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(Vec<usize>, Vec<Vec<usize>>, Vec<usize>)> {
    let mut parsed_input: Vec<(Vec<usize>, Vec<Vec<usize>>, Vec<usize>)> = Vec::new();
    for line in input {
        let mut lights: Vec<usize> = Vec::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut joltage: Vec<usize> = Vec::new();

        let mut category = 0;

        let start_bytes = line.find("[").unwrap() + 1;
        let end_bytes = line.find("]").unwrap();

        let r = &line[start_bytes..end_bytes];

        r.chars().for_each(|c| {
            if c == '.' {
                lights.push(0);
            } else {
                lights.push(1);
            }
        });

        let start_bytes = line.find("(").unwrap() + 1;
        let mut new_line = &line[start_bytes..];
        loop {
            let end_bytes = new_line.find(")").unwrap();
            let r = &new_line[0..end_bytes];

            buttons.push(
                r.split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
            match new_line.find("(") {
                Some(start) => new_line = &new_line[start + 1..],
                None => break,
            }
        }

        let start_bytes = line.find("{").unwrap() + 1;
        let end_bytes = line.find("}").unwrap();
        let r = &line[start_bytes..end_bytes];
        r.split(",").for_each(|c| {
            joltage.push(c.parse::<usize>().unwrap());
        });

        parsed_input.push((lights, buttons, joltage));
    }

    parsed_input
}

fn compare_vec(vec1: &Vec<usize>, vec2: &Vec<usize>) -> bool {
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }
    true
}

fn bfs(lights: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let initial_lights = vec![0; lights.len()];
    let mut queue: Vec<(usize, Vec<usize>, Vec<usize>)> = Vec::new();
    queue.insert(0, (0, initial_lights, vec![]));

    loop {
        let (depth, current_lights, mut buttons_pressed_before) = queue.remove(0);
        for (i, button) in buttons.iter().enumerate() {
            if buttons_pressed_before.contains(&i) {
                continue;
            } else {
                buttons_pressed_before.push(i);
            }
            let mut next_lights = current_lights.clone();
            for light_id in button {
                next_lights[*light_id] = 1 - next_lights[*light_id];
            }

            if compare_vec(&next_lights, &lights) {
                return depth + 1;
            }
            queue.push((depth + 1, next_lights, buttons_pressed_before.clone()));
        }
    }
    unreachable!()
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let parsed_input = parse_input(input);
    let mut sum = 0;

    for (lights, buttons, _) in parsed_input {
        sum += bfs(lights, buttons);
    }
    sum
}

fn possible_button_presses(
    starting_joltage: Vec<usize>,
    buttons: &Vec<Vec<usize>>,
) -> Vec<(usize, Vec<usize>)> {
    let mut buttons_pressed = Vec::new();

    // Get parities (1 = odd, 0 = even)
    let parities: Vec<usize> = starting_joltage.iter().map(|n| n % 2).collect();

    // Try all combinations of button presses
    // pressing twice does not change thet parity so we dont have to compute that
    for mask in 0..(1u64 << buttons.len()) {
        let mut new_parities = parities.clone();
        let mut presses = 0;

        for i in 0..buttons.len() {
            if (mask >> i) & 1 == 1 {
                presses += 1;
                for &idx in &buttons[i] {
                    new_parities[idx] ^= 1;
                }
            }
        }

        if new_parities.iter().all(|&p| p == 0) {
            // Count how many times each joltage is decreased
            let mut decreased = vec![0; starting_joltage.len()];
            for i in 0..buttons.len() {
                if (mask >> i) & 1 == 1 {
                    for &idx in &buttons[i] {
                        decreased[idx] += 1;
                    }
                }
            }

            // Check if we do not go below 0 by our presses
            let valid = starting_joltage
                .iter()
                .zip(decreased.iter())
                .all(|(&val, &dec)| val >= dec);

            if valid {
                let halved: Vec<usize> = starting_joltage
                    .iter()
                    .zip(decreased.iter())
                    .map(|(&val, &dec)| (val - dec) / 2)
                    .collect();
                buttons_pressed.push((presses, halved));
            }
        }
    }

    buttons_pressed
}

fn dfs(starting_joltage: Vec<usize>, buttons: &Vec<Vec<usize>>) -> usize {
    if starting_joltage.iter().all(|c| *c == 0) {
        return 0;
    }
    if starting_joltage.iter().all(|n| n % 2 == 0) {
        let next_joltage = starting_joltage.iter().map(|n| n / 2).collect();
        return 2 * dfs(next_joltage, buttons);
    } else {
        return possible_button_presses(starting_joltage, buttons)
            .iter()
            .map(|(button_presses, next_joltage)| {
                button_presses + (2 * dfs(next_joltage.clone(), buttons))
            })
            .collect::<Vec<usize>>()
            .into_iter()
            .min()
            .unwrap_or(100000);
    }
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let parsed_input = parse_input(input);
    parsed_input
        .into_iter()
        .map(|(_, buttons, joltage)| dfs(joltage, &buttons))
        .sum()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: usize = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
    let result_part_b: usize = solve_part_b(&input);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(7, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(33, solve_part_b(&example));
    }
}
