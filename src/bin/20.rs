use advent_of_code_2020::simplegrid::SimpleGrid;

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    tile: SimpleGrid,
}

impl Tile {
    fn rotate(&self) -> Tile {
        let mut t = SimpleGrid::new(self.tile.cols(), self.tile.rows());
        for ((row, col), v) in self.tile.values() {
            t.set(col, t.cols()-1-row, *v);
        }
        Tile{ id: self.id, tile: t }
    }

    fn flip(&self) -> Tile {
        let mut t = SimpleGrid::new(self.tile.rows(), self.tile.cols());
        for ((row, col), v) in self.tile.values() {
            t.set(row, t.cols()-1-col, *v);
        }
        Tile{ id: self.id, tile: t }
    }

    fn border_up(&self) -> Vec<u8> {
        (0..self.tile.cols()).map(|col| *self.tile.get(0, col).unwrap()).collect()
    }

    fn border_left(&self) -> Vec<u8> {
        (0..self.tile.rows()).map(|row| *self.tile.get(row, 0).unwrap()).collect()
    }

    fn border_down(&self) -> Vec<u8> {
        (0..self.tile.cols()).map(|col| *self.tile.get(self.tile.rows()-1, col).unwrap()).collect()
    }

    fn border_right(&self) -> Vec<u8> {
        (0..self.tile.rows()).map(|row| *self.tile.get(row, self.tile.cols()-1).unwrap()).collect()
    }

    fn has_border(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(0, i) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(i, 0) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1, i) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(i, self.tile.cols()-1) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(0, self.tile.cols()-1-i) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1-i, 0) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1, self.tile.cols()-1-i) == Some(&border[i])) ||
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1-i, self.tile.cols()-1) == Some(&border[i]))
    }

    fn attach_right(&self, other: &Tile) -> Tile {
        assert!(self.tile.rows() == other.tile.rows());
        let mut t = SimpleGrid::new(self.tile.rows(), self.tile.cols()+other.tile.cols());
        for ((row, col), v) in self.tile.values() {
            t.set(row, col, *v);
        }
        for ((row, col), v) in other.tile.values() {
            t.set(row, self.tile.cols()+col, *v);
        }
        Tile{ id: self.id, tile: t }
    }

    fn attach_down(&self, other: &Tile) -> Tile {
        assert!(self.tile.cols() == other.tile.cols());
        let mut t = SimpleGrid::new(self.tile.rows()+other.tile.rows(), self.tile.cols());
        for ((row, col), v) in self.tile.values() {
            t.set(row, col, *v);
        }
        for ((row, col), v) in other.tile.values() {
            t.set(self.tile.rows()+row, col, *v);
        }
        Tile{ id: self.id, tile: t }
    }

    fn remove_borders(&self, border_len: usize) -> Tile {
        let new_rows = self.tile.rows() / border_len * (border_len - 2);
        let new_cols = self.tile.cols() / border_len * (border_len - 2);

        let mut t = SimpleGrid::new(new_rows, new_cols);

        let mut new_row = 0;
        let mut new_col = 0;
        for row in 0..self.tile.rows() {
            if row % border_len == 0 || row % border_len == border_len - 1 {
                continue;
            }
            for col in 0..self.tile.cols() {
                if col % border_len == 0 || col % border_len == border_len - 1 {
                    continue;
                }
                t.set(new_row, new_col, *self.tile.get(row, col).unwrap());
                new_col += 1;
            }
            new_row += 1;
            new_col = 0;
        }
        Tile{ id: self.id, tile: t }
    }

    fn highlight_pattern(&mut self, offset_row: usize, offset_col: usize, pattern: &SimpleGrid) -> bool {
        for row in 0..pattern.rows() {
            for col in 0..pattern.cols() {
                if pattern.get(row, col) != Some(&b'.') && self.tile.get(offset_row+row, offset_col+col) != Some(&b'#') {
                    return false;
                }
            }
        }
        // Pattern found!
        for row in 0..pattern.rows() {
            for col in 0..pattern.cols() {
                if pattern.get(row, col) != Some(&b'.') {
                    self.tile.set(offset_row+row, offset_col+col, *pattern.get(row, col).unwrap());
                }
            }
        }
        true
    }
}

fn remove_tile_with_pattern_up(tiles: &mut Vec<Tile>, pattern: &[u8]) -> Option<Tile> {
    for i in 0..tiles.len() {
        if pattern == tiles[i].border_up().as_slice() {
            return Some(tiles.remove(i));
        }
        if pattern == tiles[i].flip().border_up().as_slice() {
            return Some(tiles.remove(i).flip());
        }
        if pattern == tiles[i].rotate().border_up().as_slice() {
            return Some(tiles.remove(i).rotate());
        }
        if pattern == tiles[i].rotate().flip().border_up().as_slice() {
            return Some(tiles.remove(i).rotate().flip());
        }
        if pattern == tiles[i].rotate().rotate().border_up().as_slice() {
            return Some(tiles.remove(i).rotate().rotate());
        }
        if pattern == tiles[i].rotate().rotate().flip().border_up().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().flip());
        }
        if pattern == tiles[i].rotate().rotate().rotate().border_up().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().rotate());
        }
        if pattern == tiles[i].rotate().rotate().rotate().flip().border_up().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().rotate().flip());
        }
    }
    None
}

fn remove_tile_with_pattern_left(tiles: &mut Vec<Tile>, pattern: &[u8]) -> Option<Tile> {
    for i in 0..tiles.len() {
        if pattern == tiles[i].border_left().as_slice() {
            return Some(tiles.remove(i));
        }
        if pattern == tiles[i].flip().border_left().as_slice() {
            return Some(tiles.remove(i).flip());
        }
        if pattern == tiles[i].rotate().border_left().as_slice() {
            return Some(tiles.remove(i).rotate());
        }
        if pattern == tiles[i].rotate().flip().border_left().as_slice() {
            return Some(tiles.remove(i).rotate().flip());
        }
        if pattern == tiles[i].rotate().rotate().border_left().as_slice() {
            return Some(tiles.remove(i).rotate().rotate());
        }
        if pattern == tiles[i].rotate().rotate().flip().border_left().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().flip());
        }
        if pattern == tiles[i].rotate().rotate().rotate().border_left().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().rotate());
        }
        if pattern == tiles[i].rotate().rotate().rotate().flip().border_left().as_slice() {
            return Some(tiles.remove(i).rotate().rotate().rotate().flip());
        }
    }
    None
}

fn find_corners(tiles: &[Tile]) -> Vec<(usize, Vec<u8>)> {
    let mut corner_patterns = Vec::new();
    for i in 0..tiles.len() {
        // Check which of the four borders appear in other tiles
        let is_shared_border = [
            (0..tiles.len()).any(|j| j != i && tiles[j].has_border(&tiles[i].border_up())),
            (0..tiles.len()).any(|j| j != i && tiles[j].has_border(&tiles[i].border_left())),
            (0..tiles.len()).any(|j| j != i && tiles[j].has_border(&tiles[i].border_down())),
            (0..tiles.len()).any(|j| j != i && tiles[j].has_border(&tiles[i].border_right())),
        ];

        if !is_shared_border[0] && !is_shared_border[1] {
            //println!("tile {} is a corner", tiles[i].id);
            corner_patterns.push((tiles[i].id, tiles[i].border_up()));
        }
        if !is_shared_border[1] && !is_shared_border[2] {
            //println!("tile {} is a corner", tiles[i].id);
            corner_patterns.push((tiles[i].id, tiles[i].rotate().border_up()));
        }
        if !is_shared_border[2] && !is_shared_border[3] {
            //println!("tile {} is a corner", tiles[i].id);
            corner_patterns.push((tiles[i].id, tiles[i].rotate().rotate().border_up()));
        }
        if !is_shared_border[3] && !is_shared_border[0] {
            //println!("tile {} is a corner", tiles[i].id);
            corner_patterns.push((tiles[i].id, tiles[i].rotate().rotate().rotate().border_up()));
        }
    }
    corner_patterns
}

fn solve(input: &str) -> (usize, usize) {
    let tile_str = input.trim().split("\n\n");
    let mut tiles = Vec::new();

    for t in tile_str {
        let mut parts = t.splitn(2, '\n');
        let id = parts.next().unwrap()[5..9].parse::<usize>().unwrap();
        let tile = SimpleGrid::create_from(parts.next().unwrap());

        tiles.push(Tile{ id, tile });
    }

    let corner_patterns = find_corners(&tiles);
    assert_eq!(corner_patterns.len(), 4, "failed to find conrner tiles");

    let corner_product = corner_patterns.iter().map(|(id, _)| id).product();

    // Choose a pattern as reference to be the top-left corner
    let mut pattern = corner_patterns[0].1.clone();

    let mut image_rows = Vec::new();

    while let Some(mut image_row) = remove_tile_with_pattern_up(&mut tiles, &pattern) {
        pattern = image_row.border_right();

        while let Some(tile) = remove_tile_with_pattern_left(&mut tiles, &pattern) {
            image_row = image_row.attach_right(&tile);
            pattern = image_row.border_right();
        }

        pattern = image_row.border_down();
        pattern.truncate(10);

        //println!("image row:\n{}", image_row.tile);
        image_rows.push(image_row);
        //println!("tiles left: {}", tiles.len());
    }

    assert!(tiles.is_empty(), "failed to reconstruct image");

    let mut image = image_rows.remove(0);
    for i in image_rows {
        image = image.attach_down(&i);
    }
    //println!("{}", image.tile);

    image = image.remove_borders(10);

    let all_images = vec![
        image.clone(),
        image.flip(),
        image.rotate(),
        image.rotate().flip(),
        image.rotate().rotate(),
        image.rotate().rotate().flip(),
        image.rotate().rotate().rotate(),
        image.rotate().rotate().rotate().flip(),
    ];

    let monster = SimpleGrid::create_from("\
..................O.
O....OO....OO....OOO
.O..O..O..O..O..O...");

    let (oriented_image, _) = all_images.into_iter()
        .map(|mut i| {
            let mut monsters = 0;
            for row in 0..i.tile.rows()-monster.rows()+1 {
                for col in 0..i.tile.cols()-monster.cols()+1 {
                    if i.highlight_pattern(row, col, &monster) {
                        //println!("found monster at row {}, col {}", row, col);
                        monsters += 1;
                    }
                }
            }
            (i, monsters)
        })
        .find(|(_, n)| *n > 0)
        .expect("no monsters found");

    println!("{}", oriented_image.tile);

    (corner_product, oriented_image.tile.values().filter(|(_, c)| **c == b'#').count())
}

fn main() {
    let input = std::fs::read_to_string("input/20.txt").unwrap();
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
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."), (20899048083289, 273));
    }
}
