fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", count_trees(input, 3, 1));

    let mut part2 = 1;
    part2 *= count_trees(input, 1, 1);
    part2 *= count_trees(input, 3, 1);
    part2 *= count_trees(input, 5, 1);
    part2 *= count_trees(input, 7, 1);
    part2 *= count_trees(input, 1, 2);
    println!("part2: {}", part2);
}

fn count_trees(input: &str, right: usize, down: usize) -> u64 {
    let mut x = right;
    let mut trees = 0;

    for line in input.lines().skip(down).step_by(down) {
        let chars: Vec<char> = line.chars().collect();
        let index = x % chars.len();

        if chars[index] == '#' {
            trees += 1;
        }

        x += right;
    }

    return trees;
}

#[cfg(test)]
mod tests {
    use crate::count_trees;

    #[test]
    fn example() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;

        assert_eq!(count_trees(input, 1, 1), 2);
        assert_eq!(count_trees(input, 3, 1), 7);
        assert_eq!(count_trees(input, 5, 1), 3);
        assert_eq!(count_trees(input, 7, 1), 4);
        assert_eq!(count_trees(input, 1, 2), 2);
    }
}
