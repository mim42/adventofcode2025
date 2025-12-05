use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn count_neighbors(map: &Vec<Vec<String>>, row: usize, col: usize) -> i64 {
    let directions: Vec<(i64, i64)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut empty_neighbors = 0;
    for (dr, dc) in directions.iter() {
        if map[(row as i64 + dr) as usize][(col as i64 + dc) as usize] == "@" {
            empty_neighbors += 1;
        }
    }
    empty_neighbors
}

fn create_map(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut map = input
        .iter()
        .map(|line| {
            let mut line = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
            line.push(".".to_string());
            line.insert(0, ".".to_string());
            line
        })
        .collect::<Vec<Vec<String>>>();
    map.push(vec![".".to_string(); map[0].len()]);
    map.insert(0, vec![".".to_string(); map[0].len()]);
    map
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let mut count_rolls = 0;
    let map = create_map(input);
    for row in 1..map.len() - 1 {
        for col in 1..map[0].len() - 1 {
            if map[row][col] == "@" {
                if count_neighbors(&map, row, col) < 4 {
                    count_rolls += 1;
                }
            }
        }
    }
    count_rolls
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let mut count_rolls = 0;
    let mut map = create_map(input);
    loop {
        let mut to_be_removed: Vec<(usize, usize)> = Vec::new();

        for row in 1..map.len() - 1 {
            for col in 1..map[0].len() - 1 {
                if map[row][col] == "@" {
                    if count_neighbors(&map, row, col) < 4 {
                        to_be_removed.push((row, col));
                    }
                }
            }
        }

        to_be_removed.iter().for_each(|(row, col)| {
            map[*row][*col] = ".".to_string();
        });

        if to_be_removed.len() == 0 {
            break;
        }
        count_rolls += to_be_removed.len();
    }

    count_rolls
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
        assert_eq!(13, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(43, solve_part_b(&example));
    }
}
