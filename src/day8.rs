use itertools::Itertools;
use std::{collections::HashMap, iter};

#[derive(Debug)]
pub struct ElectricalBox(isize, isize, isize);

impl ElectricalBox {
    pub fn new(input: String) -> ElectricalBox {
        let Some((x, y, z)) = input
            .split(",")
            .map(|x| x.parse::<isize>().expect("Malformed input"))
            .collect_tuple()
        else {
            panic!("Malformed input")
        };
        ElectricalBox { 0: x, 1: y, 2: z }
    }

    pub fn distance(&self, other: &Self) -> isize {
        let x = (self.0 - other.0).pow(2);
        let y = (self.1 - other.1).pow(2);
        let z = (self.2 - other.2).pow(2);
        x + y + z
    }
}

pub fn calculate_distances(boxes: Vec<ElectricalBox>) -> HashMap<(usize, usize), isize> {
    let mut distances_memoization: HashMap<(usize, usize), isize> = HashMap::new();
    for ((i, box_i), (j, box_j)) in boxes.iter().enumerate().tuple_combinations() {
        distances_memoization.insert((i, j), box_i.distance(box_j));
    }
    distances_memoization
}

pub fn is_valid_circuit_combinations((circuit_a, circuit_b): &(&Vec<usize>, &Vec<usize>), valid_id_pairs: &Vec<&(usize, usize)>) -> bool {
    circuit_a.iter().cartesian_product(circuit_b.iter()).any(|(x, y)| valid_id_pairs.contains(&&(*x, *y)))
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let boxes: Vec<ElectricalBox> = input
        .into_iter()
        .map(ElectricalBox::new)
        .collect::<Vec<ElectricalBox>>();
    let mut circuits: Vec<Vec<usize>> =
        boxes.iter().enumerate().map(|(idx, _)| vec![idx]).collect();
    let iterations = match boxes.len() {
        20 => 10,
        1000 => 1000,
        _ => panic!("No nr of iterations found"),
    };

    let mut distances_memoization = calculate_distances(boxes);
    let last_distance = distances_memoization.values().sorted().take(iterations).last().unwrap().clone();
    distances_memoization.retain(|_key, value| *value <= last_distance );
    let valid_pairs = distances_memoization.keys().collect_vec();

    println!("{:?}", valid_pairs);
    println!("Calculated distances.");

    for (i, j) in valid_pairs {
        println!("{:?}", circuits);
        println!("{:?}, {:?}", i, j);
        let Some(position_box_a) = circuits
            .iter()
            .position(|x| x.contains(i)) else {continue;};
        let mut circuit_a = circuits.remove(position_box_a);
        let Some(position_box_b) = circuits
            .iter()
            .position(|x| x.contains(j)) else {continue;};
        let mut circuit_b = circuits.remove(position_box_b);
        let _ = circuit_a.append(&mut circuit_b);

        circuits.push(circuit_a);
    }

    println!("{:?}", circuits);
    let mut lengths = circuits
        .iter()
        .map(|x| x.len())
        .sorted()
        .rev()
        .collect::<Vec<usize>>();
    println!("{:?}", lengths);

    let _ = lengths.split_off(3);
    let result: usize = lengths.iter().product();
    println!("{:?}", result);

    Ok(())
}
