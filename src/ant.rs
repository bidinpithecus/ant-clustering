use crate::environment::{Cell, Grid};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Carrying,
    NotCarrying,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ant {
    curr_pos: ((usize, usize), Cell),
    curr_state: State,
    view_radius: usize,
}

impl Ant {
    pub fn new(pos: ((usize, usize), Cell), view_radius: usize) -> Self {
        Self {
            curr_pos: pos,
            curr_state: State::NotCarrying,
            view_radius,
        }
    }

    pub fn action(&mut self, grid: &mut Grid, should_pick_after: bool) {
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

        if grid.is_dead_cell(new_pos.0)
            && self.curr_state == State::NotCarrying
            && should_pick_after
        {
            self.pickup_item(grid, new_pos.0);
        }
        if grid.is_empty_cell(new_pos.0) && self.curr_state == State::Carrying {
            self.drop_item(grid, new_pos.0);
        }
        if !grid.is_ant_cell(new_pos.0) {
            self.mov(grid, new_pos.0);
        }
    }

    pub fn get_pos_old_cell(&mut self) -> Cell {
        self.curr_pos.1
    }

    pub fn get_pos(&mut self) -> (usize, usize) {
        self.curr_pos.0
    }

    pub fn mov(&mut self, grid: &mut Grid, new_pos: (usize, usize)) {
        let cell_of_new_pos = grid.get(new_pos);
        let old_cell_of_curr_pos = self.curr_pos.1;

        grid.set_cell(self.curr_pos.0, old_cell_of_curr_pos);
        self.curr_pos.1 = cell_of_new_pos;

        grid.set_cell(new_pos, Cell::Ant);
        self.curr_pos.0 = new_pos;
    }

    pub fn get_state(&mut self) -> State {
        self.curr_state
    }

    pub fn change_state(&mut self, new_state: State) {
        self.curr_state = new_state;
    }

    pub fn pickup_item(&mut self, grid: &mut Grid, pos: (usize, usize)) {
        let random_uniform = thread_rng().sample(Uniform::new(0.0, 1.0));

        let num_of_cells_around = ((2 * self.view_radius + 1).pow(2) - 1) as f64;
        let dead_ants_around = grid.dead_ants_around(pos, self.view_radius) as f64;

        let odds = 1.0 - (dead_ants_around / num_of_cells_around).powi(4);
        if odds >= random_uniform {
            self.change_state(State::Carrying);
            grid.set_cell(pos, Cell::Empty);
        }
    }

    pub fn drop_item(&mut self, grid: &mut Grid, pos: (usize, usize)) {
        let random_uniform = thread_rng().sample(Uniform::new(0.0, 1.0));

        let num_of_cells_around = ((2 * self.view_radius + 1).pow(2) - 1) as f64;
        let dead_ants_around = grid.dead_ants_around(pos, self.view_radius) as f64;

        let odds = (dead_ants_around / num_of_cells_around).powi(4);
        if odds >= random_uniform {
            self.change_state(State::NotCarrying);
            grid.set_cell(pos, Cell::DeadAnt);
        }
    }
}
