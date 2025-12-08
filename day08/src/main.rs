use std::cmp::{max, min};
use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(usize, usize, usize)> {
    input
        .iter()
        .map(|line| {
            let points = line
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (points[0], points[1], points[2])
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

fn calculate_distance(point: &(usize, usize, usize), other_point: &(usize, usize, usize)) -> f64 {
    ((point.0 as f64 - other_point.0 as f64).powf(2.0)
        + ((point.1 as f64 - other_point.1 as f64).powf(2.0))
        + (point.2 as f64 - other_point.2 as f64).powf(2.0))
    .sqrt()
}

fn solve_part_a(input: &Vec<String>, number_of_nodes: usize) -> usize {
    let points = parse_input(input);
    let mut circuits: Vec<HashSet<(usize, usize, usize)>> = Vec::new();
    let mut point_distances: Vec<([(usize, usize, usize); 2], f64)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            point_distances.push((
                [points[i], points[j]],
                calculate_distance(&points[i], &points[j]),
            ));
        }
    }

    point_distances.sort_by(|(_, distance_a), (_, distance_b)| distance_a.total_cmp(distance_b));
    let mut points_iter = point_distances.iter();
    for _ in 0..number_of_nodes {
        let (points_pair, _) = points_iter.next().unwrap();

        let point_a_exists = circuits
            .iter()
            .any(|circuit| circuit.contains(&points_pair[0]));
        let point_b_exists = circuits
            .iter()
            .any(|circuit| circuit.contains(&points_pair[1]));
        if point_a_exists {
            if point_b_exists {
                let circuit_a_position = circuits
                    .iter_mut()
                    .position(|circuit| circuit.contains(&points_pair[0]))
                    .unwrap();
                let circuit_b_position = circuits
                    .iter_mut()
                    .position(|circuit| circuit.contains(&points_pair[1]))
                    .unwrap();

                if circuit_a_position != circuit_b_position {
                    let max_position = max(circuit_a_position, circuit_b_position);
                    let min_position = min(circuit_a_position, circuit_b_position);
                    let removed = circuits.remove(max_position);
                    circuits[min_position].extend(removed);
                }
            } else {
                let circuit = circuits
                    .iter_mut()
                    .find(|circuit| circuit.contains(&points_pair[0]))
                    .unwrap();
                circuit.insert(points_pair[1]);
            }
        } else if point_b_exists {
            let circuit = circuits
                .iter_mut()
                .find(|circuit| circuit.contains(&points_pair[1]))
                .unwrap();
            circuit.insert(points_pair[0]);
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(points_pair[0]);
            circuit.insert(points_pair[1]);
            circuits.push(circuit);
        }
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits.reverse();
    circuits[0..3].iter().map(|a| a.len()).product()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let points = parse_input(input);
    let mut circuits: Vec<HashSet<(usize, usize, usize)>> = Vec::new();
    let mut point_distances: Vec<([(usize, usize, usize); 2], f64)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            point_distances.push((
                [points[i], points[j]],
                calculate_distance(&points[i], &points[j]),
            ));
        }
    }

    point_distances.sort_by(|(_, distance_a), (_, distance_b)| distance_a.total_cmp(distance_b));
    let mut points_iter = point_distances.iter();
    let mut last_pair_points: [(usize, usize, usize); 2] = [(0, 0, 0), (0, 0, 0)];
    loop {
        let points_pair;
        match points_iter.next() {
            Some((k, _)) => points_pair = k,
            None => break,
        };

        let point_a_exists = circuits
            .iter()
            .any(|circuit| circuit.contains(&points_pair[0]));
        let point_b_exists = circuits
            .iter()
            .any(|circuit| circuit.contains(&points_pair[1]));
        if point_a_exists {
            if point_b_exists {
                let circuit_a_position = circuits
                    .iter_mut()
                    .position(|circuit| circuit.contains(&points_pair[0]))
                    .unwrap();
                let circuit_b_position = circuits
                    .iter_mut()
                    .position(|circuit| circuit.contains(&points_pair[1]))
                    .unwrap();

                if circuit_a_position != circuit_b_position {
                    let max_position = max(circuit_a_position, circuit_b_position);
                    let min_position = min(circuit_a_position, circuit_b_position);
                    let removed = circuits.remove(max_position);
                    circuits[min_position].extend(removed);
                    last_pair_points = *points_pair;
                }
            } else {
                let circuit = circuits
                    .iter_mut()
                    .find(|circuit| circuit.contains(&points_pair[0]))
                    .unwrap();
                circuit.insert(points_pair[1]);
                last_pair_points = *points_pair;
            }
        } else if point_b_exists {
            let circuit = circuits
                .iter_mut()
                .find(|circuit| circuit.contains(&points_pair[1]))
                .unwrap();
            circuit.insert(points_pair[0]);
            last_pair_points = *points_pair;
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(points_pair[0]);
            circuit.insert(points_pair[1]);
            circuits.push(circuit);
        }
    }
    last_pair_points[0].0 * last_pair_points[1].0
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: usize = solve_part_a(&input, 1000);
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
        assert_eq!(40, solve_part_a(&example, 10));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(25272, solve_part_b(&example));
    }
}
