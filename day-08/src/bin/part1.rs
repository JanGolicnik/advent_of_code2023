use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

type Location = String;

fn part1(input: &str) -> String {
    let mut instructions: Vec<char> = Vec::new();
    let mut i = 0;
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() {
            i = index + 1;
            break;
        }
        let mut values: Vec<char> = line.chars().collect();
        instructions.append(&mut values);
    }

    let mut nodes: HashMap<Location, (Location, Location)> = HashMap::new();

    let mut first: Location = "ZZZ".to_string();

    for (index, line) in input.lines().enumerate().skip(i) {
        let node = line[..3].chars().collect::<String>();
        let left = line[7..10].chars().collect::<String>();
        let right = line[12..15].chars().collect::<String>();
        if index == i {
            first = node.clone();
        }
        nodes.insert(node, (left, right));
    }

    let mut instruction_index = 0;

    while first != "ZZZ" {
        let node = &nodes[&first];

        first = match instructions[instruction_index % instructions.len()] {
            'L' => node.0.clone(),
            'R' => node.1.clone(),
            _ => todo!(),
        };

        println!("{first}");

        instruction_index += 1;
    }

    instruction_index.to_string()
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
