use std::vec;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut previous_category: Vec<(u64, u64)> = Vec::new();
    let mut current_category: Vec<(u64, u64)> = Vec::new();

    let mut after_adding_seeds = false;

    for mut line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(index) = line.find("seeds: ") {
            line = &line[index + "seeds: ".len()..];
            previous_category.clear();
            let numbers: Vec<&str> = line.split(' ').collect();
            for num in numbers {
                previous_category.push((num.to_string().trim().parse::<u64>().unwrap(), 0));
            }
            after_adding_seeds = true;
            continue;
        }

        if line.contains("map:") {
            if !after_adding_seeds {
                let mut vector_to_add: Vec<(u64, u64)> = Vec::new();
                for previous in previous_category.iter() {
                    if !current_category.contains(previous) {
                        current_category.push(*previous);
                    }
                }
                current_category.append(&mut vector_to_add);
                println!("-------------\nprevious {:?}", previous_category);
                println!("current {:?}", current_category);

                previous_category.clear();
            } else {
                println!("-------------\nprevious {:?}", previous_category);
                println!("current {:?}", current_category);
            }
            after_adding_seeds = false;
            previous_category.append(&mut current_category);
            current_category.clear();
            continue;
        }

        let numbers: Vec<&str> = line.split(' ').collect();
        if numbers.len() != 3 {
            continue;
        }

        let dest: u64 = numbers[0].to_string().trim().parse::<u64>().unwrap();
        let source: u64 = numbers[1].to_string().trim().parse::<u64>().unwrap();
        let length: u64 = numbers[2].to_string().trim().parse::<u64>().unwrap();

        let mut i = 0;
        while i < previous_category.len() {
            let (n, l) = previous_category[i];

            let start = source.max(n);
            let end = (source + length).min(n + l);
            if start > end {
                i += 1;
                continue;
            }
            let diff = start - source.min(n);
            current_category.push((dest + diff, end - start));

            previous_category.remove(i);
        }
    }

    let mut vector_to_add: Vec<(u64, u64)> = Vec::new();
    for previous in previous_category.iter() {
        if !current_category.contains(previous) {
            current_category.push(*previous);
        }
    }
    current_category.append(&mut vector_to_add);

    let mut lowest = u64::MAX;
    for (num, range) in current_category {
        if num < lowest {
            lowest = num;
        }
    }

    lowest.to_string()
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
