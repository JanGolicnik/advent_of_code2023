fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn execute_step(step: usize, map: &mut Vec<Vec<Option<usize>>>) {
    let width = map.first().unwrap().len();
    let height = map.len();

    for x in 0..width {
        for y in 0..height {
            if let Some(val) = map[y][x] {
                if val != step {
                    continue;
                }

                if y > 0 && map[y - 1][x].is_some_and(|e| e < step) {
                    *map.get_mut(y - 1).unwrap().get_mut(x).unwrap() = Some(step + 1);
                }

                if y + 1 < height && map[y + 1][x].is_some_and(|e| e < step) {
                    *map.get_mut(y + 1).unwrap().get_mut(x).unwrap() = Some(step + 1);
                }

                if x > 0 && map[y][x - 1].is_some_and(|e| e < step) {
                    *map.get_mut(y).unwrap().get_mut(x - 1).unwrap() = Some(step + 1);
                }

                if x + 1 < width && map[y][x + 1].is_some_and(|e| e < step) {
                    *map.get_mut(y).unwrap().get_mut(x + 1).unwrap() = Some(step + 1);
                }
            }
        }
    }
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<Option<usize>>> = Vec::new();
    for line in input.lines() {
        map.push(
            line.chars()
                .map(|e| match e {
                    'S' => Some(1),
                    '.' => Some(0),
                    _ => None,
                })
                .collect(),
        )
    }

    const N_STEPS: usize = 64;

    for i in 0..N_STEPS {
        execute_step(i + 1, &mut map);
    }

    let mut sum = 0;
    for row in map {
        println!("{:?}", row);

        for val in row {
            if val.is_some() && val.unwrap() == N_STEPS + 1 {
                sum += 1;
            }
        }
    }

    sum.to_string()
}
