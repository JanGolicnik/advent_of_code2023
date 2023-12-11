use std::collections::HashSet;

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut row_len = usize::MAX;

    for line in input.lines() {
        let row: Vec<bool> = line.chars().map(|e| e != '.').collect();
        row_len = row_len.min(row.len());
        map.push(row);
    }

    let mut expanded_columns: HashSet<usize> = HashSet::new();
    let mut expanded_rows: HashSet<usize> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        if row.iter().any(|v| *v) {
            continue;
        }
        expanded_rows.insert(y);
    }

    'outer: for x in 0..row_len {
        for row in map.iter() {
            if row[x] {
                continue 'outer;
            }
        }
        expanded_columns.insert(x);
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, v)| **v) {
            galaxies.push((x, y));
        }
    }

    let mut sum: u64 = 0;

    while !galaxies.is_empty() {
        let current = *galaxies.last().unwrap();
        for other in galaxies.iter() {
            for x in current.0.min(other.0)..current.0.max(other.0) {
                if expanded_columns.contains(&x) {
                    sum += 1_000_000;
                } else {
                    sum += 1;
                }
            }

            for y in current.1.min(other.1)..current.1.max(other.1) {
                if expanded_rows.contains(&y) {
                    sum += 1_000_000;
                } else {
                    sum += 1;
                }
            }
        }
        galaxies.pop();
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
