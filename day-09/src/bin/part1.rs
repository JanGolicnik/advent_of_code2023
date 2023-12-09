fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn is_vec_all_zeroes(vec: &Vec<i32>) -> bool {
    for n in vec {
        if *n != 0 {
            return false;
        }
    }
    true
}

fn find_next_in_sequence(nums: Vec<i32>) -> i32 {
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    let mut current_sequence = nums;

    while !is_vec_all_zeroes(&current_sequence) {
        sequences.push(current_sequence);

        let sequence = sequences.last().unwrap();

        let mut new_sequence: Vec<i32> = Vec::new();
        new_sequence.reserve(sequence.len() - 1);
        let mut i = 1;
        while i < sequence.len() {
            new_sequence.push(sequence[i] - sequence[i - 1]);
            i += 1;
        }
        current_sequence = new_sequence;
    }

    let mut i: i32 = sequences.len() as i32 - 1;
    while i >= 0 {
        let new_value;
        let this = &sequences[i as usize];
        if let Some(lower) = sequences.get(i as usize + 1) {
            new_value = *this.last().unwrap() + *lower.last().unwrap();
        } else {
            new_value = *this.last().unwrap();
        }
        let this = &mut sequences[i as usize];
        this.push(new_value);
        i -= 1;
    }

    println!("{:?}", sequences);

    *sequences[0].last().unwrap()
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split(' ')
            .map(|e| e.to_string().parse::<i32>().unwrap())
            .collect();

        sum += find_next_in_sequence(nums);
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
