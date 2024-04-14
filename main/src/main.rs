use ::rand::prelude::*;
use macroquad::prelude::*;
use std::cmp::*;

const WIDTH: f32 = 750.0;
const HEIGHT: f32 = 750.0;

const SPAWN_PROBABILITY: u8 = 6; // Enter a number n to have the probability be 1/n

const W_OS_FACTOR: f32 = 2.0;
const H_OS_FACTOR: f32 = 1.925;

const SQUARES: usize = 200;
const SQUARE_SIZE: f32 = WIDTH / SQUARES as f32;

const GRID_THICKNESS: f32 = 0.15;

fn render_grid() {
    for i in 0..=SQUARES {
        draw_line(
            i as f32 * SQUARE_SIZE,
            0.0,
            i as f32 * SQUARE_SIZE,
            HEIGHT,
            GRID_THICKNESS,
            WHITE,
        );
        draw_line(
            0.0,
            i as f32 * SQUARE_SIZE,
            WIDTH,
            i as f32 * SQUARE_SIZE,
            GRID_THICKNESS,
            WHITE,
        );
    }
}

fn color_square(row: usize, column: usize) {
    let x = row as f32 * SQUARE_SIZE;
    let y = column as f32 * SQUARE_SIZE;
    draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, WHITE);
}

fn color_board(grid: &[[i32; SQUARES]; SQUARES]) {
    for x in 0..SQUARES {
        for y in 0..SQUARES {
            if grid[x][y] == 1 {
                color_square(x, y);
            }
        }
    }
}

fn reset(grid: &mut [[i32; SQUARES]; SQUARES]) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0;
        }
    }
}

fn randomize_grid(grid: &mut [[i32; SQUARES]; SQUARES]) {
    for x in 0..SQUARES {
        for y in 0..SQUARES {
            if thread_rng().gen::<u8>() % SPAWN_PROBABILITY == 0 {
                grid[x][y] = 1;
            }
        }
    }
}

fn check_neighbours(row: usize, column: usize, grid: &[[i32; SQUARES]; SQUARES]) -> usize {
    let mut live: i32 = 0;
    for x in max(0, row as i32 - 1)..min(SQUARES as i32, row as i32 + 2) {
        for y in max(0, column as i32 - 1)..min(SQUARES as i32, column as i32 + 2) {
            if x != row as i32 || y != column as i32 {
                live += grid[(x as usize + SQUARES) % SQUARES][(y as usize + SQUARES) % SQUARES];
            }
        }
    }

    if grid[row][column] == 1 {
        if live < 2 || live > 3 {
            0
        } else {
            1
        }
    } else {
        if live == 3 {
            1
        } else {
            0
        }
    }
}

fn update_grid(grid: &mut [[i32; SQUARES]; SQUARES]) {
    let mut new_grid = *grid; // Make a copy of the grid

    for row in 0..SQUARES {
        for column in 0..SQUARES {
            new_grid[row][column] = check_neighbours(row, column, grid) as i32;
        }
    }

    // Update the original grid with the new values
    *grid = new_grid;
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut grid: [[i32; SQUARES]; SQUARES] = [[0; SQUARES]; SQUARES];
    request_new_screen_size(WIDTH / W_OS_FACTOR, HEIGHT / H_OS_FACTOR);
    randomize_grid(&mut grid);

    loop {
        clear_background(BLACK);

        render_grid();
        update_grid(&mut grid);
        color_board(&grid);

        if is_key_pressed(KeyCode::Space) {
            reset(&mut grid);
            randomize_grid(&mut grid);
        }

        next_frame().await;
    }
}

