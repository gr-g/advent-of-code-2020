use advent_of_code_2020::grid::{Direction::*, Grid, Location};
use std::collections::HashMap;

fn count_adjacent_occupied( g: &Grid, pos: &Location ) -> usize {
    [Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight]
        .iter()
        .filter(|dir| {
            g.get(&pos.go(**dir)) == Some(&'#')
        })
        .count()
}

fn count_visible_occupied( g: &Grid, pos: &Location ) -> usize {
    [Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight]
        .iter()
        .filter(|dir| {
            let mut p = *pos;
            loop {
                p = p.go(**dir);
                match g.get(&p) {
                    Some(&'#') => { break true; },
                    Some(&'L') | None => { break false; },
                    _ => {},
                }
            }
        })
        .count()
}

fn solve(input: &str) -> (usize, usize) {
    let mut model1 = Grid::create_from(input);
    let mut count = HashMap::new();
    let mut changed = true;
    while changed {
        count.clear();
        changed = false;

        // count adjacent occupied seats
        for (pos, c) in &model1.symbols {
            match *c {
                'L' | '#' => { count.insert(*pos, count_adjacent_occupied(&model1, pos)); },
                _ => {},
            }
        }

        // update seat states
        for (pos, n) in &count {
            match (model1.get(&pos), n) {
                (Some('L'), 0)            => { model1.insert(*pos, '#'); changed = true; },
                (Some('#'), n) if *n >= 4 => { model1.insert(*pos, 'L'); changed = true; },
                _ => {},
            }
        }
    }
    let model1_occupied = model1.symbols.values().filter(|c| **c == '#').count();

    let mut model2 = Grid::create_from(input);
    count = HashMap::new();
    changed = true;
    while changed {
        count.clear();
        changed = false;

        // count visible occupied seats
        for (pos, c) in &model2.symbols {
            match *c {
                'L' | '#' => { count.insert(*pos, count_visible_occupied(&model2, pos)); },
                _ => {},
            }
        }

        // update seat states
        for (pos, n) in &count {
            match (model2.get(&pos), n) {
                (Some('L'), 0)            => { model2.insert(*pos, '#'); changed = true; },
                (Some('#'), n) if *n >= 5 => { model2.insert(*pos, 'L'); changed = true; },
                _ => {},
            }
        }
    }
    let model2_occupied = model2.symbols.values().filter(|c| **c == '#').count();

    (model1_occupied, model2_occupied)
}

fn main() {
    let input = std::fs::read_to_string("input/11.txt").unwrap();
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
        let g = Grid::create_from("\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....");
        assert_eq!(count_visible_occupied(&g, &Location{ x: 3, y: 4}), 8);
    }

    #[test]
    fn example02() {
        let g = Grid::create_from("\
.............
.L.L.#.#.#.#.
.............");
        assert_eq!(count_visible_occupied(&g, &Location{ x: 1, y: 1}), 0);
        assert_eq!(count_visible_occupied(&g, &Location{ x: 3, y: 1}), 1);
    }

    #[test]
    fn example03() {
        let g = Grid::create_from("\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.");
        assert_eq!(count_visible_occupied(&g, &Location{ x: 3, y: 3}), 0);
    }

    #[test]
    fn example04() {
        assert_eq!(solve("\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"), (37, 26));
    }
}
