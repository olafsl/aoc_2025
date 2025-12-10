use itertools::Itertools;

pub fn process_machine(input: String) -> usize {
    let mut buttons = input.split(" ").map(|x| x[1..x.len() - 1].to_string()).collect_vec();

    let _joltage = buttons.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
    let mut buttons = buttons.iter();
    let goal = buttons.next().unwrap().chars().map(|x| match x {
        '#' => 1,
        '.' => 0,
        _ => panic!("Invalid character: {}", x),
    }).collect_vec();
    let buttons = buttons.map(|x| x.split(',').map(|y| y.parse::<usize>().unwrap()).collect_vec()).collect_vec();

    let result = (1..).find(|i| buttons.iter().combinations_with_replacement(*i).any(|combination| {
        let combination = combination.iter().cloned().flatten().counts_by(|x| x);
        let valid = goal.iter().enumerate().all(|(key, value)| combination.get(&key).unwrap_or(&0) % 2 == *value);
        valid
    }
    ));
    match result {
        Some(x) => return x,
        None => panic!("No result found"),
    }
}

pub fn button_presses(input: Vec<String>) -> usize {
    input.iter().map(|line| process_machine(line.clone())).sum()
}

pub fn main(input: Vec<String>) {
    let button_presses = button_presses(input);
    println!("Button presses: {}", button_presses);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let input = include_str!("../examples/day_10.txt")
            .lines()
            .map(|line| line.to_string())
            .collect();
        let button_presses = button_presses(input);
        assert_eq!(button_presses, 7);
    }
}
