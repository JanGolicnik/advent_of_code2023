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

    let mut ways_to_beat = 1;

    let mut n_winnable = 0;

    for hold_length in 0..time + 1 {
        let traveled = hold_length * (time - hold_length);
        if traveled > distance {
            n_winnable += 1;
        }
    }

    ways_to_beat *= n_winnable;

    ways_to_beat.to_string()
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
