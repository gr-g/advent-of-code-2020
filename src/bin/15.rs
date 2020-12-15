use std::collections::HashMap;

fn play(start: &[usize], turns: usize) -> usize {
    let mut seen = HashMap::new();
    let mut turn = 1;
    let mut spoken = start[0];

    while turn < start.len() {
        seen.insert(spoken, turn);
        turn += 1;
        spoken = start[turn-1];
    }

    while turn < turns {
        let age = turn - seen.insert(spoken, turn).unwrap_or(turn);
        turn += 1;
        spoken = age;
        //println!("turn {}: spoken {}", turn, spoken);
    }

    spoken
}

fn solve(input: &str) -> (usize, usize) {
    let v: Vec<_> = input.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    (play(&v, 2020), play(&v, 30000000))
}

fn main() {
    let now = std::time::Instant::now();
    let s = solve("6,4,12,1,20,0,16");
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(play(&[0, 3, 6], 4), 0);
        assert_eq!(play(&[0, 3, 6], 5), 3);
        assert_eq!(play(&[0, 3, 6], 6), 3);
        assert_eq!(play(&[0, 3, 6], 7), 1);
        assert_eq!(play(&[0, 3, 6], 8), 0);
        assert_eq!(play(&[0, 3, 6], 9), 4);
        assert_eq!(play(&[0, 3, 6], 10), 0);
        assert_eq!(play(&[0, 3, 6], 2020), 436);
    }

    #[test]
    fn example02() {
        assert_eq!(play(&[1, 3, 2], 2020), 1);
        assert_eq!(play(&[2, 1, 3], 2020), 10);
        assert_eq!(play(&[1, 2, 3], 2020), 27);
        assert_eq!(play(&[2, 3, 1], 2020), 78);
        assert_eq!(play(&[3, 2, 1], 2020), 438);
        assert_eq!(play(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    #[ignore]
    fn example03() {
        assert_eq!(play(&[0, 3, 6], 30000000), 175594);
    }

    #[test]
    #[ignore]
    fn example04() {
        assert_eq!(play(&[1, 3, 2], 30000000), 2578);
    }

    #[test]
    #[ignore]
    fn example05() {
        assert_eq!(play(&[2, 1, 3], 30000000), 3544142);
    }

    #[test]
    #[ignore]
    fn example06() {
        assert_eq!(play(&[1, 2, 3], 30000000), 261214);
    }

    #[test]
    #[ignore]
    fn example07() {
        assert_eq!(play(&[2, 3, 1], 30000000), 6895259);
    }

    #[test]
    #[ignore]
    fn example08() {
        assert_eq!(play(&[3, 2, 1], 30000000), 18);
    }

    #[test]
    #[ignore]
    fn example09() {
        assert_eq!(play(&[3, 1, 2], 30000000), 362);
    }
}
