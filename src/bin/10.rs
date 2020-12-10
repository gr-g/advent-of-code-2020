fn solve(input: &str) -> (i64, i64) {
    let mut v: Vec<_> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    v.push(0);
    v.sort();
    v.push(v.last().unwrap()+3);

    let mut count1 = 0;
    let mut count3 = 0;
    for i in 1..v.len() {
        if v[i] - v[i-1] == 1 { count1 += 1; }
        if v[i] - v[i-1] == 3 { count3 += 1; }
    }

    let mut paths = vec![0; v.len()];
    paths[0] = 1;
    for i in 1..v.len() {
        // The number of different paths reaching step i (paths[i])
        // is the sum of paths[p] for all p < i from which i can be
        // reached in one step.
        for p in i.saturating_sub(3)..i {
            if v[i] - v[p] <= 3 { paths[i] += paths[p]; }
        }
    }

    (count1 * count3, *paths.last().unwrap())
}

fn main() {
    let input = std::fs::read_to_string("input/10.txt").unwrap();
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
16
10
15
5
1
11
7
19
6
12
4"), (7 * 5, 8));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"), (22 * 10, 19208));
    }
}
