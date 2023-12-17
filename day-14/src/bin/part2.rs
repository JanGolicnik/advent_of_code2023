use std::mem::swap;

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

type Tile = Option<bool>; //true == #
type Map = Vec<Vec<Tile>>;

fn rotate_into(map: &mut [Vec<Tile>], new_map: &mut [Vec<Tile>]) {
    for (row_index, row) in map.iter().rev().enumerate() {
        for (value_index, v) in row.iter().enumerate() {
            new_map[value_index][row_index] = *v;
        }
    }
}

fn settle_map(map: &mut [Vec<Tile>]) {
    for row in map.iter_mut() {
        let mut i = row.len() - 1;
        loop {
            if let Some(false) = row[i] {
                row[i] = None;
                let mut inner_i = i;
                loop {
                    if inner_i == row.len() - 1
                        || (inner_i < row.len() && row[inner_i + 1].is_some())
                    {
                        break;
                    }
                    inner_i += 1;
                }
                row[inner_i] = Some(false);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}

fn are_maps_the_same(left: &[Vec<Tile>], right: &[Vec<Tile>]) -> bool {
    for (row_index, left_row) in left.iter().enumerate() {
        for (val_index, left_val) in left_row.iter().enumerate() {
            if *left_val != right[row_index][val_index] {
                return false;
            }
        }
    }
    true
}

fn part1(input: &str) -> String {
    let mut map: Map = Vec::new();
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

    let mut sideways_map: Map = vec![vec![None; map.len()]; map.first().unwrap().len()];
    let mut current_map: &mut Map = &mut map;
    let mut other_map: &mut Map = &mut sideways_map;

    let mut cache: Vec<Map> = Vec::new();

    const N_CYCLES: usize = 1_000_000_000;

    'outer: for i in 0..N_CYCLES {
        for _ in 0..4 {
            rotate_into(&mut current_map[..], &mut other_map[..]);
            swap(&mut current_map, &mut other_map);
            settle_map(current_map);
        }

        if let Some((index, _)) = cache
            .iter()
            .enumerate()
            .find(|(_, e)| are_maps_the_same(e, current_map))
        {
            let period = i - index;
            let remaining_cycles = N_CYCLES - i;
            println!("{i}: {period}");
            for _ in 0..remaining_cycles % period {
                for _ in 0..4 {
                    rotate_into(&mut current_map[..], &mut other_map[..]);
                    swap(&mut current_map, &mut other_map);
                    settle_map(current_map);
                }
            }
            break 'outer;
        } else {
            cache.push(current_map.clone());
        }
    }

    let mut load = 0;
    for row in current_map.iter() {
        for (index, val) in row.iter().rev().enumerate() {
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
