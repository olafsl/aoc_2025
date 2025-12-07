use std::collections::HashMap;

use itertools::Itertools;

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tachyons: HashMap<usize, usize> = HashMap::new();
    let mut input = input.iter();
    let initial_line = input.next().expect("No initial line");
    tachyons.insert(
        initial_line
            .bytes()
            .position(|byte| byte == b'S')
            .expect("Input faulty"),
        1,
    );

    for row in input {
        let splitters = row.bytes().positions(|byte| byte == b'^');
        for splitter in splitters {
            let nr_of_timelines = match tachyons.remove(&splitter) {
                None => {
                    continue;
                }
                Some(x) => x,
            };

            tachyons.entry(splitter - 1).and_modify(|x| *x += nr_of_timelines).or_insert(nr_of_timelines);
            tachyons.entry(splitter + 1).and_modify(|x| *x += nr_of_timelines).or_insert(nr_of_timelines);
        }
    }

    let result: usize = tachyons.values().sum();

    println!("Tachyons: {:?}", result);

    Ok(())
}
