fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

fn strtonum(input: &str) -> Option<u32> {
    Some(match input {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => return None,
    })
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        const WORDS: [&str; 18] = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        let mut first: Option<(&str, usize)> = None;
        let mut second: Option<(&str, usize)> = None;
        for word in WORDS.iter() {
            if let Some(i) = line.find(word) {
                if first.is_none() || i < first.unwrap().1 {
                    first = Some((word, i));
                }
            }
            if let Some(i) = line.rfind(word) {
                if second.is_none() || i > second.unwrap().1 {
                    second = Some((word, i));
                }
            }
        }

        if let Some(first) = first {
            if let Some(second) = second {
                if let Some(first) = strtonum(first.0) {
                    if let Some(second) = strtonum(second.0) {
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
