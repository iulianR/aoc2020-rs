use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", count_answers(input));
    println!("part2: {}", count_all_yes_answers(input));
}

fn count_answers(input: &str) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            count += set.len();
            set.clear();
            continue;
        }

        for c in line.chars() {
            set.insert(c);
        }
    }

    count += set.len();

    count
}

fn count_all_yes_answers(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut member_count = 0;
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            for (_k, v) in map.iter() {
                if *v == member_count {
                    count += 1;
                }
            }
            map.clear();
            member_count = 0;
            continue;
        }

        member_count += 1;
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
    }

    for (_k, v) in map.iter() {
        if *v == member_count {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{count_all_yes_answers, count_answers};

    #[test]
    fn example() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;

        assert_eq!(count_answers(input), 11);
        assert_eq!(count_all_yes_answers(input), 6);
    }
}
