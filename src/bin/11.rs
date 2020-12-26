use advent_of_code_2020::grid::{consts::*, Direction, Grid, SimpleGrid};

const DIRECTIONS: [Direction; 8] = [UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
enum State { Floor, EmptySeat, OccupiedSeat }

// Returns the seats as a vector v together with an adjacency map m:
// v[i] is the state of position i and m[i] is the list of seats adjacent to i.
fn adjacency_map(grid: &SimpleGrid) -> (Vec<State>, Vec<Vec<usize>>) {
    let mut seats = vec![State::Floor; grid.rows()*grid.cols()];
    let mut adjacency_map = vec![vec![]; grid.rows()*grid.cols()];
    for (pos, c) in grid.cells() {
        let i = pos.y as usize * grid.cols() + pos.x as usize;
        match c {
            '#' => { seats[i] = State::OccupiedSeat; },
            'L' => { seats[i] = State::EmptySeat; },
            _ => { continue; },
        }
        for d in &DIRECTIONS {
            let dpos = pos.go(d);
            match grid.get(&dpos) {
                Some('L') | Some('#') => {
                    adjacency_map[i].push(dpos.y as usize * grid.cols() + dpos.x as usize);
                },
                _ => {},
            }
        }
    }
    (seats, adjacency_map)
}

// Returns the seats as a vector v together with a 'visibility' map m:
// v[i] is the state of position i and m[i] is the list of seats adjacent to i.
fn visibility_map(grid: &SimpleGrid) -> (Vec<State>, Vec<Vec<usize>>) {
    let mut seats = vec![State::Floor; grid.rows()*grid.cols()];
    let mut visibility_map = vec![vec![]; grid.rows()*grid.cols()];
    for (pos, c) in grid.cells() {
        let i = pos.y as usize * grid.cols() + pos.x as usize;
        match c {
            '#' => { seats[i] = State::OccupiedSeat; },
            'L' => { seats[i] = State::EmptySeat; },
            _ => { continue; },
        }
        for d in &DIRECTIONS {
            let dpos = pos.go_until(d, |p| grid.get(p) != Some('.')).unwrap();
            match grid.get(&dpos) {
                Some('L') | Some('#') => {
                    visibility_map[i].push(dpos.y as usize * grid.cols() + dpos.x as usize);
                },
                _ => {},
            }
        }
    }
    (seats, visibility_map)
}

fn count_occupied(seats: &[State], neighbors: &[usize]) -> usize {
    neighbors
        .iter()
        .filter(|i| seats[**i] == State::OccupiedSeat)
        .count()
}

fn run_simulation(seats: &mut [State], neighbor_map: &[Vec<usize>], threshold: usize) {
    let mut to_be_checked: Vec<_> = seats.iter().map(|s| *s != State::Floor).collect();
    let mut to_be_changed = Vec::new();

    loop {
        // check seats and mark them as "to be changed"
        for i in 0..seats.len() {
            if !to_be_checked[i] { continue; }
            if seats[i] == State::OccupiedSeat {
                if count_occupied(&seats, &neighbor_map[i]) >= threshold {
                    to_be_changed.push((i, State::EmptySeat));
                }
            } else if seats[i] == State::EmptySeat {
                if count_occupied(&seats, &neighbor_map[i]) == 0 {
                    to_be_changed.push((i, State::OccupiedSeat));
                }
            }
            to_be_checked[i] = false;
        }

        if to_be_changed.is_empty() { break; }

        // update seats and mark neighbors as "to be checked"
        for (i, c) in to_be_changed.drain(..) {
            seats[i] = c;

            for ni in &neighbor_map[i] {
                to_be_checked[*ni] = true;
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let grid = SimpleGrid::create_from(input);

    // prepare and run the simulation using the adjacency rules
    let (mut seats, adjacency_map) = adjacency_map(&grid);
    run_simulation(&mut seats, &adjacency_map, 4);
    let occupied_adj = seats.iter().filter(|s| **s == State::OccupiedSeat).count();

    // prepare and run the simulation using the visibility rules
    let (mut seats, visibility_map) = visibility_map(&grid);
    run_simulation(&mut seats, &visibility_map, 5);
    let occupied_vis = seats.iter().filter(|s| **s == State::OccupiedSeat).count();

    (occupied_adj, occupied_vis)
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
        let g = SimpleGrid::create_from("\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....");
        let (seats, visibility_map) = visibility_map(&g);
        let neighbors = &visibility_map[4 * g.cols() + 3];
        assert_eq!(count_occupied(&seats, neighbors), 8);
    }

    #[test]
    fn example02() {
        let g = SimpleGrid::create_from("\
.............
.L.L.#.#.#.#.
.............");
        let (seats, visibility_map) = visibility_map(&g);
        let neighbors = &visibility_map[1 * g.cols() + 1];
        assert_eq!(count_occupied(&seats, neighbors), 0);
        let neighbors = &visibility_map[1 * g.cols() + 3];
        assert_eq!(count_occupied(&seats, neighbors), 1);
    }

    #[test]
    fn example03() {
        let g = SimpleGrid::create_from("\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.");
        let (seats, visibility_map) = visibility_map(&g);
        let neighbors = &visibility_map[3 * g.cols() + 3];
        assert_eq!(count_occupied(&seats, neighbors), 0);
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
