fn parse_input(input: &[String]) -> Vec<Vec<String>> {
    let max_len = input.iter().map(|s| s.len()).max().unwrap_or(0);
    let splits: Vec<usize> = (0..max_len)
        .filter(|&i| {
            input
                .iter()
                .all(|row| row.chars().nth(i).map(|c| c == ' ').unwrap_or(false))
        })
        .collect();

    let ranges: Vec<(usize, usize)> = std::iter::once(0)
        .chain(splits.iter().map(|&s| s + 1))
        .zip(splits.iter().copied().chain(std::iter::once(max_len)))
        .collect();

    ranges
        .iter()
        .map(|&(start, end)| {
            input
                .iter()
                .map(|row| row.get(start..end).unwrap_or(&row[start..]).to_string())
                .collect()
        })
        .collect()
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut columns = parse_input(&input);

    let mut sum: usize = 0;

    for column in columns.iter_mut() {
        let operator = column.pop().unwrap();
        let mut nrs: Vec<usize> = Vec::new();
        for i in 0..column[0].len().clone() {
            let nr = column.iter().fold(0, |acc: usize, value: &String| {
                let bla = value.clone().chars().nth(i).unwrap();
                match bla.to_digit(10) {
                    Some(x) => acc * 10 + x as usize,
                    None => acc,
                }
            });
            nrs.push(nr);
        }

        sum += match operator.trim() {
            "+" => nrs.iter().sum::<usize>(),
            "*" => nrs.iter().product(),
            _ => panic!("Should not be allowed"),
        }
    }
    println!("{:?}", sum);

    Ok(())
}
