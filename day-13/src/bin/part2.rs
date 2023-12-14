use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn count_invalid_symmetries(row: &Vec<bool>, symmetries: &mut HashMap<usize, bool>) {
    let mut for_removal: Vec<usize> = Vec::new();
    'outer: for (symmetry, count) in symmetries.iter_mut() {
        let mut left = *symmetry - 1;
        let mut right = *symmetry;
        while row[left] == row[right] {
            if left == 0 || right == row.len() - 1 {
                continue 'outer;
            }
            left -= 1;
            right += 1;
        }
        if *count{
            for_removal.push(*symmetry);
        } else {
            *count = true;
        }
    }
    for v in for_removal {
        symmetries.remove(&v);
    }
}

type Pattern = Vec<Vec<bool>>;
fn eval_patterns(patterns: &mut Vec<Pattern>) -> usize {
    let mut sum = 0;

    let mut i = 0;
    'outer: while i < patterns.len(){
        let pattern = patterns.get(i).unwrap();
        let mut symmetries: HashMap<usize, bool> = HashMap::new();

        let row = pattern.first().unwrap();
        for (left_index, left) in row.iter().enumerate() {
            for (right_index, right) in row.iter().enumerate().rev() {
                let mut left = *left;
                let mut right = *right;
                let mut left_index = left_index;
                let mut right_index = right_index;
                while left == right && left_index < right_index {
                    if left_index + 1 == right_index {
                        symmetries.insert(right_index, false);
                        break;
                    }
                    left_index += 1;
                    right_index -= 1;
                    left = row[left_index];
                    right = row[right_index];
                }
            }
        }


        if symmetries.is_empty() {
            i += 1;
            continue 'outer;
        }

        for row in pattern.iter() {
            count_invalid_symmetries(row, &mut symmetries);
        }

        if symmetries.iter().filter(|(_,v)| **v).count() == 1 {
            sum += symmetries
                .iter()
                .filter(|(_,v)| **v)
                .take(1)
                .collect::<Vec<(&usize, &bool)>>()
                .first()
                .unwrap().0;

            patterns.remove(i);

            continue;
        }

        i += 1;
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
