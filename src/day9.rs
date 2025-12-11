use std::collections::HashMap;

use itertools::Itertools;

type Id = (isize, isize);

pub fn create_grid(edges: &Vec<Id>) -> HashMap<Id, bool> {
    let mut grid = HashMap::new();
    let max = 100000;
    for i in 0..max {
        for j in 0..max {
            grid.insert((i, j), false);
        }
        if i % 1000 == 0 {
            println!("Creating grid in row {}", i);
        }
    }

    println!("Created grid");
    let windows = edges.iter().tuple_windows().collect_vec();

    for (a, b) in windows {
        if a.0 == b.0 {
            for i in a.1.min(b.1)..=a.1.max(b.1) {
                grid.insert((a.0, i), true);
            }
        } else {
            for i in a.0.min(b.0)..=a.0.max(b.0) {
                grid.insert((i, a.1), true);
            }
        }
    }
    println!("Filled in corners");
    for i in edges[0].0..=edges[edges.len() - 1].1 {
        grid.insert((edges[0].0, i), true);
    }
    for i in edges[0].1..=edges[edges.len() - 1].0 {
        grid.insert((i, edges[0].1), true);
    }
    println!("Filled in edges");

    let mut fill_in = Vec::new();
    for i in 0..max {
        let mut inside = false;
        let mut from_up = false;
        let mut from_down = false;
        for j in 0..max {
            let val = grid.get(&(i, j)).unwrap();
            if *val {
                let up = grid.get(&(i-1, j)).unwrap_or(&false);
                let down = grid.get(&(i+1, j)).unwrap_or(&false);
                println!("({:?}, {:?}) -> (up: {:?}, down: {:?}, val: {:?})", i, j, up, down, val);
                if (from_up && *up) || (from_down && *down) {
                    from_up = false;
                    from_down = false;
                    continue;
                }
                if (from_up && *down) || (from_down && *up) {
                    inside = !inside;
                    from_up = false;
                    from_down = false;
                    continue;
                }
                if *up && *down {
                    inside = !inside;
                    continue;
                }
                if *up {
                    from_up = true;
                    continue;
                }
                if *down {
                    from_down = true;
                    continue;
                }


            }
            if inside {
                fill_in.push((i, j));
            }
        }
        if i % 1000 == 0 {
            println!("Filled in row {}", i);
        }
    }

    for (i, j) in fill_in {
        grid.insert((i, j), true);
    }

    grid
}

pub fn area(a: &Id, b: &Id) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

pub fn check_edge(a: &Id, b: &Id, grid: &HashMap<Id, bool>) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);
    let mut edges = (min_x..=max_x).map(|x| (x, min_y)).collect_vec();
    edges.extend((min_x..=max_x).map(|x| (x, max_y)).collect_vec());
    edges.extend((min_y..=max_y).map(|y| (min_x, y)).collect_vec());
    edges.extend((min_y..=max_y).map(|y| (max_x, y)).collect_vec());
    let edges = edges.iter().sorted().dedup().collect_vec();
    // println!("{:?}", edges);
    edges.iter().all(|(x, y)| *grid.get(&(*x, *y)).unwrap())
}

pub fn main(input: Vec<String>) {
    let tiles = input.iter().map(|line| {
        let (a, b) = line.split_once(",").expect("Input malformed");
        (
            a.parse::<isize>().expect("Not Parsable"),
            b.parse::<isize>().expect("Not Parsable"),
        )
    });
    let edges: Vec<Id> = tiles.clone().collect_vec();
    let grid = create_grid(&edges);

    // println!("{:?}", grid);

    let a = tiles
        .tuple_combinations()
        .sorted_by(|(a, b), (c, d)| area(&a, &b).cmp(&area(&c, &d)).reverse())
        .filter(|(a, b)| {
            // println!("{:?}, {:?}", a, b);
            let bla = check_edge(a, b, &grid);
            // println!("{:?}", bla);
            bla
        })
        .next()
        .unwrap();
    // println!("{:?},{:?} -> {:?}", a.0, a.1, area(&a.0, &a.1));
}
