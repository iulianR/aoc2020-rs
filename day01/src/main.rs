fn main() -> anyhow::Result<()> {
    let entries = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    'part1: for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            if entries[i] + entries[j] == 2020 {
                println!("part1: {}", entries[i] * entries[j]);
                break 'part1;
            }
        }
    }

    'part2: for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            for k in (j + 1)..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    println!("part2: {}", entries[i] * entries[j] * entries[k]);
                    break 'part2;
                }
            }
        }
    }

    Ok(())
}
