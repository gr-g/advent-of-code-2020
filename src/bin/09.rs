#[derive(Debug)]
struct XmasCode  {
    code: Vec<u64>,
    window_len: usize,
}

impl XmasCode {
    fn is_valid_position(&self, pos: usize) -> bool {
        assert!(pos >= self.window_len && pos < self.code.len());

        for i in pos-self.window_len..pos-1 {
            for j in i+1..pos {
                if self.code[pos] == self.code[i]+self.code[j] {
                    return true;
                }
            }
        }
        false
    }

    fn find_range_with_sum(&self, target_sum: u64) -> Option<&[u64]> {
        for i in 0..self.code.len() {
            let mut sum = 0;
            let mut j = i;
            while sum <= target_sum && j < self.code.len() {
                sum += self.code[j];
                j += 1;
                if sum == target_sum {
                    return Some(&self.code[i..j]);
                }
            }
        }
        None
    }
}

fn solve(input: &str, window_len: usize) -> (u64, u64) {
    let code: Vec<_> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    let c = XmasCode{ code, window_len };

    let invalid_pos = (c.window_len..c.code.len()).find(|&i| !c.is_valid_position(i)).unwrap();
    let invalid_value = c.code[invalid_pos];

    let weakness_range = c.find_range_with_sum(invalid_value).unwrap();
    let weakness = weakness_range.iter().min().unwrap() + weakness_range.iter().max().unwrap();

    (invalid_value, weakness)
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 25);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let mut c = XmasCode{
            code: vec![20, 1, 25, 2, 24, 3, 23, 4, 22, 5, 21, 6, 19, 7, 18, 8, 17, 9, 16, 10, 15, 11, 14, 12, 13, 0],
            window_len: 25,
        };
        c.code[25] = 26;
        assert!(c.is_valid_position(25), "{:?}: is_valid_position(25)? {}", c, c.is_valid_position(25));
        c.code[25] = 49;
        assert!(c.is_valid_position(25), "{:?}: is_valid_position(25)? {}", c, c.is_valid_position(25));
        c.code[25] = 100;
        assert!(!c.is_valid_position(25), "{:?}: is_valid_position(25)? {}", c, c.is_valid_position(25));
        c.code[25] = 50;
        assert!(!c.is_valid_position(25), "{:?}: is_valid_position(25)? {}", c, c.is_valid_position(25));
    }

    #[test]
    fn example02() {
        let mut c = XmasCode{
            code: vec![20, 1, 25, 2, 24, 3, 23, 4, 22, 5, 21, 6, 19, 7, 18, 8, 17, 9, 16, 10, 15, 11, 14, 12, 13, 45, 0],
            window_len: 25,
        };
        c.code[26] = 26;
        assert!(c.is_valid_position(26), "{:?}: is_valid_position(26)? {}", c, c.is_valid_position(26));
        c.code[26] = 65;
        assert!(!c.is_valid_position(26), "{:?}: is_valid_position(26)? {}", c, c.is_valid_position(26));
        c.code[26] = 64;
        assert!(c.is_valid_position(26), "{:?}: is_valid_position(26)? {}", c, c.is_valid_position(26));
        c.code[26] = 66;
        assert!(c.is_valid_position(26), "{:?}: is_valid_position(26)? {}", c, c.is_valid_position(26));
    }

    #[test]
    fn example03() {
        assert_eq!(solve("\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576", 5), (127, 62));
    }
}
