use std::{ops::RangeInclusive, u128};

pub fn compare_ranges(a: RangeInclusive<u128>, b: RangeInclusive<u128>) {
    match (a.start(), a.end()) {
        (start, end) if end < b.start() || start > b.end() => (a, b),
        (start, end) if end 


    }
}



pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = input.clone();
    let empty_line = input.iter().position(|x| x.len() == 0).unwrap();
    let available_ingredients = input.split_off(empty_line);

    let mut fresh_ingredient_ranges: Vec<RangeInclusive<u128>> = input
        .iter()
        .map(|x: &String| x.split_once("-").unwrap())
        .map(|(a, z)| (a.parse::<u128>().unwrap(), z.parse::<u128>().unwrap()))
        .map(|(a, z)| a..=z)
        .collect();
    let available_ingredients = available_ingredients
        .iter()
        .skip(1)
        .map(|x| x.parse::<u128>().unwrap());

    let result_a = available_ingredients
        .filter(|ingredient_id| {
            fresh_ingredient_ranges
                .iter()
                .any(|range: &RangeInclusive<_>| range.contains(ingredient_id))
        })
        .count();

    let mut completed_ranges: Vec<RangeInclusive<u128>> = Vec::new();
    let mut sum = 0;
    for range in fresh_ingredient_ranges {
        println!("Range: {:?}", range);
        let vals = range.clone().filter(|val| !completed_ranges.iter().any(|completed: &RangeInclusive<u128>| completed.contains(val)));
        sum += vals.count();
        completed_ranges.push(range);
    }


    println!("Result A: {:?}", result_a);
    println!("Result B: {:?}", sum);

    Ok(())
}
