pub fn recursive_accumulate<'a>(acc: &'a mut Vec<u64>, digits: &'a mut Vec<u64>) -> &'a Vec<u64> {
    match digits.pop() {
        None => return acc,
        Some(digit) if &digit >= acc.last().unwrap() => {
            let index = match acc.windows(2).rposition(|window| window[0] > window[1]) {
                Some(i) => i + 1,
                None => 0,
            };

            acc.remove(index);
            acc.push(digit);
        }
        _ => (),
    }

    recursive_accumulate(acc, digits)
}

pub fn handle_bank(bank: &str, length: usize) -> u64 {
    let mut digits = bank
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();
    let mut initial_acc = digits.split_off(digits.len() - length);
    digits.reverse();

    let primary = recursive_accumulate(&mut initial_acc, &mut digits);

    primary
        .into_iter()
        .rfold(0u64, |acc, digit| acc * 10 + digit)
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let result_a: u64 = input.iter().map(|bank| handle_bank(bank, 2)).sum();
    println!("Result A: {:?}", result_a);
    let result_b: u64 = input.iter().map(|bank| handle_bank(bank, 12)).sum();
    println!("Result B: {:?}", result_b);

    return Ok(());
}
