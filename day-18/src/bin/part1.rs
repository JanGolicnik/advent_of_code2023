fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Tile {
    RealUndug,
    Undug,
    Dug,
}
fn flood_fill(pos: (usize, usize), map: &mut Vec<Vec<Tile>>) {
    map[pos.1][pos.0] = Tile::RealUndug;

    if pos.0 > 0 && map[pos.1][pos.0 - 1] == Tile::Undug {
        flood_fill((pos.0 - 1, pos.1), map);
    }
    if pos.0 < map.first().unwrap().len() - 1 && map[pos.1][pos.0 + 1] == Tile::Undug {
        flood_fill((pos.0 + 1, pos.1), map);
    }
    if pos.1 > 0 && map[pos.1 - 1][pos.0] == Tile::Undug {
        flood_fill((pos.0, pos.1 - 1), map);
    }
    if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] == Tile::Undug {
        flood_fill((pos.0, pos.1 + 1), map);
    }
}
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
fn part1(input: &str) -> String {
    let mut plan: Vec<(Dir, u32, u32)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let dir = match parts[0] {
            "R" => Dir::Right,
            "D" => Dir::Down,
            "U" => Dir::Up,
            "L" => Dir::Left,
            _ => panic!(),
        };
        plan.push((dir, parts[1].to_string().parse::<u32>().unwrap(), 0));
    }

    //find size of map
    let mut pos: (i32, i32) = (0, 0);
    let mut mapsize: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
    for (dir, amount, _) in plan.iter() {
        match dir {
            Dir::Up => {
                pos.1 -= *amount as i32;
                mapsize.0 .1 = mapsize.0 .1.min(pos.1);
            }
            Dir::Down => {
                pos.1 += *amount as i32;
                mapsize.1 .1 = mapsize.1 .1.max(pos.1);
            }
            Dir::Left => {
                pos.0 -= *amount as i32;
                mapsize.0 .0 = mapsize.0 .0.min(pos.0);
            }
            Dir::Right => {
                pos.0 += *amount as i32;
                mapsize.1 .0 = mapsize.1 .0.max(pos.0);
            }
        }
    }

    let mut map: Vec<Vec<Tile>> = Vec::new();

    for (y_index, _) in (mapsize.0 .1..mapsize.1 .1 + 1).enumerate() {
        map.push(Vec::new());
        map[y_index].reserve((mapsize.1 .0 - mapsize.0 .0) as usize + 1);
        for _ in mapsize.0 .0..mapsize.1 .0 + 1 {
            map[y_index].push(Tile::Undug);
        }
    }

    let mut pos: (i32, i32) = (0, 0);
    for (dir, amount, _) in plan {
        for _ in 0..amount {
            let x_index = pos.0 - mapsize.0 .0;
            let y_index = pos.1 - mapsize.0 .1;
            map[y_index as usize][x_index as usize] = Tile::Dug;

            match dir {
                Dir::Up => {
                    pos.1 -= 1;
                }
                Dir::Down => {
                    pos.1 += 1;
                }
                Dir::Left => {
                    pos.0 -= 1;
                }
                Dir::Right => {
                    pos.0 += 1;
                }
            }
        }
    }

    for (y_index, _) in (mapsize.0 .1..mapsize.1 .1 + 1).enumerate() {
        if map[y_index][0] == Tile::Undug {
            flood_fill((0, y_index), &mut map);
        }
    }

    for (y_index, _) in (mapsize.0 .1..mapsize.1 .1 + 1).enumerate() {
        if map[y_index][map.first().unwrap().len() - 1] == Tile::Undug {
            flood_fill((map.first().unwrap().len() - 1, y_index), &mut map);
        }
    }

    for (x_index, _) in (mapsize.0 .0..mapsize.1 .0 + 1).enumerate() {
        if map[0][x_index] == Tile::Undug {
            flood_fill((x_index, 0), &mut map);
        }
    }

    for (x_index, _) in (mapsize.0 .0..mapsize.1 .0 + 1).enumerate() {
        if map[map.len() - 1][x_index] == Tile::Undug {
            flood_fill((x_index, map.len() - 1), &mut map);
        }
    }

    let mut sum = 0;
    for row in map {
        for tile in row {
            if tile != Tile::RealUndug {
                sum += 1;
            }
        }
    }

    sum.to_string()
}
