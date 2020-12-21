use advent_of_code_2020::simplegrid::SimpleGrid;

fn trees_on_slope( grid: &SimpleGrid, slope_x: usize, slope_y: usize ) -> usize {
    let cols = grid.cols();
    let (mut x, mut y) = (slope_x, slope_y);
    let mut count = 0;
    while let Some(c) = grid.get(y, x) {
        if c == &b'#' { count += 1; }
        x = (x + slope_x) % cols;
        y = y + slope_y;
    }
    count
}

fn solve(input: &str) -> (usize, usize) {
    let g = SimpleGrid::create_from(input);

    let count_1_1 = trees_on_slope(&g, 1, 1);
    let count_3_1 = trees_on_slope(&g, 3, 1);
    let count_5_1 = trees_on_slope(&g, 5, 1);
    let count_7_1 = trees_on_slope(&g, 7, 1);
    let count_1_2 = trees_on_slope(&g, 1, 2);

    (count_3_1, count_1_1 * count_3_1 * count_5_1 * count_7_1 * count_1_2)
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();
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
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"), (7, 336));
    }
}
