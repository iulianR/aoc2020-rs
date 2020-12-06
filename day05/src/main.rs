fn main() {
    let input = include_str!("../input.txt");

    let mut boarding_passes: Vec<i32> = input
        .lines()
        .map(|line| {
            line.chars().fold(0, |acc, c| -> i32 {
                let half = if c == 'B' || c == 'R' { 1 } else { 0 };
                acc << 1 | half
            })
        })
        .collect();

    println!("part1: {}", boarding_passes.iter().max().unwrap());

    boarding_passes.sort();

    println!(
        "part2: {}",
        boarding_passes
            .windows(2)
            .find(|window| window[0] + 1 != window[1])
            .unwrap()[0]
            + 1
    );
}
