fn play(start: &[usize], turns: usize) -> usize {
    // Use a vector as dictionary, storing in position s the last turn
    // when s was seen (or 0 if never seen). Values stored are limited to
    // 32 bits for efficiency.
    // Note that the generation process is such that the values generated
    // in n turns are less than n.
    let max_possible_value = std::cmp::max(turns, *start.iter().max().unwrap());
    let mut last_seen = vec![0; 1+max_possible_value];

    let mut turn = 1;
    let mut spoken = start[0];

    while turn < start.len() {
        last_seen[spoken] = turn as u32;
        turn += 1;
        spoken = start[turn-1];
    }

    while turn < turns {
        let age = match last_seen[spoken] { 0 => 0, t => turn - t as usize};
        last_seen[spoken] = turn as u32;
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
        assert_eq!(play(&[1, 3, 2], 30000000), 2578);
        assert_eq!(play(&[2, 1, 3], 30000000), 3544142);
        assert_eq!(play(&[1, 2, 3], 30000000), 261214);
        assert_eq!(play(&[2, 3, 1], 30000000), 6895259);
        assert_eq!(play(&[3, 2, 1], 30000000), 18);
        assert_eq!(play(&[3, 1, 2], 30000000), 362);
    }
}
