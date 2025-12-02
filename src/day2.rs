pub fn filter_both(acc: (i64, i64), number: i64) -> (i64, i64) {
    let digits: Vec<u8> = number.to_string().bytes().map(|b| b - b'0').collect();
    let length = digits.len();
    let test_ranges = (1..=(length/2)).rev().filter(|&x| length % x == 0);
    for test_length in test_ranges {
        let first_chunk = &digits[0..test_length];
        if digits.chunks(test_length).all(|chunk| chunk == first_chunk) {
            match test_length == length / 2 {
                true => return (acc.0 + number, acc.1),
                false => return (acc.0, acc.1 + number),
            }
        }
    }
    acc
}

pub fn process_range(mut acc: (i64, i64), s: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let (start, end) = s.split_once("-").ok_or("Invalid input")?;
    let start = start.parse::<i64>()?;
    let end = end.parse::<i64>()?;

    (start..=end).for_each(|i| acc = filter_both(acc, i));
    Ok(acc)
}

pub fn main(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let (sum_a, sum_b) = input[0].split(",").try_fold((0i64, 0i64), |acc, s| process_range(acc, s))?;
   
    println!("Sum A: {}, Sum B: {}", sum_a, sum_b + sum_a);

    Ok(())
}
