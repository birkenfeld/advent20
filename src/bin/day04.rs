use advtools::prelude::*;
use advtools::input::input_string;

fn main() {
    let needed_types = HashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let in_range = |x: &str, low, high| x.parse().map_or(false, |x: i32| x >= low && x <= high);

    let mut all_fields = 0;
    let mut all_valid = 0;

    for passport in input_string().split("\n\n") {
        let mut found_types = HashSet::new();
        let mut valid = true;

        for info in passport.trim().split_whitespace() {
            let (key, val) = info.split(':').collect_tuple().unwrap();
            valid &= match key {
                "byr" => in_range(val, 1920, 2002),
                "iyr" => in_range(val, 2010, 2020),
                "eyr" => in_range(val, 2020, 2030),
                "hgt" => {
                    let (num, unit) = val.split_at(val.len() - 2);
                    if unit == "cm" { in_range(num, 150, 193) } else { in_range(num, 59, 76) }
                },
                "hcl" => val.starts_with('#') && u32::from_str_radix(&val[1..], 16).is_ok(),
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),
                "pid" => val.len() == 9 && val.chars().all(|ch| ch.is_digit(10)),
                "cid" => continue,
                _ => false,
            };
            found_types.insert(key);
        }

        if found_types == needed_types {
            all_fields += 1;
            all_valid += valid as u32;
        }
    }

    advtools::verify("Passports with all fields", all_fields, 254);
    advtools::verify("Valid passports", all_valid, 184);
}
