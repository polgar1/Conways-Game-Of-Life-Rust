use ::rand::{random_range, Rng, RngExt};
use ::macroquad::prelude::*;
use macroquad::prelude::coroutines::wait_seconds;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_width: 1980,
        window_height: 1080,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let cell_size = 10;
    let width = 1920 / cell_size;
    let height = 1080 / cell_size;
    let chance_to_be_alive = 50;

    let mut last_update = get_time();
    let update_speed = 0.05;

    let mut grid = generate_grid(width, height, chance_to_be_alive);

    loop {

        clear_background(BLACK);
        if get_time() - last_update > update_speed {
            grid = simulate_grid(&grid, width, height, false);
            last_update = get_time();
        }
        draw_grid(&grid, cell_size as usize);
        wait_seconds(1.0);

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

fn generate_grid(width: i32, height: i32, chance: i32) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _ in 0..height {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..width {
            let random_num = ::rand::rng().random_range(1..=100);

            if random_num <= chance {
                row.push(1);
            }
            else {
                row.push(0);
            }
        }
        grid.push(row);
    }

    grid
}

fn simulate_grid(grid: &Vec<Vec<i32>>, width: i32, height: i32, highlife_rule: bool) -> Vec<Vec<i32>> {
    let mut temp_grid = grid.clone();

    for y in 0..height {
        for x in 0..width {
            let mut alive_count = 0;
            let current_cell = grid[y as usize][x as usize];

            if x > 0 && grid[y as usize][x as usize - 1] == 1 {
                alive_count += 1;
            }
            if x < width - 1 && grid[y as usize][x as usize + 1] == 1 {
                alive_count += 1;
            }
            if y > 0 && grid[y as usize - 1][x as usize] == 1 {
                alive_count += 1;
            }
            if y < height - 1 && grid[y as usize + 1][x as usize] == 1 {
                alive_count += 1;
            }
            if x > 0 && y > 0 && grid[y as usize - 1][x as usize - 1] == 1 {
                alive_count += 1;
            }
            if y < height -1 && x < width - 1 && grid[y as usize + 1][x as usize + 1] == 1 {
                alive_count += 1;
            }
            if y < height -1 && x > 0 && grid[y as usize + 1][x as usize - 1] == 1 {
                alive_count += 1;
            }
            if x < width -1 && y > 0 && grid[y as usize - 1][x as usize + 1] == 1 {
                alive_count += 1;
            }

            if current_cell == 1 {
                if alive_count > 3 || alive_count < 2 {
                    temp_grid[y as usize][x as usize] = 0;
                }
                else {
                    temp_grid[y as usize][x as usize] = 1;
                }
            }
            else {
                if alive_count == 3 || (highlife_rule && alive_count == 6) {
                    temp_grid[y as usize][x as usize] = 1;
                }
            }

        }
    }

    return temp_grid
}

fn draw_grid(grid: &Vec<Vec<i32>>, cell_size: usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 1 {
                draw_rectangle((x * cell_size as usize) as f32,
                                (y * cell_size as usize) as f32,
                                cell_size as f32,
                                cell_size as f32,
                                GREEN);
            }
        }
    }
}
