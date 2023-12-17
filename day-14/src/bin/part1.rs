fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn rotate_map(map: &mut Vec<Vec<Tile>>) {
    let mut new_map: Vec<Vec<Tile>> = vec![Vec::new(); map.first().unwrap().len()];
    for row in map.iter() {
        for (i, v) in row.iter().rev().enumerate() {
            new_map[i].push(*v);
        }
    }
    map.clear();
    map.append(&mut new_map);
}

type Tile = Option<bool>; //true == #
fn part1(input: &str) -> String {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in input.lines() {
        map.push(
            line.chars()
                .map(|e| match e {
                    '.' => None,
                    'O' => Some(false),
                    '#' => Some(true),
                    _ => panic!(),
                })
                .collect::<Vec<Tile>>(),
        )
    }

    rotate_map(&mut map);

    for row in map.iter() {
        for c in row {
            print!(
                "{}",
                match c {
                    None => '.',
                    Some(true) => '#',
                    _ => 'O',
                }
            );
        }
        println!();
    }
    println!();

    for row in map.iter_mut() {
        let mut index = 0;
        while index < row.len() {
            let val = row[index];
            if !val.unwrap_or(true) {
                row[index] = None;
                loop {
                    if index == 0 || (index > 0 && row[index - 1].is_some()) {
                        break;
                    }
                    index -= 1;
                }
                row[index] = Some(false);
            }

            index += 1;
        }
    }

    for row in map.iter() {
        for c in row {
            print!(
                "{}",
                match c {
                    None => '.',
                    Some(true) => '#',
                    _ => 'O',
                }
            );
        }
        println!()
    }

    let mut load = 0;
    for row in map.into_iter() {
        for (index, val) in row.into_iter().rev().enumerate() {
            if let Some(false) = val {
                load += index + 1;
            }
        }
    }

    load.to_string()
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
