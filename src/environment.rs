use rand::{self, Rng};
use std::fmt;

use crate::data::{self, Data};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cell {
    Data { content: data::Data },
    Ant { carrying: Option<data::Data> },
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
            Cell::Data { .. } => write!(f, "D"),
            Cell::Ant { carrying } => {
                if carrying.is_none() {
                    write!(f, "A")
                } else {
                    write!(f, "a")
                }
            }
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

    pub fn get(&mut self, cell: (isize, isize)) -> Cell {
        let cell = self.safe_cell(cell);
        self.m[cell.0][cell.1]
    }

    pub fn random_empty_cell(&mut self) -> (usize, usize) {
        let mut inserted = false;
        let mut rng = rand::thread_rng();
        let mut row: usize = 0;
        let mut col: usize = 0;

        while !inserted {
            row = rng.gen::<usize>() % self.num_rows;
            col = rng.gen::<usize>() % self.num_cols;
            if self.m[row][col] == Cell::Empty {
                inserted = true;
            }
        }
        (row, col)
    }

    pub fn is_data_cell(&mut self, cell: (isize, isize)) -> bool {
        let cell = self.safe_cell(cell);
        match self.m[cell.0][cell.1] {
            Cell::Data { .. } => true,
            Cell::Ant { .. } => false,
            Cell::Empty => false,
        }
    }

    pub fn get_data_from_data_cell(&mut self, cell: (isize, isize)) -> Option<data::Data> {
        let cell = self.safe_cell(cell);
        match self.m[cell.0][cell.1] {
            Cell::Data { content } => Some(content),
            Cell::Ant { carrying } => carrying,
            Cell::Empty => None,
        }
    }

    pub fn is_empty_cell(&mut self, cell: (isize, isize)) -> bool {
        let cell = self.safe_cell(cell);
        self.m[cell.0][cell.1] == Cell::Empty
    }

    pub fn is_ant_cell(&mut self, cell: (isize, isize)) -> bool {
        let cell = self.safe_cell(cell);
        match self.m[cell.0][cell.1] {
            Cell::Data { .. } => false,
            Cell::Ant { .. } => true,
            Cell::Empty => false,
        }
    }

    pub fn set_cell(&mut self, cell: (isize, isize), new_state: Cell) {
        let cell = self.safe_cell(cell);
        self.m[cell.0][cell.1] = new_state;
    }

    fn safe_cell(&mut self, cell: (isize, isize)) -> (usize, usize) {
        let mut x = if cell.0 < 0 {
            self.width() - 1
        } else if cell.0 >= self.width() as isize {
            (cell.0 as usize) % self.width()
        } else {
            cell.0 as usize
        };

        let mut y = if cell.1 < 0 {
            self.height() - 1
        } else if cell.1 >= self.height() as isize {
            (cell.1 as usize) % self.height()
        } else {
            cell.1 as usize
        };

        // Ensure x and y are within bounds
        if x >= self.width() {
            x = self.width() - 1;
        }
        if y >= self.height() {
            y = self.height() - 1;
        }

        (x, y)
    }

    pub fn data_around(&mut self, data: Data, view_radius: usize, alpha: f64) -> f64 {
        let (x, y) = (data.pos().0 as isize, data.pos().1 as isize);
        let mut similarity = 0.0;

        let view_radius = view_radius as isize + 1;

        for dx in -view_radius as isize..view_radius as isize {
            for dy in -view_radius as isize..view_radius as isize {
                if dx == 0 && dy == 0 {
                    continue; // Skip the center cell
                }
                let (nx, ny) = (x + dx, y + dy);
                if self.is_data_cell((nx, ny)) {
                    if let Some(data_next_cell) = self.get_data_from_data_cell((nx, ny)) {
                        let dij = data.euclidian_distance(data_next_cell);
                        similarity += 1.0 - dij / alpha;
                    }
                }
            }
        }

        if similarity <= 0.0 {
            return 0.0;
        }

        similarity / 8.0 // Divided by 8 since there are 8 neighbors (excluding diagonals)
    }
}
