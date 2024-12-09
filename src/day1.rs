use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Day1Data {
    difference_total: i32,
    multiplication_total: i32,
}

fn read_data(asset_path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(asset_path)?;
    Ok(content)
}

fn calculate_day1_data(data: &str) -> Day1Data {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    let mut right_numbers_freq: HashMap<i32, i32> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let part1 = parts[0]
            .parse::<i32>()
            .expect("Failed to parse part1 as i32");
        let part2 = parts[1]
            .parse::<i32>()
            .expect("Failed to parse part2 as i32");

        left_numbers.push(part1);
        right_numbers.push(part2);

        *right_numbers_freq.entry(part2).or_insert(0) += 1;
    }

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let mut total_difference = 0;
    let mut total_difference_multiplier = 0;

    for (num1, num2) in left_numbers.iter().zip(right_numbers.iter()) {
        total_difference += (num1 - num2).abs();

        let frequency = right_numbers_freq.get(num1).unwrap_or(&0);
        total_difference_multiplier += (num1 * frequency).abs();
    }

    Day1Data {
        difference_total: total_difference,
        multiplication_total: total_difference_multiplier,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let test_data = "
3   4
4   3
2   5
1   3
3   9
3   3";

        let result = calculate_day1_data(&test_data);
        assert_eq!(result.difference_total, 11);
        assert_eq!(result.multiplication_total, 31);
    }

    #[test]
    fn main_data() {
        let test_data = read_data("assets/day1_input.txt").expect("Could not find test file.");
        let result = calculate_day1_data(&test_data);
        assert_eq!(result.difference_total, 1882714);
        assert_eq!(result.multiplication_total, 19437052);
    }
}
