use std::collections::HashSet;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn remove_invalid_symmetries(row: &Vec<bool>, symmetries: &mut HashSet<usize>) {
    let mut for_removal: Vec<usize> = Vec::new();
    'outer: for symmetry in symmetries.iter() {
        let mut left = *symmetry - 1;
        let mut right = *symmetry;
        while row[left] == row[right] {
            if left == 0 || right == row.len() - 1 {
                continue 'outer;
            }
            left -= 1;
            right += 1;
        }
        for_removal.push(*symmetry);
    }

    for v in for_removal {
        symmetries.remove(&v);
    }
}

type Pattern = Vec<Vec<bool>>;
fn eval_patterns(patterns: &mut Vec<Pattern>) -> usize {
    let mut sum = 0;

    let mut i = 0;
    'outer: while i < patterns.len() {
        let pattern = patterns.get(i).unwrap();
        let mut symmetries: HashSet<usize> = HashSet::new();

        let row = pattern.first().unwrap();
        for (left_index, left) in row.iter().enumerate() {
            for (right_index, right) in row.iter().enumerate().rev() {
                let mut left = *left;
                let mut right = *right;
                let mut left_index = left_index;
                let mut right_index = right_index;
                while left == right && left_index < right_index {
                    if left_index + 1 == right_index {
                        symmetries.insert(right_index);
                        break;
                    }
                    left_index += 1;
                    right_index -= 1;
                    left = row[left_index];
                    right = row[right_index];
                }
            }
        }

        for row in pattern.iter().skip(1) {
            remove_invalid_symmetries(row, &mut symmetries);
            if symmetries.is_empty() {
                i += 1;
                continue 'outer;
            }
        }

        sum += *symmetries
            .iter()
            .take(1)
            .collect::<Vec<&usize>>()
            .first()
            .unwrap();

        patterns.remove(i);
    }

    sum
}

fn rotate_patterns(patterns: &mut Vec<Pattern>) {
    let mut new_patterns: Vec<Pattern> = Vec::new();

    for pattern in patterns.iter() {
        let mut new_pattern: Pattern = vec![Vec::new(); pattern.first().unwrap().len()];
        for row in pattern {
            for (i, v) in row.iter().rev().enumerate() {
                new_pattern[i].push(*v);
            }
        }
        for row in new_pattern.iter() {
            println!("{:?}", row);
        }

        new_patterns.push(new_pattern);
    }

    patterns.clear();
    patterns.append(&mut new_patterns);
}

fn part1(input: &str) -> String {
    let mut patterns: Vec<Pattern> = vec![Vec::new()];

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Vec::new());
            continue;
        }
        let row: Vec<bool> = line.chars().map(|e| e == '#').collect();
        patterns.last_mut().unwrap().push(row);
    }

    let mut sum: usize = eval_patterns(&mut patterns);
    rotate_patterns(&mut patterns);
    sum += eval_patterns(&mut patterns) * 100;
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
