use itertools::Itertools;

pub fn process_machine(input: String) -> usize {
    let mut buttons = input.split(" ").map(|x| x[1..x.len() - 1].to_string()).collect_vec();

    let joltage = buttons.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
    let mut buttons = buttons.iter();
    let _goal = buttons.next();
    let buttons = buttons.map(|x| x.split(',').map(|y| y.parse::<usize>().unwrap()).collect_vec()).map(|x| {
        (0..joltage.len()).map(|y| x.contains(&y)).collect_vec()
    }).collect_vec();

    let result = (1..).find(|i| buttons.iter().combinations_with_replacement(*i).any(|combination| {
        println!("{:?}", combination);
        true
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
        assert_eq!(button_presses, 33);
    }
}
