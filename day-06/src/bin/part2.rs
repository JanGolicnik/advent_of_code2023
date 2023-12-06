fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let time_line: &str = &lines[0][11..];
    let time: u64 = time_line
        .to_string()
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance_line: &str = &lines[1][11..];
    let distance: u64 = distance_line
        .to_string()
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let mut first_winnable = 0;
    let mut last_winnable = u64::MAX;

    for hold_length in 0..time + 1 {
        let traveled = hold_length * (time - hold_length);
        if traveled > distance {
            first_winnable = hold_length;
            break;
        }
    }

    for hold_length in (0..time + 1).rev() {
        let traveled = hold_length * (time - hold_length);
        if traveled > distance {
            last_winnable = hold_length;
            break;
        }
    }

    (last_winnable - first_winnable + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let result = part1("");
        assert_eq!(result, "".to_string());
    }
}
