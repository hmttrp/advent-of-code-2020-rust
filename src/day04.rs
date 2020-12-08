use regex::Regex;
use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "./input/day04.txt";
// const INPUT_FILE: &str = "./input/day04_example.txt";

fn get_puzzle() -> Vec<String> {
    let file = fs::read_to_string(INPUT_FILE).expect("Failed reading puzzle file");
    file.split("\n\n")
        .map(|s: &str| s.replace("\n", " ").to_string())
        .collect()
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn has_required_keys(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        let birth_year_valid = match &self.byr {
            Some(birth_year) => {
                let year: i32 = birth_year.parse().unwrap_or(0);
                year >= 1920 && year <= 2002
            }
            None => false,
        };

        let issue_year_valid = match &self.iyr {
            Some(issue_year) => {
                let year: i32 = issue_year.parse().unwrap_or(0);
                year >= 2010 && year <= 2020
            }
            None => false,
        };

        let expiry_year_valid = match &self.eyr {
            Some(expiry_year) => {
                let year: i32 = expiry_year.parse().unwrap_or(0);
                year >= 2020 && year <= 2030
            }
            None => false,
        };

        let height_valid = match &self.hgt {
            Some(height) => {
                let cm_regex = Regex::new(r"\d{3}cm").unwrap();
                let in_regex = Regex::new(r"\d{2}in").unwrap();
                if cm_regex.is_match(&height) {
                    height
                        .replace("cm", "")
                        .parse::<i32>()
                        .map(|h: i32| h >= 150 && h <= 193)
                        .unwrap_or(false)
                } else if in_regex.is_match(&height) {
                    height
                        .replace("in", "")
                        .parse::<i32>()
                        .map(|h: i32| h >= 59 && h <= 76)
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            None => false,
        };

        let hair_color_valid = match &self.hcl {
            Some(hair_color) => {
                let re = Regex::new("#[0-9a-f]{6}").unwrap();
                re.is_match(&hair_color)
            }
            None => false,
        };

        let eye_color_valid = match &self.ecl {
            Some(eye_color) => {
                eye_color == "amb"
                    || eye_color == "blu"
                    || eye_color == "brn"
                    || eye_color == "gry"
                    || eye_color == "grn"
                    || eye_color == "hzl"
                    || eye_color == "oth"
            }
            None => false,
        };

        let pid_valid = match &self.pid {
            Some(pid) => {
                let re = Regex::new("^[0-9]{9}$").unwrap();
                re.is_match(&pid)
            }
            None => false,
        };

        birth_year_valid
            && issue_year_valid
            && expiry_year_valid
            && height_valid
            && hair_color_valid
            && eye_color_valid
            && pid_valid
    }

    fn from_string(s: &str) -> Self {
        let key_value_strings: Vec<Vec<&str>> = s
            .split_whitespace()
            .map(|kv: &str| kv.split(":").collect())
            .collect();

        let mut passport_data = HashMap::new();
        for key_value in key_value_strings {
            passport_data.insert(key_value[0].to_string(), key_value[1].to_string());
        }

        Passport {
            byr: passport_data.get("byr").map(|s| s.to_string()),
            iyr: passport_data.get("iyr").map(|s| s.to_string()),
            eyr: passport_data.get("eyr").map(|s| s.to_string()),
            hgt: passport_data.get("hgt").map(|s| s.to_string()),
            hcl: passport_data.get("hcl").map(|s| s.to_string()),
            ecl: passport_data.get("ecl").map(|s| s.to_string()),
            pid: passport_data.get("pid").map(|s| s.to_string()),
            cid: passport_data.get("cid").map(|s| s.to_string()),
        }
    }
}

fn parse_passports(lines: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec![];

    for line in lines {
        passports.push(Passport::from_string(line));
    }

    passports
}

pub fn part1() -> i64 {
    let lines = get_puzzle();
    let passports = parse_passports(&lines);

    let mut valid_passports = 0;

    for passport in passports {
        if passport.has_required_keys() {
            valid_passports += 1
        }
    }

    valid_passports
}

pub fn part2() -> i64 {
    let lines = get_puzzle();
    let passports = parse_passports(&lines);

    let mut valid_passports = 0;

    for passport in passports {
        if passport.has_required_keys() && passport.is_valid() {
            valid_passports += 1
        }
    }

    valid_passports
}
