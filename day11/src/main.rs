use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    input
        .iter()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            (
                a.to_string(),
                b.split(" ").map(|c| c.to_string()).collect::<Vec<String>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn dfs(
    current: &str,
    graph: &HashMap<String, Vec<String>>,
    end: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if current == end {
        return 1;
    }

    if let Some(cached) = cache.get(current) {
        return *cached;
    }

    let count = graph
        .get(current)
        .map(|neighbors| neighbors.iter().map(|n| dfs(n, graph, end, cache)).sum())
        .unwrap_or(0);

    cache.insert(current.to_string(), count);
    count
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let graph = parse_input(input);
    find_path_length(&graph, "you", "out")
}

fn find_path_length(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut cache = HashMap::new();
    dfs(start, graph, end, &mut cache)
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let graph = parse_input(input);

    let svr_to_fft = find_path_length(&graph, "svr", "fft");
    let svr_to_dac = find_path_length(&graph, "svr", "dac");

    let dac_to_fft = find_path_length(&graph, "dac", "fft");
    let fft_to_dac = find_path_length(&graph, "fft", "dac");

    let fft_to_out = find_path_length(&graph, "fft", "out");
    let dac_to_out = find_path_length(&graph, "dac", "out");

    (svr_to_fft * fft_to_dac * dac_to_out) + (svr_to_dac * dac_to_fft * fft_to_out)
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
        assert_eq!(5, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(2, solve_part_b(&example));
    }
}
