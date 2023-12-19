use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

fn part1(input: &str) -> String {
    let mut workflows: HashMap<&str, (Vec<(Category, char, u32, &str)>, &str)> = HashMap::new();

    let lines = input.lines();
    let mut parts_index: usize = 0;
    for line in lines {
        parts_index += 1;

        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('{').collect();

        let label = &parts[0];

        let rules: Vec<&str> = parts[1][..parts[1].len() - 1].split(',').collect();
        let mut parsed_rules: Vec<(Category, char, u32, &str)> = Vec::new();
        let mut last_rule = "";
        for rule in rules {
            if let Some((op_index, _)) = rule
                .chars()
                .enumerate()
                .find(|(_, e)| *e == '>' || *e == '<')
            {
                let category = match &rule[..op_index] {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => panic!(),
                };

                let operator = rule.chars().nth(op_index).unwrap();

                let num_and_label: Vec<&str> = rule[op_index + 1..].split(':').collect();
                let num = num_and_label[0].to_string().parse::<u32>().unwrap();
                let label = num_and_label[1];

                parsed_rules.push((category, operator, num, label));

                continue;
            }

            last_rule = rule;
        }

        workflows.insert(label, (parsed_rules, last_rule));
    }

    let mut parts: Vec<[u32; 4]> = Vec::new();
    for line in input.lines().skip(parts_index) {
        let line = &line[1..line.len() - 1];
        let unparsed_parts: Vec<&str> = line.split(',').collect();
        let mut parsed_part: [u32; 4] = [0; 4];
        for (index, part) in unparsed_parts.iter().enumerate() {
            let parts: Vec<&str> = part.split('=').collect();
            let num = parts[1].to_string().parse::<u32>().unwrap();
            parsed_part[index] = num;
        }
        parts.push(parsed_part);
    }

    println!("{:?}", workflows);
    println!();
    println!("{:?}", parts);

    let mut accepted_parts: Vec<[u32; 4]> = Vec::new();

    'partsloop: for part in parts {
        let mut current_workflow = workflows["in"].clone();
        'outer: loop {
            for (category, operator, num, label) in current_workflow.0.iter() {
                let num = *num;
                let category_val = match category {
                    Category::X => part[0],
                    Category::M => part[1],
                    Category::A => part[2],
                    Category::S => part[3],
                };
                if match operator {
                    '>' => category_val > num,
                    '<' => category_val < num,
                    _ => panic!(),
                } {
                    match *label {
                        "R" => {
                            continue 'partsloop;
                        }
                        "A" => {
                            accepted_parts.push(part);
                            continue 'partsloop;
                        }
                        _ => {
                            current_workflow = workflows[label].clone();
                            continue 'outer;
                        }
                    }
                }
            }
            match current_workflow.1 {
                "R" => {
                    break;
                }
                "A" => {
                    accepted_parts.push(part);
                    break;
                }
                _ => current_workflow = workflows[current_workflow.1].clone(),
            }
        }
    }

    println!();
    println!("{:?}", accepted_parts);

    let mut sum = 0;
    for part in accepted_parts {
        for val in part {
            sum += val;
        }
    }

    sum.to_string()
}
