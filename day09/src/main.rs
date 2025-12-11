use std::cmp::{max, min};
use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(usize, usize)> {
    input
        .iter()
        .map(|line| {
            let points = line
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (points[0], points[1])
        })
        .collect::<Vec<(usize, usize)>>()
}

fn calculate_area(point: &(usize, usize), other_point: &(usize, usize)) -> usize {
    (max(point.0, other_point.0) - min(point.0, other_point.0) + 1)
        * (max(point.1, other_point.1) - min(point.1, other_point.1) + 1)
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let points = parse_input(input);
    let mut pair_of_points: Vec<usize> = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pair_of_points.push(calculate_area(&points[i], &points[j]))
        }
    }

    *pair_of_points.iter().max().unwrap()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let points = parse_input(input);
    let n = points.len();

    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for i in 0..n {
        let prev = if i == 0 { n - 1 } else { i - 1 };
        let (e1, e2) = if points[i] <= points[prev] {
            (points[i], points[prev])
        } else {
            (points[prev], points[i])
        };
        edges.push((e1, e2));
    }

    let mut sizes: Vec<(usize, (usize, usize), (usize, usize))> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let (c1, c2) = if points[i] <= points[j] {
                (points[i], points[j])
            } else {
                (points[j], points[i])
            };
            sizes.push((calculate_area(&c1, &c2), c1, c2));
        }
    }

    sizes.sort_by(|a, b| b.0.cmp(&a.0));

    for (size, (x1, y1), (x2, y2)) in sizes {
        let (y1, y2) = (min(y1, y2), max(y1, y2));

        let rectangle_overlaps_with_edge = edges
            .iter()
            .any(|((x3, y3), (x4, y4))| *x4 > x1 && *x3 < x2 && *y4 > y1 && *y3 < y2);

        if !rectangle_overlaps_with_edge {
            return size;
        }
    }

    0
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
        assert_eq!(50, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(24, solve_part_b(&example));
    }
}
