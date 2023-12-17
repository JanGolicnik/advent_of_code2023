use std::collections::HashSet;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
type Beam = ((usize, usize), Dir);

#[derive(Debug)]
struct Tile {
    structure: Option<char>,
    beams: HashSet<Beam>,
}

fn apply_dir(pos: &mut (usize, usize), dir: Dir, map: &[Vec<Tile>]) -> bool {
    match dir {
        Dir::Up => {
            if pos.1 == 0 {
                false
            } else {
                pos.1 -= 1;
                true
            }
        }
        Dir::Down => {
            if pos.1 == map.len() - 1 {
                false
            } else {
                pos.1 += 1;
                true
            }
        }
        Dir::Left => {
            if pos.0 == 0 {
                false
            } else {
                pos.0 -= 1;
                true
            }
        }
        Dir::Right => {
            if pos.0 == map.first().unwrap().len() - 1 {
                false
            } else {
                pos.0 += 1;
                true
            }
        }
    }
}

fn run_beam(mut beam: Beam, map: &mut [Vec<Tile>], new_beams: &mut Vec<Beam>, mut skip_dir: bool) {
    while !map[beam.0 .1][beam.0 .0].beams.contains(&beam) {
        map[beam.0 .1][beam.0 .0].beams.insert(beam);
        if !skip_dir && !apply_dir(&mut beam.0, beam.1, map) {
            break;
        }
        match map[beam.0 .1][beam.0 .0].structure {
            Some('|') => {
                println!("hey");
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Up, map) {
                    new_beams.push((beam.0, Dir::Up));
                }
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Down, map) {
                    new_beams.push((beam.0, Dir::Down))
                }
                println!("| at {:?}", beam.0);
                break;
            }
            Some('-') => {
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Left, map) {
                    new_beams.push((beam.0, Dir::Left));
                }
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Right, map) {
                    new_beams.push((beam.0, Dir::Right))
                }
                println!("- at {:?}", beam.0);
                break;
            }
            Some('\\') => {
                match beam.1 {
                    Dir::Up => beam.1 = Dir::Left,
                    Dir::Down => beam.1 = Dir::Right,
                    Dir::Left => beam.1 = Dir::Up,
                    Dir::Right => beam.1 = Dir::Down,
                }
                print!("\\");
            }
            Some('/') => {
                match beam.1 {
                    Dir::Up => beam.1 = Dir::Right,
                    Dir::Down => beam.1 = Dir::Left,
                    Dir::Left => beam.1 = Dir::Down,
                    Dir::Right => beam.1 = Dir::Up,
                }
                print!("/");
            }
            None => {}
            Some(_) => todo!(),
        }
        if skip_dir && !apply_dir(&mut beam.0, beam.1, map) {
            break;
        }
        skip_dir = false;
        println!(" at {:?}", beam.0);
    }
}

fn part1(input: &str) -> String {
    let mut map: Vec<Vec<Tile>> = Vec::new();

    for line in input.lines() {
        map.push(
            line.chars()
                .map(|e| Tile {
                    structure: if e == '.' { None } else { Some(e) },
                    beams: HashSet::new(),
                })
                .collect(),
        );
    }

    let mut beams_to_run: Vec<Beam> = vec![((0, 0), Dir::Right)];
    let mut finished_beams: HashSet<Beam> = HashSet::new();
    let mut i = 0;
    while let Some(beam) = beams_to_run.pop() {
        i += 1;
        let mut potential_beams: Vec<Beam> = Vec::new();
        run_beam(beam, &mut map, &mut potential_beams, i == 1);
        for potential in potential_beams.iter() {
            if !finished_beams.contains(potential) {
                beams_to_run.push(*potential);
                finished_beams.insert(*potential);
            }
        }
        potential_beams.clear();
    }

    let mut sum = 0;
    for row in map {
        for tile in row {
            if !tile.beams.is_empty() {
                sum += 1;
            }
        }
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
