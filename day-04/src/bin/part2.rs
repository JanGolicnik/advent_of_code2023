use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut card_count: HashMap<usize, u32> = HashMap::new();
    for (line_index, mut line) in input.lines().enumerate() {
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
        let mut num_winning_numbers = 0;
        while i < our_numbers_section.len() {
            if winning_numbers.contains(
                &our_numbers_section[i..i + 3]
                    .to_string()
                    .trim()
                    .parse::<u8>()
                    .unwrap(),
            ) {
                num_winning_numbers += 1;
            }
            i += 3;
        }

        let card_n = line_index + 1;
        if let std::collections::hash_map::Entry::Vacant(e) = card_count.entry(card_n) {
            e.insert(1);
        } else {
            *card_count.get_mut(&card_n).unwrap() += 1;
        }
        let n_this_card = *card_count.get(&card_n).unwrap();

        for i in 1..num_winning_numbers + 1 {
            let index = i + card_n;
            if let std::collections::hash_map::Entry::Vacant(e) = card_count.entry(index) {
                e.insert(n_this_card);
            } else {
                *card_count.get_mut(&index).unwrap() += n_this_card;
            }
        }
    }

    let mut sum = 0;

    for (k, value) in card_count.iter() {
        println!("{k}: {value}");
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
