use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct SimpleGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl SimpleGrid {
    pub fn new(rows: usize, cols: usize) -> SimpleGrid {
        SimpleGrid { rows, cols, data: vec![0; rows*cols] }
    }

    pub fn create_from(s: &str) -> SimpleGrid {
        let cols = s.find('\n').unwrap();
        let mut rows = 0;
        let mut data = Vec::new();

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
        self.data.get(row * self.cols + col)
    }

    pub fn set(&mut self, row: usize, col: usize, v: u8) {
        self.data[row * self.cols + col] = v;
    }

    pub fn values(&self) -> impl Iterator<Item = ((usize, usize), &u8)> {
        (0..self.rows).flat_map(move |row| {
            (0..self.cols).map(move |col| {
                ((row, col), self.data.get(row * self.cols + col).unwrap())
            })
        })
    }
}

impl Display for SimpleGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
