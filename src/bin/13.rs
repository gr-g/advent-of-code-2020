fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum.rem_euclid(prod))
}

fn read_bus_list(s: &str) -> (Vec<i64>, Vec<i64>) {
    let mut minutes = Vec::new();
    let mut busses = Vec::new();
    let mut m = 0;
    for bus in s.split(',') {
        if let Some(p) = bus.parse::<i64>().ok() {
            minutes.push(m);
            busses.push(p);
        }
        m -= 1;
    }
    (busses, minutes)
}

fn earliest_bus(busses: &[i64], timestamp: i64) -> (i64, i64) {
    (0..).find_map(|wait| {
        match busses.iter().find(|bus| (timestamp + wait) % **bus == 0) {
            Some(bus) => Some((wait, *bus)),
            None => None,
        }
    }).unwrap()
}

fn contest_timestamp(busses: &[i64], minutes: &[i64]) -> i64 {
    chinese_remainder(&minutes, &busses).unwrap()
}

fn solve(input: &str) -> (i64, i64) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<i64>().unwrap();
    let (busses, minutes) = read_bus_list(lines.next().unwrap());

    let (wait, bus) = earliest_bus(&busses, timestamp);
    let contest_timestamp = contest_timestamp(&busses, &minutes);

    (wait * bus, contest_timestamp)
}

fn main() {
    let input = std::fs::read_to_string("input/13.txt").unwrap();
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
939
7,13,x,x,59,x,31,19"), (295, 1068781));
    }

    #[test]
    fn example02() {
        let (busses, minutes) = read_bus_list("17,x,13,19");
        assert_eq!(contest_timestamp(&busses, &minutes), 3417);
    }

    #[test]
    fn example03() {
        let (busses, minutes) = read_bus_list("67,7,59,61");
        assert_eq!(contest_timestamp(&busses, &minutes), 754018);
    }

    #[test]
    fn example04() {
        let (busses, minutes) = read_bus_list("67,x,7,59,61");
        assert_eq!(contest_timestamp(&busses, &minutes), 779210);
    }

    #[test]
    fn example05() {
        let (busses, minutes) = read_bus_list("67,7,x,59,61");
        assert_eq!(contest_timestamp(&busses, &minutes), 1261476);
    }

    #[test]
    fn example06() {
        let (busses, minutes) = read_bus_list("1789,37,47,1889");
        assert_eq!(contest_timestamp(&busses, &minutes), 1202161486);
    }
}
