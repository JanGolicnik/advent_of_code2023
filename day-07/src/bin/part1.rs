use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_part1.txt");
    let output = part1(input);
    dbg!(output);
}

type Hand = Vec<char>;
type Strength = u8;
fn get_hand_strength(hand: &Hand) -> Strength {
    let mut mapped: HashMap<char, u8> = HashMap::new();
    for c in hand {
        if mapped.contains_key(c) {
            *mapped.get_mut(c).unwrap() += 1;
        } else {
            mapped.insert(*c, 1);
        }
    }

    match mapped.len() {
        1 => 7,
        2 => {
            let first = mapped.iter().next().unwrap();
            if *first.1 == 4 || *first.1 == 1 {
                6
            } else {
                5
            }
        }
        3 => {
            for el in mapped.iter() {
                if *el.1 == 3 {
                    return 4;
                }
            }
            3
        }
        4 => 2,
        5 => 1,
        _ => todo!(),
    }
}

fn get_card_strength(card: char) -> u8 {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => todo!(),
    }
}

fn part1(input: &str) -> String {
    type Bid = u32;
    let mut hands: Vec<(Hand, Bid, Strength)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let mut hand: Hand = Vec::new();
        hand.reserve(5);
        for i in 0..5 {
            hand.push(parts[0].chars().nth(i).unwrap());
        }

        let bid = parts[1].to_string().trim().parse::<u32>().unwrap();
        let strength = get_hand_strength(&hand);
        hands.push((hand, bid, strength));
    }

    hands.sort_by(|a, b| {
        if a.2 == b.2 {
            let mut i = 0;
            while a.0[i] == b.0[i] {
                i += 1;
            }
            get_card_strength(a.0[i]).cmp(&get_card_strength(b.0[i]))
        } else {
            a.2.cmp(&b.2)
        }
    });

    let mut sum = 0;
    for (index, (_, bid, _)) in hands.iter().enumerate() {
        sum += (index as u32 + 1) * *bid;
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
