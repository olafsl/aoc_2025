use itertools::Itertools;

type Idxf = (f64, f64);
type Idx = (isize, isize);

pub fn checker(idx: &Idxf, edge: &Vec<Idxf>) -> bool {
    let count = edge
        .iter()
        .tuple_windows()
        .filter(|(a, _)| idx.0 >= a.0)
        .map(|(a, b)| {
            if a.1 < b.1 {
                (a.1 + 0.5, b.1 - 0.5)
            } else {
                (b.1 + 0.5, a.1 - 0.5)
            }
        })
        .filter(|(a, b)| *a < idx.1 && idx.1 < *b)
        .count();

    count % 2 == 0
}

pub fn valid_combination(a: &Idx, b: &Idx, edges: &Vec<Idxf>) -> bool {
    let x = if a.0 > b.0 { b.0..=a.0 } else { a.0..=b.0 };
    let y = if a.1 > b.1 { b.1..=a.1 } else { a.1..=b.1 };
    let check = x
        .cartesian_product(y)
        .all(|(i, j)| checker(&(i as f64, j as f64), edges));
    println!("({:?}, {:?}), {:?} {:?}", a, b, area(a, b), check);
    check
}

pub fn area(a: &Idx, b: &Idx) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

pub fn main(input: Vec<String>) {
    let tiles = input.iter().map(|line| {
        let (a, b) = line.split_once(",").expect("Input malformed");
        (
            a.parse::<isize>().expect("Not Parsable"),
            b.parse::<isize>().expect("Not Parsable"),
        )
    });
    let edges: Vec<Idxf> = tiles
        .clone()
        .map(|(a, b)| (a as f64, b as f64))
        .collect_vec();

    let a = tiles
        .tuple_combinations()
        //.map(|(a, b)| order_rectangle(a, b))
        .sorted_by(|(a, b), (c, d)| area(&a, &b).cmp(&area(&c, &d)).reverse())
        .filter(|(a, b)| valid_combination(a, b, &edges))
        .next()
        .unwrap();
    //.collect_vec();
    println!("{:?},{:?} -> {:?}", a.0, a.1, area(&a.0, &a.1));
}
