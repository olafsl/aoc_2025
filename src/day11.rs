use std::collections::HashMap;

use itertools::Itertools;

pub fn create_graph(input: Vec<String>) -> (HashMap<usize, Vec<usize>>, HashMap<String, usize>) {
    let mut graph = HashMap::new();
    let mut mapping = HashMap::new();
    for (index, line) in input.iter().enumerate() {
        let (from, _to) = line.split_once(":").unwrap();
        mapping.insert(from.to_string(), index);
    }
    mapping.insert("out".to_string(), input.len());

    for (index, line) in input.iter().enumerate() {
        let (_from, to) = line.split_once(":").unwrap();
        let followers = to
            .trim()
            .split(" ")
            .map(|x| mapping.get(x).unwrap().clone())
            .collect_vec();

        graph.insert(index, followers);
    }
    (graph, mapping)
}

fn expand_endpoints(
    endpoints: &HashMap<usize, usize>,
    graph: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (&node, &count) in endpoints {
        if let Some(followers) = graph.get(&node) {
            for &follower in followers {
                *result.entry(follower).or_insert(0) += count;
            }
        }
    }
    result
}

pub fn count_paths(input: Vec<String>) -> usize {
    let (graph, mapping) = create_graph(input);

    let mut nr_of_paths: usize = 0;

    let mut current: HashMap<usize, usize> = HashMap::new();
    let mut current_fft: HashMap<usize, usize> = HashMap::new();
    let mut current_dac: HashMap<usize, usize> = HashMap::new();
    let mut current_fft_dac: HashMap<usize, usize> = HashMap::new();

    let svr = *mapping.get("svr").unwrap();
    let out = *mapping.get("out").unwrap();
    let fft = *mapping.get("fft").unwrap();
    let dac = *mapping.get("dac").unwrap();

    current.insert(svr, 1);

    while !current.is_empty()
        || !current_fft.is_empty()
        || !current_dac.is_empty()
        || !current_fft_dac.is_empty()
    {
        let followers = expand_endpoints(&current, &graph);
        let followers_passed_fft = expand_endpoints(&current_fft, &graph);
        let followers_passed_dac = expand_endpoints(&current_dac, &graph);
        let followers_passed_fft_and_dac = expand_endpoints(&current_fft_dac, &graph);

        current.clear();
        current_fft.clear();
        current_dac.clear();
        current_fft_dac.clear();

        for (&endpoint, &count) in &followers {
            match endpoint {
                x if x == fft => *current_fft.entry(endpoint).or_insert(0) += count,
                x if x == dac => *current_dac.entry(endpoint).or_insert(0) += count,
                x if x == out => (),
                _ => *current.entry(endpoint).or_insert(0) += count,
            }
        }

        for (&endpoint, &count) in &followers_passed_dac {
            match endpoint {
                x if x == fft => *current_fft_dac.entry(endpoint).or_insert(0) += count,
                x if x == out => (),
                _ => *current_dac.entry(endpoint).or_insert(0) += count,
            }
        }

        for (&endpoint, &count) in &followers_passed_fft {
            match endpoint {
                x if x == dac => *current_fft_dac.entry(endpoint).or_insert(0) += count,
                x if x == out => (),
                _ => *current_fft.entry(endpoint).or_insert(0) += count,
            }
        }

        for (&endpoint, &count) in &followers_passed_fft_and_dac {
            match endpoint {
                x if x == out => nr_of_paths += count,
                _ => *current_fft_dac.entry(endpoint).or_insert(0) += count,
            }
        }
    }
    nr_of_paths
}

pub fn main(input: Vec<String>) {
    let nr_of_paths = count_paths(input);
    println!("Number of paths: {}", nr_of_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let input = include_str!("../examples/day_11.txt")
            .lines()
            .map(|line| line.to_string())
            .collect();
        let nr_of_paths = count_paths(input);
        assert_eq!(nr_of_paths, 2);
    }
}
