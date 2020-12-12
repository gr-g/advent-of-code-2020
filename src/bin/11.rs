use advent_of_code_2020::grid::{Direction::*, Grid, Location};
use std::collections::HashSet;

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

    let mut to_be_checked = HashSet::new();
    let mut to_be_changed = Vec::new();

    // mark all seats as "to be checked"
    for (l, c) in model1.values() {
        if c == &'L' {
            to_be_checked.insert(l);
        }
    }

    while !to_be_checked.is_empty() {
        // check seats and mark them as "to be changed"
        // using the rules based on adjacent cells
        for pos in to_be_checked.drain() {
            match (model1.get(&pos), count_adjacent_occupied(&model1, &pos)) {
                (Some('L'), 0) => { to_be_changed.push((pos, '#')); },
                (Some('#'), n) if n >= 4 => { to_be_changed.push((pos, 'L')); },
                _ => {},
            }
        }

        // update seats and mark neighbours as "to be checked"
        for (pos, c) in to_be_changed.drain(..) {
            for d in &[Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight] {
                to_be_checked.insert(pos.go(*d));
            }

            model1.insert(pos, c);
        }
    }
    let model1_occupied = model1.values().filter(|(_, c)| *c == &'#').count();

    let mut model2 = Grid::create_from(input);

    // mark all seats as "to be checked"
    for (l, c) in model2.values() {
        if c == &'L' {
            to_be_checked.insert(l);
        }
    }

    while !to_be_checked.is_empty() {
        // check seats and mark them as "to be changed"
        // using the rules based on visible cells
        for pos in to_be_checked.drain() {
            match (model2.get(&pos), count_visible_occupied(&model2, &pos)) {
                (Some('L'), 0) => { to_be_changed.push((pos, '#')); },
                (Some('#'), n) if n >= 5 => { to_be_changed.push((pos, 'L')); },
                _ => {},
            }
        }

        // update seats and mark neighbours as "to be checked"
        for (pos, c) in to_be_changed.drain(..) {
            for dir in &[Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight] {
                let mut p = pos;
                loop {
                    p = p.go(*dir);
                    let c = model1.get(&p);
                    if c == Some(&'L') || c == Some(&'#') {
                        to_be_checked.insert(p);
                        break;
                    } else if c == None {
                        break;
                    }
                }
            }

            model2.insert(pos, c);
        }
    }
    let model2_occupied = model2.values().filter(|(_, c)| *c == &'#').count();

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
