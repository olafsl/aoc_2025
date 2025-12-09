use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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

pub fn calculate_distances(boxes: &Vec<ElectricalBox>) -> HashMap<(usize, usize), isize> {
    let mut distances_memoization: HashMap<(usize, usize), isize> = HashMap::new();
    for ((i, box_i), (j, box_j)) in boxes.iter().enumerate().tuple_combinations() {
        distances_memoization.insert((i, j), box_i.distance(box_j));
    }
    distances_memoization
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let boxes: Vec<ElectricalBox> = input
        .into_iter()
        .map(ElectricalBox::new)
        .collect::<Vec<ElectricalBox>>();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    let distances_memoization = calculate_distances(&boxes);
    let valid_pairs = distances_memoization
        .iter()
        .sorted_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
        .map(|(idx, _)| idx)
        .collect_vec();

    for (i, j) in valid_pairs {
        let mut circuit_a = match circuits.iter().position(|x| x.contains(i)) {
            Some(x) => circuits.remove(x),
            None => vec![i.clone()],
        };

        if circuit_a.contains(j) {
            circuits.push(circuit_a);
            continue;
        }

        let mut circuit_b = match circuits.iter().position(|x| x.contains(j)) {
            Some(x) => circuits.remove(x),
            None => vec![j.clone()],
        };
        let _ = circuit_a.append(&mut circuit_b);

        circuits.push(circuit_a);

        if circuits[0].len() == boxes.len() {
            println!("{:?}", circuits);
            println!("{:?}, {:?}", i, j);
            let box_a = &boxes[*i].clone();
            let box_b = &boxes[*j].clone();

            println!("{:?}", box_a.0 * box_b.0);
            break;
        }
    }

    Ok(())
}
