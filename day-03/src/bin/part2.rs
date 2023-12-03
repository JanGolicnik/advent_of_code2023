use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut number_indices: HashMap<(usize, usize), (u32, (usize, usize))> = HashMap::new();

    for (line_index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let mut c = chars[i];
            if c.is_numeric() {
                let mut number: String = String::new();
                let number_start_index = i;
                while c.is_numeric() {
                    number.push(c);
                    i += 1;
                    if i == chars.len() {
                        break;
                    }
                    c = chars[i];
                }
                for char_index in number_start_index..i {
                    number_indices.insert(
                        (char_index, line_index),
                        (number.parse::<u32>().unwrap(), (number_start_index, i)),
                    );
                }
            }
            i += 1;
        }
    }

    let mut sum = 0;

    const OFFSETS: [(i32, i32); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    for (line_index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if *c == '*' {
                let mut neighbours = 0;
                let mut gear_ratio = 1;
                for (char_offset, line_offset) in OFFSETS {
                    let key_char_index = (i as i32 + char_offset) as usize;
                    let key_line_index = (line_index as i32 + line_offset) as usize;
                    let key = (key_char_index, key_line_index);

                    if number_indices.contains_key(&key) {
                        neighbours += 1;
                        let (number, (start_index, end_index)) =
                            number_indices.remove(&key).unwrap();
                        for index in start_index..end_index {
                            number_indices.remove(&(index, key_line_index));
                        }
                        gear_ratio *= number;
                    }
                }
                if neighbours == 2 {
                    sum += gear_ratio;
                }
            }
        }
    }

    println!("{:?}", number_indices);

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
