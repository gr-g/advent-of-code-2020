const P: u64 = 20201227;

fn pow_p(base: u64, exp: u64) -> u64 {
    if exp == 0 {
        1
    } else if exp % 2 == 1 {
        let x = pow_p(base, exp / 2);
        (((x * x) % P) * base) % P
    } else {
        let x = pow_p(base, exp / 2);
        (x * x) % P
    }
}

fn log_p(base: u64, value: u64) -> Option<u64> {
    let mut v = 1;
    for exp in 0..P {
        if v == value { return Some(exp); }
        v = (v * base) % P;
    }
    None
}

fn solve(input: &str) -> u64 {
    let mut pubkeys = input.lines().map(|line| line.parse::<u64>().unwrap());
    let card_pubkey = pubkeys.next().unwrap();
    let door_pubkey = pubkeys.next().unwrap();

    //let card_loops = log_p(7, card_pubkey).unwrap();
    let door_loops = log_p(7, door_pubkey).unwrap();

    //let encryption_key = pow_p(7, card_loops * door_loops);
    //let card_encryption_key = pow_p(door_pubkey, card_loops);
    let door_encryption_key = pow_p(card_pubkey, door_loops);
    //assert_eq!(encryption_key, card_encryption_key);
    //assert_eq!(encryption_key, door_encryption_key);

    door_encryption_key
}

fn main() {
    let input = std::fs::read_to_string("input/25.txt").unwrap();
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
5764801
17807724"), 14897079);
    }
}
