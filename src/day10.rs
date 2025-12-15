use good_lp::{constraint, default_solver, variables, Expression, Solution, SolverModel};
use itertools::Itertools;

pub fn process_machine(input: String) -> usize {
    let mut parts = input
        .split(" ")
        .map(|x| x[1..x.len() - 1].to_string())
        .collect_vec();

    let target: Vec<f64> = parts
        .pop()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<f64>().unwrap())
        .collect_vec();
    let mut parts_iter = parts.iter();
    let _goal = parts_iter.next();
    let buttons: Vec<Vec<usize>> = parts_iter
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let num_positions = target.len();
    let num_buttons = buttons.len();

    let mut problem = variables!();

    let vars: Vec<_> = (0..num_buttons)
        .map(|_| problem.add(good_lp::variable().integer().min(0)))
        .collect();

    let objective: Expression = vars.iter().sum();

    let mut model = problem.minimise(&objective).using(default_solver);

    for i in 0..num_positions {
        let lhs: Expression = buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.contains(&i))
            .map(|(j, _)| vars[j])
            .sum();

        model = model.with(constraint!(lhs == target[i]));
    }

    let solution = model.solve().expect("No solution found");

    vars.iter().map(|&v| solution.value(v).round() as usize).sum()
}

pub fn button_presses(input: Vec<String>) -> usize {
    let start = std::time::Instant::now();
    let result = input.iter().map(|line| {
        println!("Processing line: {}", line);
        let line_start = std::time::Instant::now();
        let var = process_machine(line.clone());
        println!("Processed line: {}", line);
        println!("Time taken: {:?}", line_start.elapsed());
        var
    }).sum();

    let duration = start.elapsed();
    println!("Time taken Total: {:?}", duration);
    result
}

pub fn main(input: Vec<String>) {
    let button_presses = button_presses(input);
    println!("Button presses: {}", button_presses);
}
