use crate::ant::Ant;
use environment::Cell;
use environment::Grid;
use image::{Rgb, RgbImage};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

const EMPTY_CELL_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
const DEAD_ANT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);
const ALIVE_ANT_COLOR: Rgb<u8> = Rgb([169,99,49]);

mod ant;
mod environment;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let (dir_name, num_of_rows, num_of_cols, num_of_alive_ants, num_of_dead_ants, num_of_iterations, ant_vision) = if args.len() == 8 {
        let dir_name: String = args[1].clone();
        let ant_vision: usize = args[2].parse()?;
        let num_of_rows: usize = args[3].parse()?;
        let num_of_cols: usize = args[4].parse()?;
        let num_of_alive_ants: usize = args[5].parse()?;
        let num_of_dead_ants: usize = args[6].parse()?;
        let num_of_iterations: usize = args[7].parse()?;
        (dir_name, num_of_rows, num_of_cols, num_of_alive_ants, num_of_dead_ants, num_of_iterations, ant_vision)
    } else if args.len() == 1 {
        let dir_name = String::from("results");
        let ant_vision = 1;
        let num_of_alive_ants: usize = 150;
        let num_of_dead_ants: usize = 3_000;
        let num_of_iterations: usize = 1_000_000;
        let num_of_rows: usize = 100;
        let num_of_cols: usize = 100;
        (dir_name, num_of_rows, num_of_cols, num_of_alive_ants, num_of_dead_ants, num_of_iterations, ant_vision)
    } else {
        eprintln!("Usage: {} <dir_name_for_result> <ant_vision> <rows> <cols> <alive_ants> <dead_ants> <iterations>", &args[0]);
        eprintln!("Example: {} {} {} {} {} {} {} {}", &args[0], "results", 1, 100, 100, 150, 3_000, 1_000_000);
        std::process::exit(1);
    };

    if !Path::new(&dir_name).exists() {
        fs::create_dir(&dir_name)?;
    }

    let mut ants: Vec<Ant> = Vec::with_capacity(num_of_alive_ants);

    let mut grid = Grid::new(num_of_rows, num_of_cols);
    grid.randomly_populate(num_of_dead_ants, Cell::Data);
    let alive_ants_pos = grid.randomly_populate(num_of_alive_ants, Cell::Ant);

    for i in 0..num_of_alive_ants {
        ants.push(Ant::new((alive_ants_pos[i], Cell::Empty), ant_vision));
    }

    let initial_state_image = render(&mut grid);
    initial_state_image.save(format!("{}/initial_state_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_dead_ants_and_{}_iterations.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_dead_ants, num_of_iterations))?;

    let final_state_image = simulate_and_render(grid, ants, num_of_iterations);
    final_state_image.save(format!("{}/final_state_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_dead_ants_and_{}_iterations.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_dead_ants, num_of_iterations))?;

    Ok(())
}

fn simulate_and_render(mut grid: Grid, mut ants: Vec<Ant>, num_of_iterations: usize) -> RgbImage {
    let mut indices_to_remove = Vec::new();

    for _ in 0..num_of_iterations {
        for (_, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut grid, true);
        }
    }

    while !ants.is_empty() {
        for (i, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut grid, false);
            if ant.get_state() == ant::State::NotCarrying {
                indices_to_remove.push(i);
                grid.set_cell(ant.get_pos(), ant.get_pos_old_cell());
            }
        }

        for &index in indices_to_remove.iter().rev() {
            ants.remove(index);
        }
        indices_to_remove.clear();
    }

    render(&mut grid)
}

fn render(grid: &mut Grid) -> RgbImage {
    let width = grid.width() as u32;
    let height = grid.height() as u32;
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cell_color = match grid.get((x as usize, y as usize)) {
                Cell::Empty => EMPTY_CELL_COLOR,
                Cell::Ant => ALIVE_ANT_COLOR,
                Cell::Data => DEAD_ANT_COLOR,
            };
            img.put_pixel(x, y, cell_color);
        }
    }

    img
}
