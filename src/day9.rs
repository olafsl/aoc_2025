use itertools::Itertools;

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let tiles = input.iter().map(|line| {
        let (a, b) = line.split_once(",").expect("Input malformed");
        (
            a.parse::<isize>().expect("Not Parsable"),
            b.parse::<isize>().expect("Not Parsable"),
        )
    });
    let area = tiles.tuple_combinations().fold(0, |acc, (a, b)| {
        acc.max((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
    });
    println!("{:?}", area);
    Ok(())
}
