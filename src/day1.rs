use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Day1Data {
    total_difference: i32,
    total_difference_part_2: i32,
}

fn read_data() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("assets/day1_input.txt")?;
    Ok(content)
}

fn calculate_stuff(data: &String) -> Day1Data {
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();
    let mut right_side_hash: HashMap<i32, i32> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let part1 = parts[0].parse::<i32>().unwrap();
            let part2 = parts[1].parse::<i32>().unwrap();

            left_side.push(part1);

            right_side.push(part2);
            let right_side_data = right_side_hash.entry(part2).or_insert(0);
            *right_side_data += 1;

            println!("Part 1: {}, Part 2: {} - Multiplier: {}", part1, part2, right_side_data);
        } else {
            eprintln!("Line does not contain exactly two parts: {}", line);
        }
    }

    left_side.sort_unstable();
    right_side.sort_unstable();

    let mut total_difference = 0;
    let mut total_difference_multiplier = 0;

    for i in 0..left_side.len() {
        let num1 = left_side[i];
        let num2 = right_side[i];

        let difference = (num1 - num2).abs();
        total_difference += difference;

        let num1_multiplier = right_side_hash.get(&num1).unwrap_or(&0);
        let difference_multiplier = (num1 * num1_multiplier).abs();
        total_difference_multiplier += difference_multiplier;
    }

    Day1Data {
        total_difference,
        total_difference_part_2: total_difference_multiplier,
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
3   3"
            .to_string();

        let result = calculate_stuff(&test_data);
        println!("Result: {:?}", result);
    }

    #[test]
    fn main_data() {
        let test_data = read_data().unwrap();
        let result = calculate_stuff(&test_data);
        println!("Result: {:?}", result);
    }
}
