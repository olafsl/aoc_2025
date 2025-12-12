use itertools::Itertools;

pub fn process_machine(input: String) -> usize {
    let mut parts = input
        .split(" ")
        .map(|x| x[1..x.len() - 1].to_string())
        .collect_vec();

    let joltage = parts
        .pop()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();

    let mut parts = parts.iter();
    let _goal = parts.next();

    // Each button is a list of column indices it affects
    let buttons: Vec<Vec<usize>> = parts
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let num_columns = joltage.len();
    let num_buttons = buttons.len();

    // For each column, which buttons affect it?
    let mut column_to_buttons: Vec<Vec<usize>> = vec![Vec::new(); num_columns];
    for (button_idx, button_cols) in buttons.iter().enumerate() {
        for &col in button_cols {
            column_to_buttons[col].push(button_idx);
        }
    }

    // Calculate upper bound for each button based on column constraints
    let mut upper_bounds: Vec<usize> = vec![usize::MAX; num_buttons];
    for (col_idx, &target) in joltage.iter().enumerate() {
        for &button_idx in &column_to_buttons[col_idx] {
            upper_bounds[button_idx] = upper_bounds[button_idx].min(target);
        }
    }

    // Replace MAX with a reasonable bound
    for i in 0..num_buttons {
        if upper_bounds[i] == usize::MAX {
            upper_bounds[i] = joltage.iter().max().copied().unwrap_or(0);
        }
    }

    // Search for minimum total presses
    let mut best = usize::MAX;
    let mut current_presses = vec![0usize; num_buttons];

    search_minimum(
        &joltage,
        &column_to_buttons,
        &buttons,
        &upper_bounds,
        &mut current_presses,
        0,
        0,
        &mut best,
    );
    println!("Best: {}", best);

    best
}

fn search_minimum(
    joltage: &[usize],
    column_to_buttons: &[Vec<usize>],
    buttons: &[Vec<usize>],
    upper_bounds: &[usize],
    current_presses: &mut [usize],
    button_idx: usize,
    current_sum: usize,
    best: &mut usize,
) {
    // Pruning: if current sum already >= best, stop
    if current_sum >= *best {
        return;
    }

    if button_idx == current_presses.len() {
        // Check if all columns are satisfied
        for (col_idx, &target) in joltage.iter().enumerate() {
            let sum: usize = column_to_buttons[col_idx]
                .iter()
                .map(|&b| current_presses[b])
                .sum();
            if sum != target {
                return;
            }
        }
        *best = current_sum;
        return;
    }

    // Calculate tighter bounds for this button based on current state
    let mut min_count = 0usize;
    let mut max_count = upper_bounds[button_idx];

    // For each column this button affects
    for &col in &buttons[button_idx] {
        let target = joltage[col];

        // Sum from buttons already assigned (indices < button_idx)
        let assigned_sum: usize = column_to_buttons[col]
            .iter()
            .filter(|&&b| b < button_idx)
            .map(|&b| current_presses[b])
            .sum();

        // If already exceeded, this path is invalid
        if assigned_sum > target {
            return;
        }

        let remaining = target - assigned_sum;

        // Max possible contribution from future buttons (excluding current)
        let future_max: usize = column_to_buttons[col]
            .iter()
            .filter(|&&b| b > button_idx)
            .map(|&b| upper_bounds[b])
            .sum();

        // Current button must contribute at least (remaining - future_max)
        if remaining > future_max {
            min_count = min_count.max(remaining - future_max);
        }

        // Current button can contribute at most 'remaining'
        max_count = max_count.min(remaining);
    }

    if min_count > max_count {
        return;
    }

    // Try each possible count in order (start with minimum for faster pruning)
    for count in min_count..=max_count {
        current_presses[button_idx] = count;

        search_minimum(
            joltage,
            column_to_buttons,
            buttons,
            upper_bounds,
            current_presses,
            button_idx + 1,
            current_sum + count,
            best,
        );
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
