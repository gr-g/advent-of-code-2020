struct DatabaseEntry {
    policy: (usize, usize, char),
    password: String,
}

impl DatabaseEntry {
    fn create_from(s: &str) -> DatabaseEntry {
        let (policy_str, password) = s.split_once(": ").unwrap();
        let (policy_nums, policy_char) = policy_str.split_once(" ").unwrap();
        let (policy_n1, policy_n2) = policy_nums.split_once("-").unwrap();

        DatabaseEntry {
            policy: (
                policy_n1.parse().unwrap(),
                policy_n2.parse().unwrap(),
                policy_char.chars().nth(0).unwrap()
            ),
            password: password.to_string()
        }
    }

    fn is_valid1(&self) -> bool {
        let (min, max, c) = self.policy;
        let n = self.password.chars().filter(|x| *x == c).count();
        n >= min && n <= max
    }

    fn is_valid2(&self) -> bool {
        let (p1, p2, c) = self.policy;
        (self.password.chars().nth(p1-1).unwrap() == c)
            != (self.password.chars().nth(p2-1).unwrap() == c)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let v: Vec<_> = input.lines().map(DatabaseEntry::create_from).collect();

    let n_valid1 = v.iter().filter(|p| p.is_valid1()).count();
    let n_valid2 = v.iter().filter(|p| p.is_valid2()).count();

    (n_valid1, n_valid2)
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
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
        assert_eq!(solve("\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"), (2, 1));
    }
}
