use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Rule {
    Literal(char),
    And(Vec<usize>),
    Or(Vec<Rule>),
}

impl Rule {
    fn create_from(r: &[&str]) -> Rule {
        if r.len() == 1 && r[0].starts_with('"') {
            return Rule::Literal(r[0].chars().nth(1).unwrap());
        }
        if let Some(i) = r.iter().position(|x| *x == "|") {
            return Rule::Or(vec![Rule::create_from(&r[0..i]), Rule::create_from(&r[i+1..])]);
        }
        Rule::And(r.iter().map(|x| x.parse::<usize>().unwrap()).collect())
    }
}

#[derive(Clone, Debug)]
struct RuleSet {
    rules: HashMap<usize, Rule>,
}

impl RuleSet {
    fn create_from(s: &str) -> RuleSet {
        let mut rules = HashMap::new();
        for line in s.lines() {
            let (id, rule) = line.split_once(": ").unwrap();
            let id = id.parse::<usize>().unwrap();
            let rule = rule.split_whitespace().collect::<Vec<_>>();
            rules.insert(id, Rule::create_from(&rule));
        }
        RuleSet{ rules }
    }

    // Matches a rule in all possible ways on the initial part of a message,
    // returning the possible leftover messages after positive matches.
    fn matches<'a>(&self, rule: &Rule, msg: &'a str) -> Vec<&'a str> {
        //println!("Matching rule {:?} on {}", rule, msg);
        match rule {
            Rule::Literal(c) => {
                match msg.strip_prefix(*c) {
                    Some(msg_tail) => { vec![msg_tail] },
                    _ => { vec![] },
                }
            }
            Rule::And(list) => {
                match list.split_first() {
                    Some((i, tail)) => {
                        self.matches(&self.rules[i], msg)
                            .into_iter()
                            .flat_map(|msg_tail| self.matches(&Rule::And(tail.to_vec()), msg_tail))
                            .collect()
                    },
                    None => { vec![msg] },
                }
            }
            Rule::Or(list) => {
                list.iter().flat_map(|r| self.matches(r, msg)).collect()
            }
        }
    }

    fn fully_matches(&self, rule: usize, msg: &str) -> bool {
        self.matches(&self.rules[&rule], msg).iter().find(|r| r.is_empty()).is_some()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (rules, messages) = input.split_once("\n\n").unwrap();

    let ruleset = RuleSet::create_from(rules);
    let valid = messages.lines()
        .filter(|msg| {
            let result = ruleset.fully_matches(0, msg);
            //println!("Tested original ruleset on {} -> {}", msg, result);
            result
        })
        .count();

    let mut ruleset_mod = ruleset.clone();
    ruleset_mod.rules.insert(8, Rule::create_from(&["42", "|", "42", "8"]));
    ruleset_mod.rules.insert(11, Rule::create_from(&["42", "31", "|", "42", "11", "31"]));
    let valid_mod = messages.lines()
        .filter(|msg| {
            let result = ruleset_mod.fully_matches(0, msg);
            //println!("Tested modified ruleset on {} -> {}", msg, result);
            result
        })
        .count();

    (valid, valid_mod)
}

fn main() {
    let input = std::fs::read_to_string("input/19.txt").unwrap();
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
        let ruleset = RuleSet::create_from("\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"");
        assert!(ruleset.fully_matches(0, "ababbb"));
        assert!(!ruleset.fully_matches(0, "bababa"));
        assert!(ruleset.fully_matches(0, "abbbab"));
        assert!(!ruleset.fully_matches(0, "aaabbb"));
        assert!(!ruleset.fully_matches(0, "aaaabbb"));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), (3, 12));
    }
}
