#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn new(line: &str) -> Self {
        let magnitude = line[1..].parse::<i32>().unwrap();
        if line.chars().next().unwrap() == 'L' {
            Self::Left(magnitude)
        } else {
            Self::Right(magnitude)
        }
    }

    fn apply(&self, val: i32) -> i32 {
        match self {
            Self::Left(magnitude) => val - magnitude,
            Self::Right(magnitude) => val + magnitude,
        }
    }
}

pub fn div_rem(value: i32) -> (i32, i32) {
    (value.div_euclid(100).abs(), value.rem_euclid(100))
}

pub fn calculate(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut value = 50;
    let mut passes = 0;
    let mut nr_of_zeroes = 0;

    for line in input.iter() {
        let rotation = Rotation::new(line);
        let (div, rem) = div_rem(rotation.apply(value));
        passes += div;

        if rem == 0 {
            nr_of_zeroes += 1;
        }

        if matches!(rotation, Rotation::Left(_)) {
            if value == 0 {
                passes -= 1;
            }
            if rem == 0 {
                passes += 1;
            }
        }

        value = rem;
    }

    println!("Passes: {}, Zeroes: {}", passes, nr_of_zeroes);
    return Ok(());
}
