use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy, Debug)]
struct Range {
    start: u128,
    end: u128,
}

impl Range {
    fn new(start: &str, end: &str) -> Range {
        Range {
            start: start.parse::<u128>().unwrap(),
            end: end.parse::<u128>().unwrap(),
        }
    }

    fn remove_existing(&mut self, other: Self) -> Result<(), Box<dyn std::error::Error>> {
        if (self.start >= other.start && self.end <= other.end)
            || (self.start < other.end && self.start == self.end)
        {
            return Err("Nothing left".into());
        }

        if self.start <= other.end && self.end > other.end {
            self.start = other.end + 1
        }

        Ok(())
    }

    fn length(self) -> u128 {
        self.end - self.start + 1
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end).reverse(),
            x => x,
        }
    }
}

pub fn main(input: Vec<String>) {
    let mut input = input.clone();
    let empty_line = input.iter().position(|x| x.len() == 0).unwrap();
    input.truncate(empty_line);

    let mut fresh_ingredient_ranges: Vec<Range> = input
        .iter()
        .map(|x: &String| x.split_once("-").unwrap())
        .map(|(a, z)| Range::new(a, z))
        .collect();

    fresh_ingredient_ranges.sort_by(|a, b| a.cmp(b));

    let mut completed_ranges: Vec<Range> = Vec::new();
    let mut sum = 0;
    'outer: for range in fresh_ingredient_ranges.iter_mut() {
        for completed in completed_ranges.iter() {
            match range.remove_existing(*completed) {
                Err(_) => {
                    continue 'outer;
                }
                Ok(()) => (),
            }
        }

        sum += range.length().clone();
        completed_ranges.push(range.clone());
    }

    println!("Result B: {:?}", sum);
}
