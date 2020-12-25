use std::collections::HashMap;
use std::collections::HashSet;

fn coordinates(directions: &str) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut chars = directions.chars();
    while let Some(c) = chars.next() {
        // Map the six directions to three vectors (v1, v2, v3) and their
        // opposites (-v1, -v2, -v3). The mapping is arbitrary: any mapping
        // is suitable provided that v1, v2, v3 are chosen so that
        // v1 + v2 + v3 = 0.
        match c {
            'e' => { x += 2; },
            'w' => { x -= 2; },
            'n' => {
                match chars.next() {
                    Some('e') => { x += 1; y -= 1; },
                    Some('w') => { x -= 1; y -= 1; },
                    _ => { panic!("invalid directions"); },
                }
            },
            's' => {
                match chars.next() {
                    Some('e') => { x += 1; y += 1; },
                    Some('w') => { x -= 1; y += 1; },
                     _ => { panic!("invalid directions"); },
                }
            },
            _ => { panic!("invalid directions"); },
        }
    }
    (x, y)
}

fn solve(input: &str) -> (usize, usize) {
    let mut black_tiles = HashSet::new();

    for pos in input.lines().map(|line| coordinates(line)) {
        if !black_tiles.insert(pos) {
            black_tiles.remove(&pos);
        }
    }
    let n0 = black_tiles.len();

    let mut count = HashMap::new();
    for _ in 0..100 {
        for (x, y) in black_tiles.iter().copied() {
            count.entry((x, y)).or_insert(0);
            *count.entry((x+2, y)).or_insert(0) += 1;
            *count.entry((x+1, y-1)).or_insert(0) += 1;
            *count.entry((x-1, y-1)).or_insert(0) += 1;
            *count.entry((x-2, y)).or_insert(0) += 1;
            *count.entry((x-1, y+1)).or_insert(0) += 1;
            *count.entry((x+1, y+1)).or_insert(0) += 1;
        }

        for ((x, y), n) in count.drain() {
            if black_tiles.contains(&(x, y)) {
                match n {
                    0 | 3 | 4 | 5 | 6 => { black_tiles.remove(&(x, y)); },
                    _ => {},
                }
            } else {
                match n {
                    2 => { black_tiles.insert((x, y)); },
                    _ => {}
                }
            }
        }
    }
    let n100 = black_tiles.len();

    (n0, n100)
}

fn main() {
    let input = std::fs::read_to_string("input/24.txt").unwrap();
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
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"), (10, 2208));
    }
}
