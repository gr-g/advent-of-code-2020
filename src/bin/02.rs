struct DatabaseEntry {
    policy: (usize, usize, char),
    password: String,
}

impl DatabaseEntry {
    fn create_from(s: &str) -> DatabaseEntry {
        let mut parts = s.split(": ");
        let mut policy_parts = parts.next().unwrap().split(' ');
        let mut policy_nums = policy_parts.next().unwrap().split('-');

        let policy = (
            policy_nums.next().unwrap().parse().unwrap(),
            policy_nums.next().unwrap().parse().unwrap(),
            policy_parts.next().unwrap().chars().nth(0).unwrap(),
        );
        let password = parts.next().unwrap().to_string();

        DatabaseEntry { policy, password }
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
