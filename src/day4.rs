use itertools::izip;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Paper,
    Emptied,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Paper => write!(f, "@"),
            Cell::Emptied => write!(f, "x"),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '@' => Cell::Paper,
            '.' => Cell::Empty,
            _ => panic!("Invalid cell character: {}", c),
        }
    }
}

pub fn process<'a>(center: &'a mut Cell, neighbours: impl Iterator<Item = &'a Cell>) {
    let paper_count = neighbours.filter(|x| matches!(x, Cell::Paper)).count();
    if matches!(*center, Cell::Paper) && paper_count <= 4 {
        *center = Cell::Emptied;
    }
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let input_height = input.len();
    let input_width = input.first().map(|s| s.len()).unwrap_or(0);

    let data: Vec<Cell> = input
        .iter()
        .flat_map(|line| line.chars())
        .map(Cell::from)
        .collect();

    let grid_width = input_width + 2;
    let grid_height = input_height + 2;

    let mut grid = vec![Cell::Empty; grid_width * grid_height];

    for (row_idx, line) in data.chunks(input_width).enumerate() {
        let start_idx = (row_idx + 1) * grid_width + 1;
        grid[start_idx..start_idx + input_width].copy_from_slice(line);
    }

    let mut nr_of_emptied = 0;

    let mut nr_of_cycles = 0;
    loop {
        nr_of_cycles += 1;

        let cloned_grid = grid.clone();
        let row_1 = cloned_grid.windows(3);
        let row_2 = cloned_grid.windows(3).skip(grid_width);
        let row_3 = cloned_grid.windows(3).skip(grid_width * 2);

        let center = grid.iter_mut().skip(grid_width + 1);

        let neighbour_windows =
            izip!(row_1, row_2, row_3).map(|(x, y, z): (&[Cell], &[Cell], &[Cell])| {
                x.into_iter().chain(y.into_iter()).chain(z.into_iter())
            });
        for (center, neighbors) in center.zip(neighbour_windows) {
            process(center, neighbors);
        }

        let new_count = grid.iter().filter(|x| matches!(x, Cell::Emptied)).count();
        if new_count == nr_of_emptied {
            break;
        }
        nr_of_emptied = new_count;
    }

    println!("Result: {:?}, in {:?} cycles", nr_of_emptied, nr_of_cycles);
    Ok(())
}
