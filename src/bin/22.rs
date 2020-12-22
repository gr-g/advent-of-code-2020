use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq)]
enum GameResult {
    P1Wins(usize),
    P2Wins(usize),
    InfiniteLoop,
}

fn hash(p1: &VecDeque<u8>, p2: &VecDeque<u8>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    p1.hash(&mut hasher);
    p2.hash(&mut hasher);
    hasher.finish()
}

fn score(p: &VecDeque<u8>) -> usize {
    p.iter().enumerate().map(|(i, c)| (p.len() - i) * *c as usize).sum()
}

fn play(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> GameResult {
    loop {
        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        if card_p1 > card_p2 {
            p1.push_back(card_p1);
            p1.push_back(card_p2);
        }
        else {
            p2.push_back(card_p2);
            p2.push_back(card_p1);
        }

        if p1.is_empty() {
            return GameResult::P2Wins(score(&p2));
        }
        if p2.is_empty() {
            return GameResult::P1Wins(score(&p1));
        }
    }
}

fn play_recursive(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> GameResult {
    let mut seen = HashSet::new();

    loop {
        if !seen.insert(hash(p1, p2)) {
            return GameResult::InfiniteLoop;
        }

        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        if card_p1 as usize <= p1.len() && card_p2 as usize <= p2.len() {
            let mut sub_p1 = p1.clone();
            sub_p1.truncate(card_p1 as usize);
            let mut sub_p2 = p2.clone();
            sub_p2.truncate(card_p2 as usize);
            match play_recursive(&mut sub_p1, &mut sub_p2) {
                GameResult::InfiniteLoop | GameResult::P1Wins(_) => {
                    p1.push_back(card_p1);
                    p1.push_back(card_p2);
                },
                GameResult::P2Wins(_) => {
                    p2.push_back(card_p2);
                    p2.push_back(card_p1);
                },
            }
        } else {
            if card_p1 > card_p2 {
                p1.push_back(card_p1);
                p1.push_back(card_p2);
            }
            else {
                p2.push_back(card_p2);
                p2.push_back(card_p1);
            }
        }

        if p1.is_empty() {
            return GameResult::P2Wins(score(&p2));
        }
        if p2.is_empty() {
            return GameResult::P1Wins(score(&p1));
        }
    }
}

fn solve(input: &str) -> (GameResult, GameResult) {
    let mut players = input.split("\n\n").map(|s| {
        s.lines().skip(1).map(|line| line.parse::<u8>().unwrap()).collect::<VecDeque<u8>>()
    });
    let mut p1 = players.next().unwrap();
    let mut p2 = players.next().unwrap();

    let result = play(&mut p1.clone(), &mut p2.clone());
    let result_recursive = play_recursive(&mut p1, &mut p2);

    (result, result_recursive)
}

fn main() {
    let input = std::fs::read_to_string("input/22.txt").unwrap();
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
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"), (GameResult::P2Wins(306), GameResult::P2Wins(291)));
    }

    #[test]
    fn example02() {
        let mut p1 = VecDeque::from(vec![43, 19]);
        let mut p2 = VecDeque::from(vec![2, 29, 14]);
        assert_eq!(play_recursive(&mut p1, &mut p2), GameResult::InfiniteLoop);
    }
}
