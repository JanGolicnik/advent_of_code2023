use std::collections::HashSet;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for mut line in input.lines() {
        let card_num_index = line.find(':').unwrap();
        line = &line[card_num_index + 1..];
        let mut winning_numbers: HashSet<u8> = HashSet::new();
        winning_numbers.reserve(5);

        let winning_numbers_index = line.find('|').unwrap();
        let winning_numbers_section = &line[..winning_numbers_index - 1];
        let mut i = 0;
        while i < winning_numbers_section.len() {
            winning_numbers.insert(line[i..i + 3].to_string().trim().parse::<u8>().unwrap());
            i += 3;
        }

        let our_numbers_section = &line[winning_numbers_index + 1..];
        let mut i = 0;
        let mut value = 0;
        while i < our_numbers_section.len() {
            if winning_numbers.contains(
                &our_numbers_section[i..i + 3]
                    .to_string()
                    .trim()
                    .parse::<u8>()
                    .unwrap(),
            ) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
            i += 3;
        }

        sum += value;
    }

    sum.to_string()
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
