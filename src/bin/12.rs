#[derive(Debug, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn go(&mut self, dir: &Vector, amount: i64) {
        self.x += dir.x * amount;
        self.y += dir.y * amount;
    }

    fn rotate_left(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        self.x = -self.x;
    }

    fn rotate_right(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
        self.y = -self.y;
    }

    fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn solve(input: &str) -> (i64, i64) {
    let instructions: Vec<_> = input
        .lines()
        .map(|s| (&s[0..1], s[1..].parse::<i64>().unwrap()))
        .collect();

    let (north, south, east, west) = (
        Vector{ x: 0, y: 1 },
        Vector{ x: 0, y: -1 },
        Vector{ x: 1, y: 0 },
        Vector{ x: -1, y: 0 },
    );

    let mut pos1 = Vector{ x: 0, y: 0 };
    let mut dir = east;
    for &(c, n) in &instructions {
        match (c, n) {
            ("N", n) => { pos1.go(&north, n); },
            ("S", n) => { pos1.go(&south, n); },
            ("E", n) => { pos1.go(&east, n); },
            ("W", n) => { pos1.go(&west, n); },
            ("F", n) => { pos1.go(&dir, n); },
            ("L", 90) | ("R", 270) => { dir.rotate_left(); },
            ("L", 180) | ("R", 180) => { dir.reverse(); },
            ("L", 270) | ("R", 90) => { dir.rotate_right(); },
            _ => { panic!() },
        }
    }

    let mut pos2 = Vector{ x: 0, y: 0 };
    let mut way = Vector{ x: 10, y: 1 };
    for &(c, n) in &instructions {
        match (c, n) {
            ("N", n) => { way.go(&north, n); },
            ("S", n) => { way.go(&south, n); },
            ("E", n) => { way.go(&east, n); },
            ("W", n) => { way.go(&west, n); },
            ("F", n) => { pos2.go(&way, n); },
            ("L", 90) | ("R", 270) => { way.rotate_left(); },
            ("L", 180) | ("R", 180) => { way.reverse(); },
            ("L", 270) | ("R", 90) => { way.rotate_right(); },
            _ => { panic!() },
        }
    }

    (pos1.abs(), pos2.abs())
}

fn main() {
    let input = std::fs::read_to_string("input/12.txt").unwrap();
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
F10
N3
F7
R90
F11"), (25, 286));
    }
}
