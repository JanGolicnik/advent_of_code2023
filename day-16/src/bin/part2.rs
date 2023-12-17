use core::num;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part2(input);
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
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Up, map) {
                    new_beams.push((beam.0, Dir::Up));
                }
                let mut new_pos = beam.0;
                if apply_dir(&mut new_pos, Dir::Down, map) {
                    new_beams.push((beam.0, Dir::Down))
                }
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
                break;
            }
            Some('\\') => match beam.1 {
                Dir::Up => beam.1 = Dir::Left,
                Dir::Down => beam.1 = Dir::Right,
                Dir::Left => beam.1 = Dir::Up,
                Dir::Right => beam.1 = Dir::Down,
            },
            Some('/') => match beam.1 {
                Dir::Up => beam.1 = Dir::Right,
                Dir::Down => beam.1 = Dir::Left,
                Dir::Left => beam.1 = Dir::Down,
                Dir::Right => beam.1 = Dir::Up,
            },
            None => {}
            Some(_) => todo!(),
        }
        if skip_dir && !apply_dir(&mut beam.0, beam.1, map) {
            break;
        }
        skip_dir = false;
    }
}

fn num_energized_tiles(beam: Beam, map: &mut [Vec<Tile>]) -> u32 {
    let mut beams_to_run: Vec<Beam> = vec![beam];
    let mut finished_beams: HashSet<Beam> = HashSet::new();
    let mut i = 0;
    while let Some(beam) = beams_to_run.pop() {
        i += 1;
        let mut potential_beams: Vec<Beam> = Vec::new();
        run_beam(beam, map, &mut potential_beams, i == 1);
        for potential in potential_beams.iter() {
            if !finished_beams.contains(potential) {
                beams_to_run.push(*potential);
                finished_beams.insert(*potential);
            }
        }
    }

    let mut sum = 0;
    for row in map {
        for tile in row {
            if !tile.beams.is_empty() {
                sum += 1;
            }
            tile.beams.clear();
        }
    }

    sum
}

fn part2(input: &str) -> String {
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

    let mut max = 0;

    for i in 0..map.len() {
        max = max.max(num_energized_tiles(((i, 0), Dir::Down), &mut map));
        max = max.max(num_energized_tiles(((i, map.len() - 1), Dir::Up), &mut map));
    }

    let width = map.first().unwrap().len();
    for i in 0..width {
        max = max.max(num_energized_tiles(((0, i), Dir::Right), &mut map));
        max = max.max(num_energized_tiles(((width - 1, i), Dir::Left), &mut map));
    }

    max.to_string()
}
