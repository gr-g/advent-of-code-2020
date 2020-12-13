use std::collections::HashMap;
use std::fmt::Display;

pub mod consts {
    pub const ORIGIN: super::Location = super::Location{ x: 0, y: 0 };
    pub const UP: super::Direction = super::Direction{ dx: 0, dy: -1 };
    pub const DOWN: super::Direction = super::Direction{ dx: 0, dy: 1 };
    pub const LEFT: super::Direction = super::Direction{ dx: -1, dy: 0 };
    pub const RIGHT: super::Direction = super::Direction{ dx: 1, dy: 0 };
    pub const UP_LEFT: super::Direction = super::Direction{ dx: -1, dy: -1 };
    pub const UP_RIGHT: super::Direction = super::Direction{ dx: 1, dy: -1 };
    pub const DOWN_LEFT: super::Direction = super::Direction{ dx: -1, dy: 1 };
    pub const DOWN_RIGHT: super::Direction = super::Direction{ dx: 1, dy: 1 };
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Location {
    pub x: i64, // steps right from the reference point
    pub y: i64, // steps down from the reference point
}

impl Location {
    pub fn go(&self, dir: &Direction) -> Location {
        Location{ x: self.x + dir.dx, y: self.y + dir.dy }
    }

    pub fn go_until<F>(&self, dir: &Direction, f: F) -> Option<Location> where
        F: Fn(&Location) -> bool {
        (1..).find_map(|n| {
            let pos = self.go(&dir.times(n));
            if f(&pos) {
                Some(pos)
            } else {
                None
            }
        })
    }

    pub fn distance(&self, other: &Location) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction {
    pub dx: i64,
    pub dy: i64,
}

impl Direction {
    pub fn add(&self, other: &Direction) -> Direction {
        Direction{ dx: self.dx + other.dx, dy: self.dy + other.dy }
    }

    pub fn rotate_left(&self) -> Direction {
        Direction{ dx: self.dy, dy: -self.dx }
    }

    pub fn rotate_right(&self) -> Direction {
        Direction{ dx: -self.dy, dy: self.dx }
    }

    pub fn reverse(&self) -> Direction {
        Direction{ dx: -self.dx, dy: -self.dy }
    }

    pub fn times(&self, n: i64) -> Direction {
        Direction{ dx: self.dx * n, dy: self.dy * n }
    }
}

#[derive(Default)]
pub struct Grid {
    symbols: HashMap<Location, char>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid { symbols: HashMap::new() }
    }
    pub fn create_from(s: &str) -> Grid {
        let mut symbols = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        for c in s.chars() {
            match c {
                '\n' => { y += 1; x = 0; },
                c => {
                    if !c.is_ascii_whitespace() {
                        symbols.insert(Location { x, y }, c);
                    }
                    x += 1;
                }
            }
        }
        Grid { symbols }
    }

    pub fn x_min( &self ) -> i64 {
        *self.symbols.keys().map(|Location { x, .. }| x).min().unwrap_or(&0)
    }

    pub fn x_max( &self ) -> i64 {
        *self.symbols.keys().map(|Location { x, .. }| x).max().unwrap_or(&0)
    }

    pub fn y_min( &self ) -> i64 {
        *self.symbols.keys().map(|Location { y, .. }| y).min().unwrap_or(&0)
    }

    pub fn y_max( &self ) -> i64 {
        *self.symbols.keys().map(|Location { y, .. }| y).max().unwrap_or(&0)
    }

    pub fn get(&self, l: &Location) -> Option<&char> {
        self.symbols.get(l)
    }

    pub fn insert(&mut self, l: Location, c: char) -> Option<char> {
        self.symbols.insert(l, c)
    }

    pub fn remove(&mut self, l: &Location) -> Option<char> {
        self.symbols.remove(l)
    }

    pub fn find( &self, c: char ) -> Option<&Location> {
        self.symbols.iter().find(|(_, sym)| **sym == c ).map(|(loc, _)| loc)
    }

    pub fn values(&self) -> impl Iterator<Item = (Location, &char)> {
        self.symbols.iter().map(|(l, c)| (*l, c))
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = (Location, &mut char)> {
        self.symbols.iter_mut().map(|(l, c)| (*l, c))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x0 = self.x_min();
        let x1 = self.x_max();
        let y0 = self.y_min();
        let y1 = self.y_max();

        for y in y0..=y1 {
            for x in x0..=x1 {
                write!(f, "{}", self.get(&Location { x, y }).unwrap_or(&' '))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct FixedGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl FixedGrid {
    pub fn create_from(s: &str) -> FixedGrid {
        let cols = s.find('\n').unwrap();
        let mut rows = 0;
        let mut data = Vec::new();

        for line in s.trim().split('\n') {
            rows += 1;
            data.extend_from_slice(line.as_bytes());
            assert_eq!(data.len(), rows*cols, "input lines have different lengths");
        }

        FixedGrid{ rows, cols, data }
    }

    pub fn location_to_index(&self, l: &Location) -> usize {
        l.y as usize * self.cols + l.x as usize
    }

    pub fn index_to_location(&self, i: usize) -> Location {
        Location{ x: (i % self.cols) as i64, y: (i / self.cols) as i64 }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get(&self, l: &Location) -> Option<&u8> {
        if l.x < 0 || l.x as usize >= self.cols { return None; }
        if l.y < 0 || l.y as usize >= self.rows { return None; }

        self.data.get(self.location_to_index(l))
    }

    pub fn set(&mut self, i: usize, c: u8) {
        self.data[i] = c;
    }
}

impl Display for FixedGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{}", self.data[y * self.cols + x] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
