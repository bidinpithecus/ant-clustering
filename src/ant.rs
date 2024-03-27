use crate::data::Data;
use crate::environment::{Cell, Grid};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Ant {
    curr_pos: ((usize, usize), Cell),
    view_radius: usize,
    carrying: Option<Data>,
}

impl Ant {
    pub fn new(pos: ((usize, usize), Cell), view_radius: usize, carrying: Option<Data>) -> Self {
        Self {
            curr_pos: pos,
            view_radius,
            carrying,
        }
    }

    pub fn action(
        &mut self,
        grid: &mut Grid,
        should_pick_after: bool,
        k1: f64,
        k2: f64,
        alpha: f64,
    ) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen::<usize>() % 8;
        let mut new_pos = (
            (self.curr_pos.0 .0 as isize, self.curr_pos.0 .1 as isize),
            self.curr_pos.1,
        );
        let width = grid.width();
        let height = grid.height();

        match direction {
            0 => new_pos.0 .0 -= 1,
            1 => new_pos.0 .0 += 1,
            2 => new_pos.0 .1 -= 1,
            3 => new_pos.0 .1 += 1,
            4 => {
                new_pos.0 .0 -= 1;
                new_pos.0 .1 -= 1
            }
            5 => {
                new_pos.0 .0 -= 1;
                new_pos.0 .1 += 1
            }
            6 => {
                new_pos.0 .0 += 1;
                new_pos.0 .1 -= 1
            }
            7 => {
                new_pos.0 .0 += 1;
                new_pos.0 .1 += 1
            }
            _ => panic!("Invalid direction"),
        }

        if new_pos.0 .0 < 0 {
            new_pos.0 .0 = width as isize - 1;
        } else if new_pos.0 .0 == width as isize {
            new_pos.0 .0 = 0;
        }

        if new_pos.0 .1 < 0 {
            new_pos.0 .1 = height as isize - 1;
        } else if new_pos.0 .1 == height as isize {
            new_pos.0 .1 = 0;
        }

        let new_pos = ((new_pos.0 .0 as usize, new_pos.0 .1 as usize), new_pos.1);

        let mut data_under_ant = None;

        if grid.is_data_cell(((new_pos.0 .0 as isize), (new_pos.0 .1 as isize))) {
            data_under_ant =
                grid.get_data_from_data_cell(((new_pos.0 .0 as isize), (new_pos.0 .1 as isize)));
        }

        if self.carrying.is_none() && should_pick_after && data_under_ant.is_some() {
            self.pickup_item(grid, new_pos.0, data_under_ant.unwrap(), k2, alpha);
        } else if self.carrying().is_some()
            && grid.is_empty_cell(((new_pos.0 .0 as isize), (new_pos.0 .1 as isize)))
        {
            let mut carried_data = self.carrying.unwrap();
            carried_data.set_pos(new_pos.0);
            self.carrying = Some(carried_data);
            self.drop_item(grid, k1, alpha);
        }

        if !grid.is_ant_cell(((new_pos.0 .0 as isize), (new_pos.0 .1 as isize))) {
            self.walk(grid, new_pos.0);
        }
    }

    pub fn get_pos_old_cell(&mut self) -> Cell {
        self.curr_pos.1
    }

    pub fn get_pos(&mut self) -> (usize, usize) {
        self.curr_pos.0
    }

    pub fn carrying(&mut self) -> Option<Data> {
        self.carrying
    }

    pub fn carry(&mut self, item: Data) {
        self.carrying = Some(item);
    }

    pub fn drop(&mut self) {
        self.carrying = None
    }

    pub fn walk(&mut self, grid: &mut Grid, new_pos: (usize, usize)) {
        let cell_of_new_pos = grid.get((new_pos.0 as isize, new_pos.1 as isize));
        let old_cell_of_curr_pos = self.curr_pos.1;

        grid.set_cell(
            (self.curr_pos.0 .0 as isize, self.curr_pos.0 .1 as isize),
            old_cell_of_curr_pos,
        );
        self.curr_pos.1 = cell_of_new_pos;

        let ant = Cell::Ant {
            carrying: self.carrying,
        };

        grid.set_cell((new_pos.0 as isize, new_pos.1 as isize), ant);
        self.curr_pos.0 = new_pos;
    }

    pub fn pickup_item(
        &mut self,
        grid: &mut Grid,
        pos: (usize, usize),
        data_under_ant: Data,
        k1: f64,
        alpha: f64,
    ) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.0, 1.0);
        let random = rng.sample(uniform);

        let fx = grid.data_around(data_under_ant, self.view_radius, alpha);
        let odds = (k1 / (k1 + fx)).powi(2);

        if odds > random {
            self.carry(data_under_ant);
            grid.set_cell((pos.0 as isize, pos.1 as isize), Cell::Empty);
            return true;
        }

        false
    }

    pub fn drop_item(&mut self, grid: &mut Grid, k2: f64, alpha: f64) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0.0, 1.0);
        let random = rng.sample(uniform);

        let fx = grid.data_around(self.carrying.unwrap(), self.view_radius, alpha);
        let odds = (fx / (k2 + fx)).powi(2);

        if odds > random {
            let carried_data_pos = self.carrying.unwrap().pos();
            grid.set_cell(
                (carried_data_pos.0 as isize, carried_data_pos.1 as isize),
                Cell::Data {
                    content: self.carrying.unwrap(),
                },
            );
            self.drop();
            return true;
        }

        false
    }
}
