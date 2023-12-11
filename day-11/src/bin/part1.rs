fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut row_len = 0;

    for line in input.lines() {
        let row: Vec<bool> = line.chars().map(|e| e != '.').collect();
        row_len = row.len();
        map.push(row);
    }

    let mut y = 0;
    'outer: while y < map.len() {
        for val in map[y].iter() {
            if *val {
                y += 1;
                continue 'outer;
            }
        }
        map.insert(y, vec![false; row_len]);
        y += 2;
    }

    let mut x = 0;
    'outer: while x < map[0].len() {
        for row in map.iter() {
            if row[x] {
                x += 1;
                continue 'outer;
            }
        }
        for row in map.iter_mut() {
            row.insert(x, false);
        }
        x += 2;
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, v)| **v) {
            galaxies.push((x, y));
        }
    }

    let mut sum = 0;

    while !galaxies.is_empty() {
        let current = *galaxies.last().unwrap();
        for other in galaxies.iter() {
            sum += (current.1 as i32 - other.1 as i32).unsigned_abs()
                + (current.0 as i32 - other.0 as i32).unsigned_abs();
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
