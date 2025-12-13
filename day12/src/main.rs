use std::fs::read_to_string;
use std::usize;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> (Vec<([usize; 2], Vec<usize>)>, Vec<Vec<Vec<bool>>>) {
    let mut puzzle: Vec<([usize; 2], Vec<usize>)> = Vec::new();

    let mut pieces: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut current_piece: Vec<Vec<bool>> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        if i > 29 {
            pieces.push(current_piece.clone());
            let (a, b) = line.split_once("x").unwrap();
            let (b, c) = b.split_once(": ").unwrap();
            puzzle.push((
                [a.parse().unwrap(), b.parse().unwrap()],
                c.split(" ")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        } else {
            if line.len() > 2 {
                let a: Vec<bool> = line.chars().map(|c| c == '#').collect();
                current_piece.push(a);
            }
            if i % 5 == 0 && i != 0 {
                pieces.push(current_piece.clone());
                current_piece.clear();
            }
        }
    }

    (puzzle, pieces)
}

fn count_area(piece: &Vec<Vec<bool>>) -> usize {
    piece.iter().map(|c| c.iter().filter(|a| **a).count()).sum()
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let (puzzles, pieces) = parse_input(input);
    let mut sum: usize = 0;
    for (area, puzzle) in puzzles {
        let total_area = area[0] * area[1];
        let mut estimated_area = 0;

        for (puzzle_id, times) in puzzle.iter().enumerate() {
            estimated_area += count_area(&pieces[puzzle_id]) * times
        }

        if total_area >= estimated_area {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: usize = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(2, solve_part_a(&example));
    }
}
