use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str) -> (usize, usize) {
    let mut rules = HashMap::new();
    for l in input.lines() {
        let (outer, inner_list) = l.split_once(" bags contain ").unwrap();
        let inner: Vec<_> = inner_list
            .split(", ")
            .filter_map(|s| {
                let mut words = s.split(' ');
                Some((
                    words.next().unwrap().parse::<usize>().ok()?,
                    words.next().unwrap().to_string() + " " + words.next().unwrap()
                ))
            })
            .collect();
        rules.insert(outer, inner);
    }

    let mut can_contain = HashSet::new();
    let mut target = vec!["shiny gold"];
    while let Some(target_color) = target.pop() {
        for (outer, inner) in &rules {
            if inner.iter().any(|(_, color)| target_color == color) {
                if can_contain.insert(outer) {
                    target.push(outer);
                }
            }
        }
    }

    let mut total_contained = 0;
    let mut target = vec![(1, "shiny gold")];
    while let Some((target_n, target_color)) = target.pop() {
        for (n, color) in &rules[target_color] {
            total_contained += target_n * n;
            target.push((target_n * n, &color));
        }
    }

    (can_contain.len(), total_contained)
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
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
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."), (4, 32));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."), (0, 126));
    }
}
