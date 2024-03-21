use crate::ant::Ant;
use environment::Cell;
use environment::Grid;
use image::{Rgb, RgbImage};

const NUM_OF_ALIVE_ANTS: usize = 100;
const NUM_OF_DEAD_ANTS: usize = 2000;
const NUM_OF_ITERATIONS: usize = 10_000_000;

mod ant;
mod environment;

fn main() {
    let mut matrix = Grid::new(100, 100);

    matrix.randomly_populate(NUM_OF_DEAD_ANTS, Cell::DeadAnt);
    let mut ants_pos = matrix.randomly_populate(NUM_OF_ALIVE_ANTS, Cell::Ant);
    let mut ants = Vec::with_capacity(NUM_OF_ALIVE_ANTS);

    let initial_state_image = grid_to_image(&mut matrix);
    initial_state_image
        .save("results/initial_state.png")
        .unwrap();

    for i in 0..NUM_OF_ALIVE_ANTS {
        ants.push(Ant::new((ants_pos[i], Cell::Empty), 1));
    }

    for _ in 1..NUM_OF_ITERATIONS {
        for (_, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut matrix, true);
        }
    }

    let mut indices_to_remove = Vec::new();

    while !ants.is_empty() {
        for (i, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut matrix, false);
            if ant.get_state() == ant::State::NotCarrying {
                indices_to_remove.push(i);
                matrix.set_cell(ant.get_pos(), ant.get_pos_old_cell());
            }
        }

        for &index in indices_to_remove.iter().rev() {
            ants.remove(index);
            ants_pos.remove(index);
        }
        indices_to_remove.clear();
    }

    let final_state_image = grid_to_image(&mut matrix);
    final_state_image.save("results/final_state.png").unwrap();
}

fn grid_to_image(grid: &mut Grid) -> RgbImage {
    let width = grid.width() as u32;
    let height = grid.height() as u32;
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cell_color = match grid.get((x as usize, y as usize)) {
                Cell::Empty => Rgb([200, 200, 200]),
                Cell::Ant => Rgb([255, 0, 0]),
                Cell::DeadAnt => Rgb([0, 0, 0]),
            };
            img.put_pixel(x, y, cell_color);
        }
    }

    img
}
