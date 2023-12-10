use std::{collections::btree_set::Difference, fs::DirBuilder};

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn move_across_map(val: char, prev_pos: (usize, usize), pos: &mut (usize, usize)) {
    match val {
        '|' => {
            if prev_pos.1 < pos.1 {
                pos.1 += 1
            } else {
                pos.1 -= 1
            }
        }
        '-' => {
            if prev_pos.0 < pos.0 {
                pos.0 += 1
            } else {
                pos.0 -= 1
            }
        }
        'F' => {
            if prev_pos.1 > pos.1 {
                pos.0 += 1
            } else {
                pos.1 += 1
            }
        }
        '7' => {
            if prev_pos.1 > pos.1 {
                pos.0 -= 1
            } else {
                pos.1 += 1
            }
        }
        'J' => {
            if prev_pos.1 < pos.1 {
                pos.0 -= 1
            } else {
                pos.1 -= 1
            }
        }
        'L' => {
            if prev_pos.1 < pos.1 {
                pos.0 += 1
            } else {
                pos.1 -= 1
            }
        }
        _ => todo!(),
    }
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    map.reserve(input.lines().count());

    let mut current_coords: Option<(usize, usize)> = None;

    for (index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if current_coords.is_none() {
            if let Some(starting_pos) = chars.iter().position(|e| *e == 'S') {
                current_coords = Some((starting_pos, index));
            }
        }
        map.push(chars);
    }

    let mut pos = current_coords.unwrap();
    let mut prev_pos = pos;
    if pos.0 > 0 && matches!(map[pos.1][pos.0 - 1], '-' | 'L' | 'F') {
        pos.0 -= 1;
    } else if matches!(map[pos.1 + 1][pos.0], '|' | 'L' | 'J') {
        pos.1 += 1;
    } else if matches!(map[pos.1][pos.0 + 1], '-' | '7' | 'J') {
        pos.0 += 1;
    } else if pos.1 > 0 && matches!(map[pos.1 - 1][pos.0], '|' | '7' | 'F') {
        pos.1 -= 1;
    }
    let mut len = 1;

    while map[pos.1][pos.0] != 'S' {
        let current = map[pos.1][pos.0];
        let tmp = pos;
        move_across_map(current, prev_pos, &mut pos);
        prev_pos = tmp;
        len += 1;
    }

    (len / 2).to_string()
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
