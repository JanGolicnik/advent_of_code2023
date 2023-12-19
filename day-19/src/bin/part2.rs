use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_accepted(
    workflow_label: &str,
    mut part: [(u32, u32); 4],
    workflows: &HashMap<&str, (Vec<(u8, char, u32, &str)>, &str)>,
    accepted: &mut Vec<[(u32, u32); 4]>,
) {
    println!("{workflow_label} {:?}", part);
    let mut current_workflow = workflows[workflow_label].clone();
    'outer: loop {
        for (category, operator, num, next_label) in current_workflow.0.iter() {
            if let Some(next) = match *operator {
                '>' => {
                    if part[*category as usize].0 > *num {
                        None
                    } else {
                        let mut ret = part;
                        ret[*category as usize].0 = *num + 1;
                        part[*category as usize].1 = *num;
                        Some(ret)
                    }
                }
                '<' => {
                    if part[*category as usize].1 < *num {
                        None
                    } else {
                        let mut ret = part;
                        ret[*category as usize].1 = *num - 1;
                        part[*category as usize].0 = *num;
                        Some(ret)
                    }
                }
                _ => panic!(),
            } {
                match *next_label {
                    "R" => {}
                    "A" => {
                        accepted.push(next);
                    }
                    _ => {
                        find_accepted(next_label, next, workflows, accepted);
                    }
                }
            } else {
                match *next_label {
                    "R" => {
                        return;
                    }
                    "A" => {
                        accepted.push(part);
                        return;
                    }
                    _ => {
                        current_workflow = workflows[next_label].clone();
                        continue 'outer;
                    }
                }
            }
        }
        match current_workflow.1 {
            "R" => {
                return;
            }
            "A" => {
                accepted.push(part);
                return;
            }
            _ => current_workflow = workflows[current_workflow.1].clone(),
        }
    }
}

fn part1(input: &str) -> String {
    let mut workflows: HashMap<&str, (Vec<(u8, char, u32, &str)>, &str)> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('{').collect();

        let label = &parts[0];

        let rules: Vec<&str> = parts[1][..parts[1].len() - 1].split(',').collect();
        let mut parsed_rules: Vec<(u8, char, u32, &str)> = Vec::new();
        let mut last_rule = "";
        for rule in rules {
            if let Some((op_index, _)) = rule
                .chars()
                .enumerate()
                .find(|(_, e)| *e == '>' || *e == '<')
            {
                let category = match &rule[..op_index] {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
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

    let mut accepted: Vec<[(u32, u32); 4]> = Vec::new();
    find_accepted("in", [(0, 4000); 4], &workflows, &mut accepted);
    println!("{:?}", accepted);

    let mut sum: u64 = 0;
    for part in accepted {
        let mut variations: u64 = 1;
        for val in part {
            variations *= (val.1 - val.0) as u64;
        }
        sum += variations;
    }

    sum.to_string()
}
