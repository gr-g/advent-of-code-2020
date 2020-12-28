use std::fmt::Display;

pub mod consts {
    use super::{Location, Direction};
    pub const ORIGIN: Location = Location{ x: 0, y: 0 };
    pub const UP: Direction = Direction{ dx: 0, dy: -1 };
    pub const DOWN: Direction = Direction{ dx: 0, dy: 1 };
    pub const LEFT: Direction = Direction{ dx: -1, dy: 0 };
    pub const RIGHT: Direction = Direction{ dx: 1, dy: 0 };
    pub const UP_LEFT: Direction = Direction{ dx: -1, dy: -1 };
    pub const UP_RIGHT: Direction = Direction{ dx: 1, dy: -1 };
    pub const DOWN_LEFT: Direction = Direction{ dx: -1, dy: 1 };
    pub const DOWN_RIGHT: Direction = Direction{ dx: 1, dy: 1 };
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

    pub fn go_until<F>(&self, dir: &Direction, f: F) -> Location where
        F: Fn(&Location) -> bool {
        (1..).find_map(|n| {
            let pos = self.go(&dir.times(n));
            if f(&pos) {
                Some(pos)
            } else {
                None
            }
        }).unwrap()
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

// A fixed-size grid of bytes, with values accessed by row/column
// with get()/set(), or by Location with get_by_location().
#[derive(Clone, Debug)]
pub struct SimpleGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl SimpleGrid {
    pub fn new(rows: usize, cols: usize) -> SimpleGrid {
        SimpleGrid{ rows, cols, data: vec![0; rows*cols] }
    }

    pub fn create_from(s: &str) -> SimpleGrid {
        let cols = s.find('\n').unwrap();
        let mut rows = 0;
        let mut data = Vec::with_capacity(s.len());

        for line in s.lines() {
            rows += 1;
            data.extend_from_slice(line.as_bytes());
            assert_eq!(data.len(), rows*cols, "input lines have different lengths");
        }

        SimpleGrid{ rows, cols, data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&u8> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, v: u8) {
        assert!(row < self.rows && col < self.cols);
        self.data[row * self.cols + col] = v;
    }

    pub fn entries(&self) -> impl Iterator<Item = ((usize, usize), &u8)> {
        (0..self.rows).flat_map(move |row| {
            (0..self.cols).filter_map(move |col| {
                self.data.get(row * self.cols + col).map(|c| ((row, col), c))
            })
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &u8> {
        (0..self.rows).flat_map(move |row| {
            (0..self.cols).filter_map(move |col| {
                self.data.get(row * self.cols + col)
            })
        })
    }

    pub fn get_by_location(&self, l: &Location) -> Option<&u8> {
        if l.x >= 0 && (l.x as usize) < self.cols && l.y >= 0 && (l.y as usize) < self.rows {
            self.data.get(l.y as usize * self.cols + l.x as usize)
        } else {
            None
        }
    }

    pub fn entries_by_location(&self) -> impl Iterator<Item = (Location, &u8)> {
        (0..self.rows).flat_map(move |row| {
            (0..self.cols).filter_map(move |col| {
                self.data.get(row * self.cols + col).map(|c| {
                    (Location{ x: col as i64, y: row as i64 }, c)
                })
            })
        })
    }
}

impl Display for SimpleGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = *self.get(row, col).filter(|v| v.is_ascii_graphic()).unwrap_or(&b' ') as char;
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
