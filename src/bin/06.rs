fn solve(input: &str) -> (usize, usize) {
    let groups: Vec<_> = input.split("\n\n").collect();

    // for each group, record the size of the group and the frequency of "yes" answers
    let yes_answers_by_group: Vec<_> = groups
        .iter()
        .map(|group| {
            let mut group_size = 0;
            let mut yes_answers = [0; 26];
            for person in group.lines() {
                for c in person.as_bytes() {
                    match c {
                        b'a'..=b'z' => { yes_answers[(c-b'a') as usize] += 1; },
                        _ => { panic!() },
                    }
                }
                group_size += 1;
            }
            (group_size, yes_answers)
        }).collect();

    let yes_answers_by_any = yes_answers_by_group
        .iter()
        .map(|&(_, yes)| yes.iter().filter(|&n| *n > 0).count())
        .sum();

    let yes_answers_by_all = yes_answers_by_group
        .iter()
        .map(|&(size, yes)| yes.iter().filter(|&n| *n == size).count())
        .sum();

    (yes_answers_by_any, yes_answers_by_all)
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
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
abc

a
b
c

ab
ac

a
a
a
a

b"), (11, 6));
    }
}
