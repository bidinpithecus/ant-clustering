use crate::ant::Ant;
use data::Data;
use environment::{Cell, Grid};
use image::{Rgb, RgbImage};
use std::env;
use std::error::Error;
use std::f64::{INFINITY, NEG_INFINITY};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const EMPTY_CELL_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
const ALIVE_ANT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);

mod ant;
mod data;
mod environment;

fn add_groups_from_file(
    input_file: String,
    grid: &mut Grid,
) -> Result<(String, usize), Box<dyn Error>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut num_of_items = 0;
    let mut higher_label = 0;

    for line in reader.lines() {
        num_of_items += 1;
        let line = line?;

        let parts: Vec<&str> = line.trim().split('\t').collect();
        let mut attr = [0.0; 2];

        attr[0] = parts[0].parse()?;
        attr[1] = parts[1].parse()?;
        let label: u8 = parts[2].parse()?;
        if label > higher_label {
            higher_label = label;
        }

        let pos = grid.random_empty_cell();

        let data = Cell::Data {
            content: Data::new(pos, attr, label),
        };

        grid.set_cell(((pos.0 as isize), (pos.1 as isize)), data);
    }

    Ok((format!("{higher_label}_groups"), num_of_items))
}

fn normalize_data_set(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let input = File::open(input_file)?;
    let reader = BufReader::new(input);
    let mut min_x = INFINITY;
    let mut min_y = INFINITY;
    let mut max_x = NEG_INFINITY;
    let mut max_y = NEG_INFINITY;

    for line in reader.lines() {
        let line = line?;
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 2 {
            continue;
        }
        let x: f64 = cols[0].parse().unwrap();
        let y: f64 = cols[1].parse().unwrap();

        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    let mut buffer = vec![];

    for line in reader.lines() {
        let line = line?;
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 2 {
            continue;
        }

        let x: f64 = cols[0].parse().unwrap();
        let y: f64 = cols[1].parse().unwrap();

        let normalized_x = (x - min_x) / (max_x - min_x);
        let normalized_y = (y - min_y) / (max_y - min_y);

        buffer.push(format!(
            "{:.16}\t{:.16}\t{}\n",
            normalized_x, normalized_y, cols[2]
        ));
    }

    fs::write(output_file, buffer.join(""))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let (
        input_file,
        num_of_rows,
        num_of_cols,
        num_of_alive_ants,
        num_of_iterations,
        ant_vision,
        k1,
        k2,
        alpha,
    ) = if args.len() == 10 {
        let input_file: String = args[1].clone();
        let ant_vision: usize = args[2].parse()?;
        let num_of_rows: usize = args[3].parse()?;
        let num_of_cols: usize = args[4].parse()?;
        let num_of_alive_ants: usize = args[5].parse()?;
        let num_of_iterations: usize = args[6].parse()?;
        let k1: f64 = args[7].parse()?;
        let k2: f64 = args[8].parse()?;
        let alpha: f64 = args[9].parse()?;
        (
            input_file,
            num_of_rows,
            num_of_cols,
            num_of_alive_ants,
            num_of_iterations,
            ant_vision,
            k1,
            k2,
            alpha,
        )
    } else if args.len() == 1 {
        let input_file: String = "input/4_groups.txt".to_string();
        let ant_vision = 1;
        let num_of_alive_ants: usize = 10;
        let num_of_iterations: usize = 4_000_000;
        let num_of_rows: usize = 50;
        let num_of_cols: usize = 50;
        let k1 = 1.0; // Default value for k1
        let k2 = 1.0; // Default value for k2
        let alpha = 1.0; // Default value for alpha
        (
            input_file,
            num_of_rows,
            num_of_cols,
            num_of_alive_ants,
            num_of_iterations,
            ant_vision,
            k1,
            k2,
            alpha,
        )
    } else {
        eprintln!(
            "Usage: {} <input_file> <ant_vision> <rows> <cols> <alive_ants> <iterations> <k1> <k2> <alpha>",
            &args[0]
        );
        eprintln!(
            "Example: {} {} {} {} {} {} {} {} {} {}",
            &args[0], "input/4_groups.txt", 1, 50, 50, 10, 4_000_000, 0.35, 0.65, 13.0
        );
        std::process::exit(1);
    };

    let mut ants: Vec<Ant> = Vec::with_capacity(num_of_alive_ants);
    let mut grid = Grid::new(num_of_rows, num_of_cols);

    let (second_dir, num_of_items) = add_groups_from_file(input_file, &mut grid)?;

    let dir_name = format!("results/{second_dir}");

    if !Path::new(&dir_name).exists() {
        fs::create_dir(&dir_name)?;
    }

    for _ in 0..num_of_alive_ants {
        let pos = grid.random_empty_cell();
        ants.push(Ant::new((pos, Cell::Empty), ant_vision, None));
        grid.set_cell(
            ((pos.0 as isize), (pos.1 as isize)),
            Cell::Ant { carrying: None },
        )
    }

    let initial_state_image = render(&mut grid);
    initial_state_image.save(format!("{}/state_0_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_items_and_{}_iterations_{}_k1_{}_k2_{}_alpha.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_items, num_of_iterations, k1, k2, alpha))?;

    let final_state_image = simulate_and_render(grid, ants, num_of_iterations, k1, k2, alpha);

    final_state_image.save(format!("{}/state_2_for_{}x{}_grid_with_{}_alive_ants_with_radius_vision_{}_and_{}_items_and_{}_iterations_{}_k1_{}_k2_{}_alpha.png", dir_name, num_of_rows, num_of_cols, num_of_alive_ants, ant_vision, num_of_items, num_of_iterations, k1, k2, alpha))?;

    Ok(())
}

fn simulate_and_render(
    mut grid: Grid,
    mut ants: Vec<Ant>,
    num_of_iterations: usize,
    k1: f64,
    k2: f64,
    alpha: f64,
) -> RgbImage {
    let mut indices_to_remove: Vec<usize> = Vec::with_capacity(ants.len());
    for _ in 0..num_of_iterations {
        for ant in ants.iter_mut() {
            ant.action(&mut grid, true, k1, k2, alpha);
        }
    }

    while !ants.is_empty() {
        for (i, ant) in ants.iter_mut().enumerate() {
            ant.action(&mut grid, false, k1, k2, alpha);
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
                    1 => Rgb([132, 41, 219]),
                    2 => Rgb([14, 193, 66]),
                    3 => Rgb([255, 87, 33]),
                    4 => Rgb([98, 205, 235]),
                    5 => Rgb([168, 17, 234]),
                    6 => Rgb([40, 119, 66]),
                    7 => Rgb([221, 130, 35]),
                    8 => Rgb([73, 227, 76]),
                    9 => Rgb([15, 88, 203]),
                    10 => Rgb([194, 93, 156]),
                    11 => Rgb([255, 207, 33]),
                    12 => Rgb([109, 23, 206]),
                    13 => Rgb([32, 199, 195]),
                    14 => Rgb([225, 71, 28]),
                    15 => Rgb([77, 144, 31]),
                    _ => Rgb([255, 255, 255]),
                },
            };
            img.put_pixel(x, y, cell_color);
        }
    }

    img
}
