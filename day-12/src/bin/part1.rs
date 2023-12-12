fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn num_permutations(chars: &[char], nums: &[u8]) -> u32 {
    if nums.is_empty() || chars.is_empty() {
        return 0;
    }

    let mut sum: u32 = 0;

    let first = nums[0];

    let mut i = 0;
    let mut got_to_broken = false;
    'wholeloop: while i < chars.len() {
        let c = chars[i];
        match c {
            '.' => {}
            '?' | '#' => 'label: {
                if c == '#' {
                    got_to_broken = true;
                }
                for o in 0..first {
                    match chars.get(i + o as usize) {
                        None => {
                            break 'wholeloop;
                        }
                        Some('.') => {
                            i += o as usize;
                            break 'label;
                        }
                        Some(_) => {}
                    }
                }

                // check what the next char is
                match chars.get(i + first as usize) {
                    Some('#') => {
                        break 'label;
                    }
                    Some(_) => {}
                    None => {
                        if nums.len() == 1 {
                            sum += 1;
                            break 'wholeloop;
                        }
                        if nums.len() > 1 {
                            break 'wholeloop;
                        }
                    }
                }

                // if this was the last num then the permutation is possible
                if nums.len() == 1 {
                    sum += 1;
                    break 'label;
                }

                sum += num_permutations(&chars[i + first as usize + 1..], &nums[1..]);
            }
            _ => todo!(),
        }

        if got_to_broken {
            break 'wholeloop;
        }
        i += 1;
    }

    sum
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let chars: Vec<char> = parts[0].chars().collect();
        let nums: Vec<u8> = parts[1]
            .split(',')
            .filter(|e| *e != ",")
            .map(|e| e.to_string().parse::<u8>().unwrap())
            .collect();

        sum += num_permutations(&chars, &nums);
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
