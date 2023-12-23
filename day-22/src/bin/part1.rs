use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut bricks: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('~').collect();
        let first_coords: Vec<usize> = parts[0]
            .split(',')
            .map(|e| e.to_string().parse::<usize>().unwrap())
            .collect();
        let second_coords: Vec<usize> = parts[1]
            .split(',')
            .map(|e| e.to_string().parse::<usize>().unwrap())
            .collect();

        let mut positions: Vec<(usize, usize, usize)> = Vec::new();
        for x in first_coords[0]..second_coords[0] + 1 {
            for y in first_coords[1]..second_coords[1] + 1 {
                for z in first_coords[2]..second_coords[2] + 1 {
                    positions.push((x, y, z));
                }
            }
        }

        let brick_index = bricks.len() + 1;
        bricks.insert(brick_index, Vec::new());
        let brick = bricks.get_mut(&brick_index).unwrap();
        for (x, y, z) in positions.iter() {
            let x = *x;
            let y = *y;
            let z = *z;
            if map.len() <= z {
                map.resize(z + 1, Vec::new());
            }
            if map[z].len() <= y {
                map[z].resize(y + 1, Vec::new());
            }
            if map[z][y].len() <= x {
                map[z][y].resize(x + 1, 0);
            }
            map[z][y][x] = brick_index;
        }
        brick.append(&mut positions);
    }

    //expand layers
    let mut biggest: (usize, usize) = (0, 0);
    for row in map.iter() {
        for row in row.iter() {
            if row.len() > biggest.0 {
                biggest.0 = row.len();
            }
        }
        if row.len() > biggest.1 {
            biggest.1 = row.len();
        }
    }
    for row in map.iter_mut() {
        row.resize(biggest.1, Vec::new());
        for row in row.iter_mut() {
            row.resize(biggest.0, 0);
        }
    }

    //settling bricks
    let mut brick = 1;
    while let Some(coords) = bricks.get_mut(&brick) {
        let mut max_move_down = None;
        for (x, y, z) in coords.iter() {
            let mut move_down = 1;
            while *z - move_down > 0 {
                if map[*z - move_down][*y][*x] == 0 || map[*z - move_down][*y][*x] == brick {
                    move_down += 1;
                    if max_move_down.is_some() && move_down > max_move_down.unwrap() {
                        break;
                    }
                } else {
                    max_move_down = Some(move_down - 1);
                    break;
                }
            }
        }
        if let Some(move_down) = max_move_down {
            for (x, y, z) in coords.iter_mut() {
                map[*z][*y][*x] = 0;
            }
            for (x, y, z) in coords.iter_mut() {
                map[*z - move_down][*y][*x] = brick;
                *z -= move_down;
            }
        }
        brick += 1;
    }

    let mut safe: HashSet<usize> = HashSet::new();
    let mut not_safe: HashSet<usize> = HashSet::new();

    for (brick_index, coords) in bricks {
        let mut supporting: HashSet<usize> = HashSet::new();
        let mut supported_by: HashSet<usize> = HashSet::new();
        for (x, y, z) in coords {
            if map[z + 1][y][x] != 0 && map[z + 1][y][x] != brick_index {
                supporting.insert(map[z + 1][y][x]);
            }
            if map[z - 1][y][x] != 0 && map[z - 1][y][x] != brick_index {
                supported_by.insert(map[z - 1][y][x]);
            }
        }
        if supporting.is_empty() {
            safe.insert(brick_index);
        }
        if supported_by.len() > 1 {
            for v in supported_by {
                safe.insert(v);
            }
        } else {
            for v in supported_by {
                not_safe.insert(v);
            }
        }
    }
    for not_safe in not_safe {
        safe.remove(&not_safe);
    }

    safe.len().to_string()
}
