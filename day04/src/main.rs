use core::panic;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", count_valid_passports(input, false));
    println!("part2: {}", count_valid_passports(input, true));
}

fn year_between(input: &str, min: u32, max: u32) -> bool {
    if input.chars().count() != 4 {
        return false;
    }

    let year = match input.parse::<u32>() {
        Ok(year) => year,
        Err(_) => return false,
    };

    return (min..=max).contains(&year);
}

fn is_valid_height(input: &str) -> bool {
    let regex_cm = Regex::new("[0-9][0-9][0-9]cm").unwrap();
    let regex_in = Regex::new("[0-9][0-9]in").unwrap();

    if regex_cm.is_match(input) {
        let height: String = input.chars().take(3).collect();
        let height: u32 = match height.parse() {
            Ok(height) => height,
            Err(_) => return false,
        };

        return (150..=193).contains(&height);
    }

    if regex_in.is_match(input) {
        let height: String = input.chars().take(2).collect();
        let height: u32 = match height.parse() {
            Ok(height) => height,
            Err(_) => return false,
        };

        return (59..=76).contains(&height);
    }

    return false;
}

fn count_valid_passports(input: &str, validate_values: bool) -> u64 {
    let mut count = 0;

    let mut num_passport_fields = 0;
    for line in input.lines() {
        if line.is_empty() {
            // 7 fields required
            if num_passport_fields == 7 {
                count += 1;
            }

            num_passport_fields = 0;
        }

        for pair in line.split_whitespace() {
            let mut split = pair.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();

            let should_insert = if validate_values {
                match key {
                    "byr" => year_between(value, 1920, 2002),
                    "iyr" => year_between(value, 2010, 2020),
                    "eyr" => year_between(value, 2020, 2030),
                    "hgt" => is_valid_height(value),
                    "hcl" => {
                        Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match(value)
                    }
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                    "pid" => Regex::new("^[0-9]{9}$").unwrap().is_match(value),
                    "cid" => false,
                    _ => panic!("invalid key"),
                }
            } else {
                if key == "cid" {
                    false
                } else {
                    true
                }
            };

            if should_insert {
                num_passport_fields += 1;
            }
        }
    }

    if num_passport_fields == 7 {
        count += 1;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::count_valid_passports;

    #[test]
    fn example() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

        assert_eq!(count_valid_passports(input, false), 2);
    }

    #[test]
    fn validate_rules() {
        assert!(Regex::new("\\d{9}").unwrap().is_match("023456789"));
        assert!(Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match("#123456"));
        assert!(Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match("#abcdef"));
        assert!(Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match("#623a2f"));
        assert!(Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match("#123def"));

        assert_eq!(Regex::new("^#([0-9]|[a-f]){6}$").unwrap().is_match("123def"), false);
    }

    #[test]
    fn validate_valid_passports() {
        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

        assert_eq!(count_valid_passports(input, true), 4);
    }

    #[test]
    fn validate_invalid_passports() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

        assert_eq!(count_valid_passports(input, true), 0);
    }
}
