use rand::{self, Rng};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Data,
    Ant,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    m: Vec<Vec<Cell>>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Data => write!(f, "D"),
            Cell::Ant => write!(f, "A"),
            Cell::Empty => write!(f, " "),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.m {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(num_rows: usize, num_cols: usize) -> Grid {
        let m = vec![vec![Cell::Empty; num_cols]; num_rows];
        Grid {
            num_rows,
            num_cols,
            m,
        }
    }

    pub fn width(&mut self) -> usize {
        self.num_cols
    }

    pub fn height(&mut self) -> usize {
        self.num_rows
    }

    pub fn get(&mut self, cell: (usize, usize)) -> Cell {
        let cell = (cell.0 % self.num_rows, cell.1 % self.num_cols);
        self.m[cell.0][cell.1]
    }

    pub fn randomly_populate(&mut self, num_items: usize, item: Cell) -> Vec<(usize, usize)> {
        assert!(self.num_rows * self.num_cols > num_items);
        let mut inserted = false;
        let mut rng = rand::thread_rng();
        let mut positions: Vec<(usize, usize)> = Vec::with_capacity(num_items);

        for _ in 0..num_items {
            while !inserted {
                let row = rng.gen::<usize>() % self.num_rows;
                let col = rng.gen::<usize>() % self.num_cols;
                if self.m[row][col] == Cell::Empty {
                    self.m[row][col] = item;
                    positions.push((row, col));
                    inserted = true;
                }
            }
            inserted = false;
        }
        positions
    }

    pub fn is_data_cell(&mut self, cell: (usize, usize)) -> bool {
        let cell = (cell.0 % self.num_rows, cell.1 % self.num_cols);
        self.m[cell.0][cell.1] == Cell::Data
    }

    pub fn is_empty_cell(&mut self, cell: (usize, usize)) -> bool {
        let cell = (cell.0 % self.num_rows, cell.1 % self.num_cols);
        self.m[cell.0][cell.1] == Cell::Empty
    }

    pub fn is_ant_cell(&mut self, cell: (usize, usize)) -> bool {
        let cell = (cell.0 % self.num_rows, cell.1 % self.num_cols);
        self.m[cell.0][cell.1] == Cell::Ant
    }

    pub fn set_cell(&mut self, cell: (usize, usize), new_state: Cell) {
        let cell = (cell.0 % self.num_rows, cell.1 % self.num_cols);
        self.m[cell.0][cell.1] = new_state;
    }

    pub fn data_around(&mut self, cell: (usize, usize), view_radius: usize) -> u8 {
        let (mut x, mut y) = cell;
        let mut num_of_ants = 0;

        let width = self.num_cols;
        let height = self.num_rows;

        if x == 0 {
            x = width - 1;
        } else {
            x -= 1;
        }

        if y == 0 {
            y = height - 1;
        } else {
            y -= 1;
        }

        let view_radius = view_radius * 3;

        for i in 0..view_radius {
            for j in 0..view_radius {
                if i == 1 && j == 1 {
                    continue;
                }

                let nx = (x + i) % width;
                let ny = (y + j) % height;

                if self.is_data_cell((nx, ny)) {
                    num_of_ants += 1;
                }
            }
        }

        num_of_ants
    }
}
