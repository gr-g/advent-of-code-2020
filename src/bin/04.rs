use std::collections::HashMap;

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn create_from(s: &str) -> Passport {
        let mut fields = HashMap::new();
        for field in s.split_ascii_whitespace() {
            let mut it = field.split(':');
            fields.insert(
                it.next().unwrap().to_string(),
                it.next().unwrap().to_string()
            );
        }
        Passport { fields }
    }
    
    fn is_complete(&self) -> bool {
        [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ]
            .iter().all(|&f| self.fields.contains_key(f))
    }
    
    fn is_valid(&self) -> bool {
        if !self.is_complete() { return false; }
        let pass = &self.fields;

        // validate Birth Year
        if !pass["byr"].parse::<u64>().ok()
                       .map_or(false, |n| n >= 1920 && n <= 2002) {
            return false;
        }

        // validate Issue Year
        if !pass["iyr"].parse::<u64>().ok()
                       .map_or(false, |n| n >= 2010 && n <= 2020) {
            return false;
        }

        // validate Expiration Year
        if !pass["eyr"].parse::<u64>().ok()
                       .map_or(false, |n| n >= 2020 && n <= 2030) {
            return false;
        }

        // validate Height
        if let Some(hgt) = pass["hgt"].strip_suffix("cm") {
            if !hgt.parse::<u64>().ok()
                   .map_or(false, |n| n >= 150 && n <= 193) {
                return false;
            }
        }
        else if let Some(hgt) = pass["hgt"].strip_suffix("in") {
            if !hgt.parse::<u64>().ok()
                   .map_or(false, |n| n >= 59 && n <= 76) {
                return false;
            }
        }
        else {
            return false;
        }

        // validate Hair Color
        if !(match pass["hcl"].split_at(1) {
            ("#", color) => {
                color.len() == 6 && color.chars().all(|c| {
                    c.is_ascii_digit() || (c >= 'a' && c <= 'f')
                })
            }
            _ => { false }
        }) {
            return false;
        }
        
        // validate Eye Color
        if ![ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ].contains(&pass["ecl"].as_str()) {
            return false;
        }

        // validate Passport ID
        if !(pass["pid"].len() == 9 && pass["pid"].chars().all(|c| {
            c.is_ascii_digit()
        })) {
            return false;
        }

        true
    }
}

fn solve(input: &str) -> (usize, usize) {
    let batch: Vec<_> = input.split("\n\n").map(Passport::create_from).collect();

    let n_complete = batch.iter().filter(|&p| p.is_complete()).count();
    let n_valid = batch.iter().filter(|&p| p.is_valid()).count();

    (n_complete, n_valid)
}

fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let s = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let is_complete = Passport::create_from(s).is_complete();
        assert!(is_complete, "is_complete({}): {}", s, is_complete);
    }

    #[test]
    fn example02() {
        let s = "\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let is_complete = Passport::create_from(s).is_complete();
        assert!(!is_complete, "is_complete({}): {}", s, is_complete);
    }

    #[test]
    fn example03() {
        let s = "\
hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        let is_complete = Passport::create_from(s).is_complete();
        assert!(is_complete, "is_complete({}): {}", s, is_complete);
    }

    #[test]
    fn example04() {
        let s = "\
hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let is_complete = Passport::create_from(s).is_complete();
        assert!(!is_complete, "is_complete({}): {}", s, is_complete);
    }

    #[test]
    fn example05() {
        let s = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(!is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example06() {
        let s = "\
iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(!is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example07() {
        let s = "\
hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(!is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example08() {
        let s = "\
hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(!is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example09() {
        let s = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example10() {
        let s = "\
eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example11() {
        let s = "\
hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(is_valid, "is_valid({}): {}", s, is_valid);
    }

    #[test]
    fn example12() {
        let s = "\
iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let is_valid = Passport::create_from(s).is_valid();
        assert!(is_valid, "is_valid({}): {}", s, is_valid);
    }
}
