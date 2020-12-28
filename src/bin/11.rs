use advent_of_code_2020::grid::{consts::*, Direction, SimpleGrid};

const DIRECTIONS: [Direction; 8] = [UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Place { Floor, EmptySeat, OccupiedSeat }

// Returns the seats as a vector v together with an adjacency map m:
// v[i] is the state of place i and m[i] is the list of seats adjacent to i.
fn adjacency_map(g: &SimpleGrid) -> (Vec<Place>, Vec<Vec<usize>>) {
    let mut seats = vec![Place::Floor; g.rows() * g.cols()];
    let mut adjacency_map = vec![vec![]; g.rows() * g.cols()];
    for (pos, c) in g.entries_by_location() {
        let i = pos.y as usize * g.cols() + pos.x as usize;
        match c {
            &b'#' => { seats[i] = Place::OccupiedSeat; },
            &b'L' => { seats[i] = Place::EmptySeat; },
            _ => { continue; },
        }
        for d in &DIRECTIONS {
            let dpos = pos.go(d);
            if g.get_by_location(&dpos).filter(|c| **c != b'.').is_some() {
                let di = dpos.y as usize * g.cols() + dpos.x as usize;
                adjacency_map[i].push(di);
            }
        }
    }
    (seats, adjacency_map)
}

// Returns the seats as a vector v together with a 'visibility' map m:
// v[i] is the state of place i and m[i] is the list of seats visible from i.
fn visibility_map(g: &SimpleGrid) -> (Vec<Place>, Vec<Vec<usize>>) {
    let mut seats = vec![Place::Floor; g.rows() * g.cols()];
    let mut visibility_map = vec![vec![]; g.rows() * g.cols()];
    for (pos, c) in g.entries_by_location() {
        let i = pos.y as usize * g.cols() + pos.x as usize;
        match c {
            &b'#' => { seats[i] = Place::OccupiedSeat; },
            &b'L' => { seats[i] = Place::EmptySeat; },
            _ => { continue; },
        }
        for d in &DIRECTIONS {
            let dpos = pos.go_until(d, |p| g.get_by_location(&p) != Some(&b'.'));
            if g.get_by_location(&dpos).is_some() {
                let di = dpos.y as usize * g.cols() + dpos.x as usize;
                visibility_map[i].push(di);
            }
        }
    }
    (seats, visibility_map)
}

fn count_occupied(seats: &[Place], neighbors: &[usize]) -> usize {
    neighbors
        .iter()
        .filter(|i| seats[**i] == Place::OccupiedSeat)
        .count()
}

fn run_simulation(seats: &mut [Place], neighbor_map: &[Vec<usize>], threshold: usize) {
    let mut to_be_checked: Vec<_> = seats.iter().map(|s| *s != Place::Floor).collect();
    let mut to_be_changed = Vec::new();

    loop {
        // check seats and mark them as "to be changed"
        for i in 0..seats.len() {
            if !to_be_checked[i] { continue; }
            if seats[i] == Place::OccupiedSeat {
                if count_occupied(&seats, &neighbor_map[i]) >= threshold {
                    to_be_changed.push((i, Place::EmptySeat));
                }
            } else if seats[i] == Place::EmptySeat {
                if count_occupied(&seats, &neighbor_map[i]) == 0 {
                    to_be_changed.push((i, Place::OccupiedSeat));
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
    let g = SimpleGrid::create_from(input);

    // prepare and run the simulation using the adjacency rules
    let (mut seats, adjacency_map) = adjacency_map(&g);
    run_simulation(&mut seats, &adjacency_map, 4);
    let occupied_adj = seats.iter().filter(|s| **s == Place::OccupiedSeat).count();

    // prepare and run the simulation using the visibility rules
    let (mut seats, visibility_map) = visibility_map(&g);
    run_simulation(&mut seats, &visibility_map, 5);
    let occupied_vis = seats.iter().filter(|s| **s == Place::OccupiedSeat).count();

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
