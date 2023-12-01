fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut first: Option<char> = None;
        let mut second: Option<char> = None;

        let chars_len = chars.len();

        for i in 0..chars_len {
            if first.is_none() && chars[i].is_numeric() {
                first = Some(chars[i]);
            }
            if second.is_none() && chars[chars_len - i - 1].is_numeric() {
                second = Some(chars[chars_len - i - 1]);
            }
            if first.is_some() && second.is_some() {
                break;
            }
        }

        if let Some(first) = first {
            if let Some(second) = second {
                if let Some(first) = first.to_digit(10) {
                    if let Some(second) = second.to_digit(10) {
                        sum += first * 10 + second;
                    }
                }
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
