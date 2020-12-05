use std::env;
use std::fs;

#[derive(Debug)]
struct RawPassport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Default for RawPassport {
    fn default() -> RawPassport {
        RawPassport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
}

fn parse(file: &str) -> Vec<RawPassport> {
    file.split("\n\n")
        .map(|x| {
            let mut last_idx: usize = 0;
            let mut pp: RawPassport = Default::default();
            let mut i = x.char_indices().peekable();
            while i.peek().is_some() {
                i.find(|(_i, c)| *c == ':').unwrap();
                let (i1, _c) = i.next().unwrap();
                let key_slice = &x[last_idx..i1 - 1];
                last_idx = i1;

                // Calculate the end point of the value slice,
                // if we see a space or a newline and we're not at the end,
                // then that space or newline marks the end, otherwise use
                // the end of the passport string
                let end =
                    if i.find(|(_i, c)| *c == ' ' || *c == '\n').is_some() && i.peek().is_some() {
                        let (i2, _c) = i.next().unwrap();
                        i2 - 1
                    } else {
                        x.len()
                    };
                let value_slice = &x[last_idx..end];
                last_idx = end + 1;

                let dest_ref = match key_slice {
                    "byr" => &mut pp.birth_year,
                    "iyr" => &mut pp.issue_year,
                    "eyr" => &mut pp.expiration_year,
                    "hgt" => &mut pp.height,
                    "hcl" => &mut pp.hair_color,
                    "ecl" => &mut pp.eye_color,
                    "pid" => &mut pp.passport_id,
                    "cid" => &mut pp.country_id,
                    &_ => panic!("unknown passport key"),
                };
                *dest_ref = Some(value_slice.to_string());
            }
            pp
        })
        .collect::<Vec<RawPassport>>()
}

fn valid_part1(pp: &RawPassport) -> bool {
    pp.birth_year.is_some()
        && pp.issue_year.is_some()
        && pp.expiration_year.is_some()
        && pp.height.is_some()
        && pp.hair_color.is_some()
        && pp.eye_color.is_some()
        && pp.passport_id.is_some()
}

fn valid_part2(pp: &RawPassport) -> bool {
    valid_part1(pp)
        && match pp.birth_year.as_ref().unwrap().parse::<i32>() {
            Ok(x) => x >= 1920 && x <= 2002,
            Err(_e) => false,
        }
        && match pp.issue_year.as_ref().unwrap().parse::<i32>() {
            Ok(x) => x >= 2010 && x <= 2020,
            Err(_e) => false,
        }
        && match pp.expiration_year.as_ref().unwrap().parse::<i32>() {
            Ok(x) => x >= 2020 && x <= 2030,
            Err(_e) => false,
        }
        && {
            let mut hi = pp
                .height
                .as_ref()
                .unwrap()
                .char_indices()
                .skip_while(|(_, c)| c.is_digit(10));
            let maybe_end_of_digits = hi.next();
            match maybe_end_of_digits {
                Some((end_of_digits, _c)) => {
                    let digits_slice = &(pp.height.as_ref().unwrap())[..end_of_digits];
                    let units_slice = &(pp.height.as_ref().unwrap())[end_of_digits..];

                    match units_slice {
                        "cm" => match digits_slice.parse::<i32>() {
                            Ok(x) => x >= 150 && x <= 193,
                            Err(_e) => false,
                        },
                        "in" => match digits_slice.parse::<i32>() {
                            Ok(x) => x >= 59 && x <= 76,
                            Err(_e) => false,
                        },

                        &_ => false,
                    }
                }
                None => false,
            }
        }
        && {
            let mut hci = pp.hair_color.as_ref().unwrap().chars();
            if hci.next().unwrap() != '#' {
                false
            } else {
                hci.all(|x| {
                    x.is_ascii_hexdigit()
                        && (x.is_digit(10) || (x.is_alphabetic() && x.is_lowercase()))
                })
            }
        }
        && match pp.eye_color.as_ref().unwrap().as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            &_ => false,
        }
        && pp.passport_id.as_ref().unwrap().len() == 9
        && pp
            .passport_id
            .as_ref()
            .unwrap()
            .chars()
            .all(|x| x.is_digit(10))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    let pps = parse(&file_contents);
    let valid_count_part1 = pps.iter().filter(|x| valid_part1(x)).count();
    println!("part 1 valid count: {}", valid_count_part1);

    let valid_count_part2 = pps.iter().filter(|x| valid_part2(x)).count();
    println!("part 2 valid count: {}", valid_count_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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

    #[test]
    fn test_parse() {
        let pps = parse(EXAMPLE);
    }

    #[test]
    fn test_par1() {
        let pps = parse(EXAMPLE);
        let valid_count = pps.iter().filter(|x| valid_part1(x)).count();
        assert_eq!(valid_count, 2);
    }
}
