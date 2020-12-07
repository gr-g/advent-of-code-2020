use advent_of_code_2020::grid::{Grid, Location};

fn trees_on_slope( grid: &Grid, slope_x: i64, slope_y: i64 ) -> i64 {
    let (x_len, y_len) = (grid.x_max()+1, grid.y_max()+1);
    let (mut x, mut y) = (slope_x, slope_y);
    let mut count = 0;
    while y < y_len {
        if grid.symbols.get(&Location{ x, y }) == Some(&'#') {
            count += 1;
        }
        x = (x + slope_x) % x_len;
        y = y + slope_y;
    }
    count
}

fn solve(input: &str) -> (i64, i64) {
    let g = Grid::create_from(input);
    
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
