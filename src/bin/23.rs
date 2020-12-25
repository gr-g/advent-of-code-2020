struct Cups {
    // Position i represents the cup with label 'i+1'.
    // next[i] indicates the cup next to i in the circle.
    next: Vec<usize>,
    current: usize,
}

impl Cups {
    fn create_from(input: &str, n_cups: usize) -> Cups {
        let input_cups: Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap() as usize - 1).collect();
        let mut next = vec![0; n_cups];
        let mut current = input_cups[0];
        let mut i = 1;
        while i < n_cups {
            next[current] = *input_cups.get(i).unwrap_or(&i);
            current = next[current];
            i += 1;
        }
        next[current] = input_cups[0];
        current = next[current];

        Cups{ next, current }
    }

    fn do_moves(&mut self, n_moves: usize) {
        let n = self.next.len();

        for _ in 0..n_moves {
            // detach 3 cups from the chain
            let c1 = self.next[self.current];
            let c2 = self.next[c1];
            let c3 = self.next[c2];
            self.next[self.current] = self.next[c3];

            // find destination
            let mut dest = (self.current + n - 1) % n;
            while dest == c1 || dest == c2 || dest == c3 {
                dest = (dest + n - 1) % n;
            }

            // reattach the 3 cups after the destination cup
            let cd = self.next[dest];
            self.next[dest] = c1;
            self.next[c3] = cd;

            // advance the current cup
            self.current = self.next[self.current];
        }
    }

    fn to_string_from_1(&self) -> String {
        let mut s = String::new();
        let mut c = 0;
        for _ in 1..self.next.len() {
            c = self.next[c];
            s.push_str(&(c+1).to_string());
        }
        s
    }
}

fn solve(input: &str) -> (String, usize) {
    let mut cups9 = Cups::create_from(input, 9);
    cups9.do_moves(100);
    let result9 = cups9.to_string_from_1();

    let mut cups1mil = Cups::create_from(input, 1_000_000);
    cups1mil.do_moves(10_000_000);
    let c1 = cups1mil.next[0];
    let c2 = cups1mil.next[c1];
    let result1mil = (c1+1) * (c2+1);

    (result9, result1mil)
}

fn main() {
    let now = std::time::Instant::now();
    let s = solve("389547612");
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let mut cups = Cups::create_from("389125467", 9);
        cups.do_moves(10);
        assert_eq!(cups.to_string_from_1(), "92658374");
    }

    #[test]
    fn example02() {
        assert_eq!(solve("389125467"), ("67384529".to_string(), 149245887792));
    }
}
