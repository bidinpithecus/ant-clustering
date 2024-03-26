use crate::ant::Ant;
use data::Data;
use environment::Cell;
use environment::Grid;
use image::{Rgb, RgbImage};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const EMPTY_CELL_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
const ALIVE_ANT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);
const K1: f64 = 0.35;
const K2: f64 = 0.65;
const ALPHA: f64 = 30.0;

mod ant;
mod data;
mod environment;

fn add_groups_from_file(input_file: &str, grid: &mut Grid) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.trim().split('\t').collect();
        let mut attr = [0.0; 2];

        attr[0] = parts[0].parse()?;
        attr[1] = parts[1].parse()?;
        let label: u8 = parts[2].parse()?;

        let pos = grid.random_empty_cell();

        let data = Cell::Data {
            content: Data::new(pos, attr, label),
        };

        grid.set_cell(((pos.0 as isize), (pos.1 as isize)), data);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let (
        dir_name,
        num_of_rows,
        num_of_cols,
        num_of_alive_ants,
        num_of_items,
        num_of_iterations,
        ant_vision,
    ) = if args.len() == 8 {
        let dir_name: String = args[1].clone();
        let ant_vision: usize = args[2].parse()?;
        let num_of_rows: usize = args[3].parse()?;
        let num_of_cols: usize = args[4].parse()?;
        let num_of_alive_ants: usize = args[5].parse()?;
        let num_of_items: usize = args[6].parse()?;
        let num_of_iterations: usize = args[7].parse()?;
        (
            dir_name,
            num_of_rows,
            num_of_cols,
            num_of_alive_ants,
            num_of_items,
            num_of_iterations,
            ant_vision,
        )
    } else if args.len() == 1 {
        let dir_name = String::from("results");
        let ant_vision = 1;
        let num_of_alive_ants: usize = 50;
        let num_of_items: usize = 400;
        let num_of_iterations: usize = 5_000_000;
        let num_of_rows: usize = 60;
        let num_of_cols: usize = 60;
        (
            dir_name,
            num_of_rows,
            num_of_cols,
            num_of_alive_ants,
            num_of_items,
            num_of_iterations,
            ant_vision,
        )
    } else {
        eprintln!("Usage: {} <dir_name_for_result> <ant_vision> <rows> <cols> <alive_ants> <items> <iterations>", &args[0]);
        eprintln!(
            "Example: {} {} {} {} {} {} {} {}",
            &args[0], "results", 1, 100, 100, 50, 400, 1_000_000
        );
        std::process::exit(1);
    };

    if !Path::new(&dir_name).exists() {
        fs::create_dir(&dir_name)?;
    }

    let mut ants: Vec<Ant> = Vec::with_capacity(num_of_alive_ants);

    let mut grid = Grid::new(num_of_rows, num_of_cols);

    add_groups_from_file("input/4_groups.txt", &mut grid)?;

    for _ in 0..num_of_alive_ants {
        let pos = grid.random_empty_cell();
        ants.push(Ant::new((pos, Cell::Empty), ant_vision, None));
        grid.set_cell(
            ((pos.0 as isize), (pos.1 as isize)),
            Cell::Ant { carrying: None },
        )
    }

    let initial_state_image = render(&mut grid);
    initial_state_image.save(format!("{}/initial_state_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_items_and_{}_iterations.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_items, num_of_iterations))?;

    let final_state_image = simulate_and_render(grid, ants, num_of_iterations);
    final_state_image.save(format!("{}/final_state_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_items_and_{}_iterations.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_items, num_of_iterations))?;

    Ok(())
}

fn simulate_and_render(mut grid: Grid, mut ants: Vec<Ant>, num_of_iterations: usize) -> RgbImage {
    let mut indices_to_remove: Vec<usize> = Vec::with_capacity(ants.len());

    for _ in 0..num_of_iterations {
        for ant in ants.iter_mut() {
            ant.action(&mut grid, true, K1, K2, ALPHA);
        }
    }

    while !ants.is_empty() {
        for (i, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut grid, false, K1, K2, ALPHA);
            if ant.carrying().is_none() {
                indices_to_remove.push(i);
                let ant_pos = ant.get_pos();
                grid.set_cell(
                    (ant_pos.0 as isize, ant_pos.1 as isize),
                    ant.get_pos_old_cell(),
                );
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
    let scale_factor: u32 = 10;
    let width = grid.width() as u32 * scale_factor;
    let height = grid.height() as u32 * scale_factor;
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let grid_x = x / scale_factor;
            let grid_y = y / scale_factor;

            let cell_color = match grid.get((grid_x as isize, grid_y as isize)) {
                Cell::Empty => EMPTY_CELL_COLOR,
                Cell::Ant { .. } => ALIVE_ANT_COLOR,
                Cell::Data { content } => match content.label() {
                    1 => Rgb([255, 0, 0]),
                    2 => Rgb([0, 255, 0]),
                    3 => Rgb([0, 0, 255]),
                    4 => Rgb([255, 255, 0]),
                    _ => Rgb([255, 255, 255]),
                },
            };
            img.put_pixel(x, y, cell_color);
        }
    }

    img
}
