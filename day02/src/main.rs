fn main() -> anyhow::Result<()> {
    let lines = include_str!("../input.txt").lines();

    let part1 = lines
        .clone()
        .map(|line| line.split_whitespace())
        .fold(0, |total, tokens| {
            let tokens: Vec<&str> = tokens.collect();
            match &tokens[..] {
                [first, second, third] => {
                    let mut min_max = first.split("-");
                    let min: usize = min_max.next().unwrap().parse().unwrap();
                    let max: usize = min_max.next().unwrap().parse().unwrap();

                    let letter = second.strip_suffix(":").unwrap();

                    let password = third;

                    let count = password.matches(letter).count();
                    if count >= min && count <= max {
                        return total + 1;
                    } else {
                        return total;
                    }
                }
                _ => panic!("invalid input"),
            }
        });

    println!("part1 valid passwords: {}", part1);

    let part2 = lines
        .map(|line| line.split_whitespace())
        .fold(0, |total, tokens| {
            let tokens: Vec<&str> = tokens.collect();
            match &tokens[..] {
                [first, second, third] => {
                    let mut min_max = first.split("-");
                    let min: usize = min_max.next().unwrap().parse().unwrap();
                    let max: usize = min_max.next().unwrap().parse().unwrap();

                    let letter = second.strip_suffix(":").unwrap().chars().nth(0).unwrap();

                    let password = third;

                    let is_match = |index| password.chars().nth(index).unwrap() == letter;

                    let match1 = is_match(min - 1);
                    let match2 = is_match(max - 1);

                    if match1 ^ match2 {
                        return total + 1;
                    } else {
                        return total;
                    }
                }
                _ => panic!("invalid input"),
            }
        });

    println!("part2 valid passwords: {}", part2);

    Ok(())
}
