fn main() {
    let input = include_str!("./input_part2.txt");
    let output = part1(input);
    dbg!(output);
}

const RESTRICTIONS: [u32; 3] = [12, 13, 14]; //rgb

fn part1(input: &str) -> String {
    //get games
    let mut sum: u32 = 0;

    for mut line in input.lines() {
        line = &line[5..];
        let game_num_index = line.find(':').unwrap();
        let game_num = &line[..game_num_index];
        let game_num = game_num.to_string().parse::<u32>().unwrap();

        let sets = &line[game_num_index + 1..];
        let sets: Vec<&str> = sets.split(';').collect();

        let mut max_num_red: u32 = 0;
        let mut max_num_green: u32 = 0;
        let mut max_num_blue: u32 = 0;

        for set in sets {
            let mut num_red: u32 = 0;
            let mut num_green: u32 = 0;
            let mut num_blue: u32 = 0;

            let values: Vec<&str> = set.split(',').collect();
            for mut value in values {
                value = value.trim();
                let num_index = value.find(' ').unwrap();
                let num = &value[..num_index].to_string().parse::<u32>().unwrap();
                let color = &value[num_index + 1..];
                match color {
                    "red" => num_red += num,
                    "green" => num_green += num,
                    "blue" => num_blue += num,
                    _ => {}
                }

                if num_red > max_num_red {
                    max_num_red = num_red;
                }
                if num_green > max_num_green {
                    max_num_green = num_green;
                }
                if num_blue > max_num_blue {
                    max_num_blue = num_blue;
                }
            }
        }
        sum += max_num_red * max_num_green * max_num_blue;
    }

    sum.to_string()

    //get sets

    //parse each set
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
