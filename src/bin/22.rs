use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum GameResult {
    P1Wins(usize),
    P2Wins(usize),
    InfiniteLoop,
}

fn score(p: &VecDeque<u8>) -> usize {
    p.iter().enumerate().map(|(i, c)| (p.len() - i) * *c as usize).sum()
}

fn play(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>, recursion: bool) -> GameResult {
    let mut seen = HashSet::new();

    loop {
        if !seen.insert((p1.clone(), p2.clone())) {
            return GameResult::InfiniteLoop;
        }

        let card_p1 = p1.pop_front().unwrap();
        let card_p2 = p2.pop_front().unwrap();

        if recursion && card_p1 as usize <= p1.len() && card_p2 as usize <= p2.len() {
            let mut sub_p1 = p1.clone();
            sub_p1.truncate(card_p1 as usize);
            let mut sub_p2 = p2.clone();
            sub_p2.truncate(card_p2 as usize);
            match play(&mut sub_p1, &mut sub_p2, recursion) {
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

fn solve(input: &str) -> (usize, usize) {
    let mut players = input.split("\n\n").map(|s| {
        s.lines().skip(1).map(|line| line.parse::<u8>().unwrap()).collect::<VecDeque<u8>>()
    });
    let mut p1 = players.next().unwrap();
    let mut p2 = players.next().unwrap();

    let winning_score = match play(&mut p1.clone(), &mut p2.clone(), false) {
        GameResult::P1Wins(score) => { score },
        GameResult::P2Wins(score) => { score },
        GameResult::InfiniteLoop => { panic!("infinite game") },
    };

    let winning_score_recursive = match play(&mut p1, &mut p2, true) {
        GameResult::P1Wins(score) => { score },
        GameResult::P2Wins(score) => { score },
        GameResult::InfiniteLoop => { panic!("infinite game") },
    };

    (winning_score, winning_score_recursive)
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
10"), (306, 291));
    }

    #[test]
    fn example02() {
        let mut p1: VecDeque<u8> = vec![43, 19].into_iter().collect();
        let mut p2: VecDeque<u8> = vec![2, 29, 14].into_iter().collect();
        assert_eq!(play(&mut p1, &mut p2, false), GameResult::InfiniteLoop);
    }
}
