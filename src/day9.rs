use itertools::Itertools;

type Id = (u32, u32);

const MAX: usize = 100_000;

#[inline(always)]
fn idx(row: u32, col: u32) -> usize {
    (row as usize) * MAX + (col as usize)
}

pub fn create_grid(edges: &Vec<Id>) -> Vec<bool> {
    let mut grid = vec![false; MAX * MAX];

    println!("Created grid");
    let windows = edges.iter().tuple_windows().collect_vec();

    for (a, b) in windows {
        if a.0 == b.0 {
            for i in a.1.min(b.1)..=a.1.max(b.1) {
                grid[idx(a.0, i)] = true;
            }
        } else {
            for i in a.0.min(b.0)..=a.0.max(b.0) {
                grid[idx(i, a.1)] = true;
            }
        }
    }
    println!("Filled in corners");
    let last = &edges[edges.len() - 1];
    let first = &edges[0];
    if last.0 == first.0 {
        // Same row: fill columns between them
        for i in last.1.min(first.1)..=last.1.max(first.1) {
            grid[idx(last.0, i)] = true;
        }
    } else {
        // Same column: fill rows between them
        for i in last.0.min(first.0)..=last.0.max(first.0) {
            grid[idx(i, last.1)] = true;
        }
    }
    println!("Filled in edges");

    let mut previous_inserts: Vec<(u32, u32)> = Vec::new();
    let mut inserts: Vec<(u32, u32)> = Vec::new();
    for i in 0..(MAX as u32) {
        let mut inside = false;
        let mut from_up = false;
        let mut from_down = false;
        for j in 0..(MAX as u32) {
            let val = grid[idx(i, j)];
            if val {
                let up = i.checked_sub(1).is_some_and(|i| grid[idx(i, j)]);
                let down = i.checked_add(1).is_some_and(|i| grid[idx(i, j)]);
                if (from_up && up) || (from_down && down) {
                    from_up = false;
                    from_down = false;
                    continue;
                }
                if (from_up && down) || (from_down && up) {
                    inside = !inside;
                    from_up = false;
                    from_down = false;
                    continue;
                }
                if up && down {
                    inside = !inside;
                    continue;
                }
                if up {
                    from_up = true;
                    continue;
                }
                if down {
                    from_down = true;
                    continue;
                }
            }
            if inside {
                inserts.push((i, j));
            }
        }
        for (row, col) in &previous_inserts {
            grid[idx(*row, *col)] = true;
        }
        previous_inserts = std::mem::take(&mut inserts);
        if i % 1000 == 0 {
            println!("Filled in row {}", i);
        }
    }

    grid
}

pub fn area(a: &Id, b: &Id) -> usize {
    ((a.0 as usize).abs_diff(b.0 as usize) + 1) * ((a.1 as usize).abs_diff(b.1 as usize) + 1)
}

pub fn check_edge(a: &Id, b: &Id, grid: &[bool]) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);
    (min_x..=max_x)
        .map(|x| (x, min_y))
        .chain((min_x..=max_x).map(|x| (x, max_y)))
        .chain(
            (min_y..=max_y)
                .map(|y| (min_x, y))
                .chain((min_y..=max_y).map(|y| (max_x, y))),
        )
        .sorted()
        .dedup()
        .any(|(x, y)| !grid[idx(x, y)])
}

pub fn main(input: Vec<String>) {
    let tiles = input.iter().map(|line| {
        let (a, b) = line.split_once(",").expect("Input malformed");
        (
            a.parse::<u32>().expect("Not Parsable"),
            b.parse::<u32>().expect("Not Parsable"),
        )
    });
    let edges: Vec<Id> = tiles.clone().collect_vec();
    let grid = create_grid(&edges);

    let a = tiles
        .tuple_combinations()
        .sorted_by(|(a, b), (c, d)| area(&a, &b).cmp(&area(&c, &d)).reverse())
        .filter(|(a, b)| check_edge(a, b, &grid))
        .next()
        .unwrap();
    println!("{:?},{:?} -> {:?}", a.0, a.1, area(&a.0, &a.1));
}
