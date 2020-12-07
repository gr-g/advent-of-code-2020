fn solve(input: &str) -> (i64, i64) {
    let mut v: Vec<_> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();
    v.sort();

    let mut result2 = 0;
    for i in 0..v.len()-1 {
        if v[i+1..].binary_search(&(2020-v[i])).is_ok() {
            result2 = v[i] * (2020-v[i]);
            break;
        }
    }

    let mut result3 = 0;
    for i in 0..v.len()-2 {
        for j in i+1..v.len()-1 {
            if v[j+1..].binary_search(&(2020-v[i]-v[j])).is_ok() {
                result3 = v[i] * v[j] * (2020-v[i]-v[j]);
                break;
            }
        }
    }

    (result2, result3)
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
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
1721
979
366
299
675
1456"), (514579, 241861950));
    }
}
