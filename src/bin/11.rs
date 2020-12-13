use advent_of_code_2020::grid::{consts::*, FixedGrid, Direction};

const DIRECTIONS: [Direction; 8] = [UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

fn adjacency_map(grid: &FixedGrid) -> Vec<Vec<usize>> {
    let mut adjacency_map = vec![vec![]; grid.data().len()];
    for i in 0..grid.data().len() {
        let pos = grid.index_to_location(i);
        for d in &DIRECTIONS {
            let dp = pos.go(d);
            match grid.get(&dp) {
                Some(&b'L') | Some(&b'#') => {
                    adjacency_map[i].push(grid.location_to_index(&dp));
                },
                _ => {},
            }
        }
    }
    adjacency_map
}

fn visibility_map(grid: &FixedGrid) -> Vec<Vec<usize>> {
    let mut visibility_map = vec![vec![]; grid.data().len()];
    for i in 0..grid.data().len() {
        let pos = grid.index_to_location(i);
        for d in &DIRECTIONS {
            let dp = pos.go_until(d, |p| grid.get(p) != Some(&b'.')).unwrap();
            match grid.get(&dp) {
                Some(&b'L') | Some(&b'#') => {
                    visibility_map[i].push(grid.location_to_index(&dp));
                },
                _ => {},
            }
        }
    }
    visibility_map
}

fn count_occupied( data: &[u8], neighbors: &[usize] ) -> usize {
    neighbors
        .iter()
        .filter(|i| data.get(**i) == Some(&b'#'))
        .count()
}

fn run_simulation(grid: &mut FixedGrid, neighbor_map: &[Vec<usize>], threshold: usize) {
    let mut to_be_checked = vec![true; grid.data().len()];
    let mut to_be_changed = Vec::new();

    loop {
        // check seats and mark them as "to be changed"
        let data = grid.data();
        for i in 0..data.len() {
            if !to_be_checked[i] { continue; }
            match data.get(i) {
                Some(b'L') => {
                    if count_occupied(data, &neighbor_map[i]) == 0 {
                        to_be_changed.push((i, b'#'));
                    }
                },
                Some(b'#') => {
                    if count_occupied(data, &neighbor_map[i]) >= threshold {
                        to_be_changed.push((i, b'L'));
                    }
                },
                _ => {},
            }
            to_be_checked[i] = false;
        }

        if to_be_changed.is_empty() { break; }

        // update seats and mark neighbors as "to be checked"
        for (i, c) in to_be_changed.drain(..) {
            grid.set(i, c);

            for &ni in &neighbor_map[i] {
                to_be_checked[ni] = true;
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid1 = FixedGrid::create_from(input);

    // prepare the map with the neighbours of each seat
    // based on the adjacency rules and run the simulation
    let adjacency_map = adjacency_map(&grid1);
    run_simulation(&mut grid1, &adjacency_map, 4);
    let occupied1 = grid1.data().iter().filter(|c| *c == &b'#').count();

    let mut grid2 = FixedGrid::create_from(input);

    // prepare the map with the neighbours of each seat
    // based on the visibility rules and run the simulation
    let visibility_map = visibility_map(&grid2);
    run_simulation(&mut grid2, &visibility_map, 5);
    let occupied2 = grid2.data().iter().filter(|c| *c == &b'#').count();

    (occupied1, occupied2)
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
    use advent_of_code_2020::grid::Location;

    #[test]
    fn example01() {
        let g = FixedGrid::create_from("\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....");
        let visibility_map = visibility_map(&g);
        let neighbors = &visibility_map[g.location_to_index(&Location{ x: 3, y: 4 })];
        assert_eq!(count_occupied(g.data(), neighbors), 8);
    }

    #[test]
    fn example02() {
        let g = FixedGrid::create_from("\
.............
.L.L.#.#.#.#.
.............");
        let visibility_map = visibility_map(&g);
        let neighbors = &visibility_map[g.location_to_index(&Location{ x: 1, y: 1 })];
        assert_eq!(count_occupied(g.data(), neighbors), 0);
        let neighbors = &visibility_map[g.location_to_index(&Location{ x: 3, y: 1 })];
        assert_eq!(count_occupied(g.data(), neighbors), 1);
    }

    #[test]
    fn example03() {
        let g = FixedGrid::create_from("\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.");
        let visibility_map = visibility_map(&g);
        let neighbors = &visibility_map[g.location_to_index(&Location{ x: 3, y: 3 })];
        assert_eq!(count_occupied(g.data(), neighbors), 0);
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
