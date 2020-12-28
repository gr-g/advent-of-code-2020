use advent_of_code_2020::grid::SimpleGrid;

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    tile: SimpleGrid,
}

impl Tile {
    fn rotate(&self) -> Tile {
        let mut t = SimpleGrid::new(self.tile.cols(), self.tile.rows());
        for ((row, col), v) in self.tile.entries() {
            t.set(col, t.cols()-1-row, *v);
        }
        Tile{ id: self.id, tile: t }
    }

    fn transpose(&self) -> Tile {
        let mut t = SimpleGrid::new(self.tile.cols(), self.tile.rows());
        for ((row, col), v) in self.tile.entries() {
            t.set(col, row, *v);
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

    fn matches_border_up(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(0, i) == Some(&border[i]))
    }

    fn matches_border_up_rev(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(0, self.tile.cols()-1-i) == Some(&border[i]))
    }

    fn matches_border_left(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(i, 0) == Some(&border[i]))
    }

    fn matches_border_left_rev(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1-i, 0) == Some(&border[i]))
    }

    fn matches_border_down(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1, i) == Some(&border[i]))
    }

    fn matches_border_down_rev(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1, self.tile.cols()-1-i) == Some(&border[i]))
    }

    fn matches_border_right(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(i, self.tile.cols()-1) == Some(&border[i]))
    }

    fn matches_border_right_rev(&self, border: &[u8]) -> bool {
        (0..border.len()).all(|i| self.tile.get(self.tile.rows()-1-i, self.tile.cols()-1) == Some(&border[i]))
    }

    fn matches_border(&self, border: &[u8]) -> bool {
        self.matches_border_up(border) || self.matches_border_up_rev(border) ||
        self.matches_border_left(border) || self.matches_border_left_rev(border) ||
        self.matches_border_down(border) || self.matches_border_down_rev(border) ||
        self.matches_border_right(border) || self.matches_border_right_rev(border)
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

fn remove_tile_with_pattern_left(tiles: &mut Vec<Tile>, pattern: Vec<u8>) -> Option<Tile> {
    for i in 0..tiles.len() {
        if tiles[i].matches_border_up(&pattern) {
            return Some(tiles.remove(i).transpose());
        }
        if tiles[i].matches_border_up_rev(&pattern) {
            return Some(tiles.remove(i).rotate().rotate().rotate());
        }
        if tiles[i].matches_border_left(&pattern) {
            return Some(tiles.remove(i));
        }
        if tiles[i].matches_border_left_rev(&pattern) {
            return Some(tiles.remove(i).rotate().transpose());
        }
        if tiles[i].matches_border_down(&pattern) {
            return Some(tiles.remove(i).rotate());
        }
        if tiles[i].matches_border_down_rev(&pattern) {
            return Some(tiles.remove(i).transpose().rotate().rotate());
        }
        if tiles[i].matches_border_right(&pattern) {
            return Some(tiles.remove(i).transpose().rotate());
        }
        if tiles[i].matches_border_right_rev(&pattern) {
            return Some(tiles.remove(i).rotate().rotate());
        }
    }
    None
}

fn remove_tile_with_pattern_up(tiles: &mut Vec<Tile>, pattern: Vec<u8>) -> Option<Tile> {
    remove_tile_with_pattern_left(tiles, pattern).map(|t| t.transpose())
}

fn find_corners(tiles: &[Tile]) -> Vec<(usize, Vec<u8>)> {
    let mut corner_patterns = Vec::new();
    for i in 0..tiles.len() {
        // Check whether the four borders appear in other tiles
        let is_shared_border = [
            (0..tiles.len()).any(|j| j != i && tiles[j].matches_border(&tiles[i].border_up())),
            (0..tiles.len()).any(|j| j != i && tiles[j].matches_border(&tiles[i].border_left())),
            (0..tiles.len()).any(|j| j != i && tiles[j].matches_border(&tiles[i].border_down())),
            (0..tiles.len()).any(|j| j != i && tiles[j].matches_border(&tiles[i].border_right())),
        ];

        if !is_shared_border[0] && !is_shared_border[1] {
            corner_patterns.push((tiles[i].id, tiles[i].border_up()));
        }
        if !is_shared_border[1] && !is_shared_border[2] {
            corner_patterns.push((tiles[i].id, tiles[i].rotate().border_up()));
        }
        if !is_shared_border[2] && !is_shared_border[3] {
            corner_patterns.push((tiles[i].id, tiles[i].rotate().rotate().border_up()));
        }
        if !is_shared_border[3] && !is_shared_border[0] {
            corner_patterns.push((tiles[i].id, tiles[i].rotate().rotate().rotate().border_up()));
        }
    }
    corner_patterns
}

fn compose_image(tiles: Vec<Vec<Tile>>) -> SimpleGrid {
    let tile_rows = tiles[0][0].tile.rows();
    let tile_cols = tiles[0][0].tile.cols();
    let rows = tiles.len() * (tile_rows - 2);
    let cols = tiles[0].len() * (tile_cols - 2);

    let mut image = SimpleGrid::new(rows, cols);
    for tr in 0..tiles.len() {
        for tc in 0..tiles[tr].len() {
            for r in 1..tile_rows - 1 {
                for c in 1..tile_cols - 1 {
                    let new_r = tr * (tile_rows - 2) + r - 1;
                    let new_c = tc * (tile_cols - 2) + c - 1;
                    image.set(new_r, new_c, *tiles[tr][tc].tile.get(r, c).unwrap());
                }
            }
        }
    }
    image
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

    let n_tiles = tiles.len();

    let corner_patterns = find_corners(&tiles);
    assert_eq!(corner_patterns.len(), 4, "failed to find conrner tiles");

    let corner_product = corner_patterns.iter().map(|(id, _)| id).product();

    // Choose a pattern as reference to be the top border on the top-left corner
    let mut pattern = corner_patterns[0].1.clone();

    let mut arranged_tiles = Vec::new();

    while let Some(tile) = remove_tile_with_pattern_up(&mut tiles, pattern) {
        pattern = tile.border_right();
        let mut image_row = vec![tile];

        while let Some(tile) = remove_tile_with_pattern_left(&mut tiles, pattern) {
            pattern = tile.border_right();
            image_row.push(tile);
        }

        assert_eq!(image_row.len() * image_row.len(), n_tiles, "failed to reconstruct image");

        pattern = image_row[0].border_down();
        arranged_tiles.push(image_row);
    }

    assert!(tiles.is_empty(), "failed to reconstruct image");

    let image = Tile{ id: 0, tile: compose_image(arranged_tiles) };

    let all_images = vec![
        image.clone(),
        image.rotate(),
        image.rotate().rotate(),
        image.rotate().rotate().rotate(),
        image.transpose(),
        image.transpose().rotate(),
        image.transpose().rotate().rotate(),
        image.transpose().rotate().rotate().rotate(),
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

    (corner_product, oriented_image.tile.values().filter(|c| **c == b'#').count())
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
