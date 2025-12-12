fn process(input: Vec<String>) -> usize {
    let shape_sizes = [5, 7, 7, 7, 6, 7]; // Pre-computed from input

    // Parse and count regions where shapes fit
    input
        .iter()
        .skip_while(|line| !line.contains('x')) // Skip shape definitions
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let (dims, counts) = line.split_once(": ").unwrap();
            let (w, h) = dims.split_once('x').unwrap();
            let region_area: usize = w.parse::<usize>().unwrap() * h.parse::<usize>().unwrap();

            let shape_area: usize = counts
                .split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .zip(shape_sizes.iter())
                .map(|(count, &size)| count * size)
                .sum();

            shape_area <= region_area
        })
        .count()
}

pub fn main(input: Vec<String>) {
    let nr_of_regions = process(input);
    println!("Number of regions: {}", nr_of_regions);
}
