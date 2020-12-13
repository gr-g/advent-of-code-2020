use advent_of_code_2020::grid::{consts::*, Direction};

fn solve(input: &str) -> (i64, i64) {
    let instructions: Vec<_> = input
        .lines()
        .map(|s| (&s[0..1], s[1..].parse::<i64>().unwrap()))
        .collect();

    let mut pos1 = ORIGIN;
    let mut dir = RIGHT;
    for &(c, n) in &instructions {
        match (c, n) {
            ("N", n) => { pos1 = pos1.go(&UP.times(n)); },
            ("S", n) => { pos1 = pos1.go(&DOWN.times(n)); },
            ("E", n) => { pos1 = pos1.go(&RIGHT.times(n)); },
            ("W", n) => { pos1 = pos1.go(&LEFT.times(n)); },
            ("L", 90) | ("R", 270) => { dir = dir.rotate_left(); },
            ("L", 180) | ("R", 180) => { dir = dir.reverse(); },
            ("L", 270) | ("R", 90) => { dir = dir.rotate_right(); },
            ("F", n) => { pos1 = pos1.go(&dir.times(n)); },
            _ => { panic!("unexpected instruction") },
        }
    }

    let mut pos2 = ORIGIN;
    let mut way = Direction{ dx: 10, dy: -1 };
    for &(c, n) in &instructions {
        match (c, n) {
            ("N", n) => { way = way.add(&UP.times(n)); },
            ("S", n) => { way = way.add(&DOWN.times(n)); },
            ("E", n) => { way = way.add(&RIGHT.times(n)); },
            ("W", n) => { way = way.add(&LEFT.times(n)); },
            ("L", 90) | ("R", 270) => { way = way.rotate_left(); },
            ("L", 180) | ("R", 180) => { way = way.reverse(); },
            ("L", 270) | ("R", 90) => { way = way.rotate_right(); },
            ("F", n) => { pos2 = pos2.go(&way.times(n)); },
            _ => { panic!("unexpected instruction") },
        }
    }

    (pos1.distance(&ORIGIN), pos2.distance(&ORIGIN))
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
