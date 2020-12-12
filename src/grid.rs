use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Location {
    pub fn go(&self, direction: Direction) -> Location {
        match direction {
            Direction::Up => Location{ x: self.x, y: self.y - 1 },
            Direction::Down => Location{ x: self.x, y: self.y + 1 },
            Direction::Left => Location{ x: self.x - 1, y: self.y },
            Direction::Right => Location{ x: self.x + 1, y: self.y },
            Direction::UpLeft => Location{ x: self.x - 1, y: self.y - 1 },
            Direction::UpRight => Location{ x: self.x + 1, y: self.y - 1 },
            Direction::DownLeft => Location{ x: self.x - 1, y: self.y + 1 },
            Direction::DownRight => Location{ x: self.x + 1, y: self.y + 1 },
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Location {
    pub x: i64, // steps right from the reference point
    pub y: i64, // steps down from the reference point
}

#[derive(Default)]
pub struct Grid {
    pub symbols: HashMap<Location, char>,
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
