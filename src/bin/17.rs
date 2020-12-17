use advent_of_code_2020::grid::Grid;
use std::collections::HashMap;
use std::collections::HashSet;

// This struct stores the positions of the active cubes.
struct ActiveCubes(HashSet<(i64, i64, i64, i64)>);

impl ActiveCubes {
    fn create_from(s: &str) -> ActiveCubes {
        let mut cubes = HashSet::new();
        let g = Grid::create_from(s);
        for (l, c) in g.values() {
            if c == &'#' {
                cubes.insert((l.x, l.y, 0, 0));
            }
        }
        ActiveCubes(cubes)
    }

    fn to_string(&self) -> String {
        let x_min = *self.0.iter().map(|(x, _, _, _)| x).min().unwrap();
        let x_max = *self.0.iter().map(|(x, _, _, _)| x).max().unwrap();
        let y_min = *self.0.iter().map(|(_, y, _, _)| y).min().unwrap();
        let y_max = *self.0.iter().map(|(_, y, _, _)| y).max().unwrap();
        let z_min = *self.0.iter().map(|(_, _, z, _)| z).min().unwrap();
        let z_max = *self.0.iter().map(|(_, _, z, _)| z).max().unwrap();
        let w_min = *self.0.iter().map(|(_, _, _, w)| w).min().unwrap();
        let w_max = *self.0.iter().map(|(_, _, _, w)| w).max().unwrap();

        let mut s = String::new();
        for w in w_min..=w_max {
            for z in z_min..=z_max {
                if w_min == 0 && w_max == 0 {
                    s += &format!("z={}\n", z);
                } else {
                    s += &format!("z={}, w={}\n", z, w);
                }
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        if self.0.contains(&(x, y, z, w)) {
                            s.push('#');
                        } else {
                            s.push('.');
                        }
                    }
                    s.push('\n');
                }
                s.push('\n');
            }
        }
        s
    }

    // apply the the rules using a neighborhood radius of (xr, yr, zr, wr)
    fn advance( &mut self, xr: i64, yr: i64, zr: i64, wr: i64 ) {
        let mut count = HashMap::new();

        // Count cubes in the neighbouring region of each cube, including self.
        for &(x, y, z, w) in self.0.iter() {
            for dw in -wr..=wr {
                for dz in -zr..=zr {
                    for dy in -yr..=yr {
                        for dx in -xr..=xr {
                            *count.entry((x+dx, y+dy, z+dz, w+dw)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Update active cubes.
        for (cube, n) in count.into_iter() {
            if self.0.contains(&cube) {
                match n-1 {
                    2 | 3 => {},
                    _ => { self.0.remove(&cube); }
                }
            } else {
                match n {
                    3 => { self.0.insert(cube); },
                    _ => {}
                }
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut cubes3d = ActiveCubes::create_from(input);
    println!("{}", cubes3d.to_string());
    for _ in 0..6 {
        cubes3d.advance(1, 1, 1, 0);
    }

    let mut cubes4d = ActiveCubes::create_from(input);
    for _ in 0..6 {
        cubes4d.advance(1, 1, 1, 1);
    }

    (cubes3d.0.len(), cubes4d.0.len())
}


fn main() {
    let input = std::fs::read_to_string("input/17.txt").unwrap();
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
        let mut cubes = ActiveCubes::create_from(
            "\
.#.
..#
###");
        cubes.advance(1, 1, 1, 0);
        println!("{}", cubes.to_string());
        assert_eq!(
            cubes.to_string().trim(),
            "\
z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.");
        cubes.advance(1, 1, 1, 0);
        println!("{}", cubes.to_string());
        assert_eq!(
            cubes.to_string().trim(),
            "\
z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....");
        cubes.advance(1, 1, 1, 0);
        println!("{}", cubes.to_string());
        assert_eq!(
            cubes.to_string().trim(),
            "\
z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......");
        cubes.advance(1, 1, 1, 0);
        cubes.advance(1, 1, 1, 0);
        cubes.advance(1, 1, 1, 0);
        assert_eq!(cubes.0.len(), 112);
    }

    #[test]
    fn example02() {
        let mut cubes = ActiveCubes::create_from(
            "\
.#.
..#
###");
        cubes.advance(1, 1, 1, 1);
        println!("{}", cubes.to_string());
        assert_eq!(
            cubes.to_string().trim(),
            "\
z=-1, w=-1
#..
..#
.#.

z=0, w=-1
#..
..#
.#.

z=1, w=-1
#..
..#
.#.

z=-1, w=0
#..
..#
.#.

z=0, w=0
#.#
.##
.#.

z=1, w=0
#..
..#
.#.

z=-1, w=1
#..
..#
.#.

z=0, w=1
#..
..#
.#.

z=1, w=1
#..
..#
.#.");
        cubes.advance(1, 1, 1, 1);
        println!("{}", cubes.to_string());
        assert_eq!(
            cubes.to_string().trim(),
            "\
z=-2, w=-2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=0, w=-1
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-1, w=0
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
.....");
        cubes.advance(1, 1, 1, 1);
        cubes.advance(1, 1, 1, 1);
        cubes.advance(1, 1, 1, 1);
        cubes.advance(1, 1, 1, 1);
        assert_eq!(cubes.0.len(), 848);
    }
}
