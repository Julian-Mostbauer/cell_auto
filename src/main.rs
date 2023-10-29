mod grid_object;

use grid_object::GridObject;
use minifb::{Key, Window, WindowOptions};

/* GLOBAL CONSTANTS---------------------------------------------------------------------------------------------------- */
const WINDOW_WIDTH: usize = 900;
const WINDOW_HEIGHT: usize = 900;
const _RED: u32 = 0xff0000;
const _GREEN: u32 = 0x00ff00;
const _BLUE: u32 = 0x0000ff;
const _BLACK: u32 = 0x000000;
const _WHITE: u32 = 0xffffff;
const _YELLOW: u32 = 0xffff00;
fn main() {
    let fps: f64 = 60.0;

    let mut grid1: GridObject = GridObject::new(100, 100);
    grid1.randomize();

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut window = Window::new(
        "CELLULAR AUTOMATA - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(
        (1.0 / fps * 1_000_000.0) as u64,
    )));

    let mut loop_count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut window, &mut grid1, loop_count);
        calc_image(&mut buffer, &mut grid1, loop_count);

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        loop_count += 1;
    }
}

/* INPUT HANDLEING AND PHYSICS -----------------------------------------------------------------------------------------------*/
fn handle_input(window: &mut Window, grid: &mut GridObject, _loop_counter: u32) {
    if window.is_key_down(Key::R) {
        grid.randomize();
        println!("randomized")
    }
    if window.is_key_down(Key::T) {
        grid.uniform_to_value(true);
        println!("set all to true")
    }
    if window.is_key_down(Key::F) {
        grid.uniform_to_value(false);
        println!("set all to false")
    }
}

fn calc_image(buffer: &mut [u32], grid: &mut GridObject, _loop_counter: u32) {
    clear_buffer(buffer);
    draw_grid(buffer, grid, _loop_counter);
    grid.apply_rules();
}

fn draw_grid(buffer: &mut [u32], grid: &mut GridObject, _loop_counter: u32) {
    let cell_size = (WINDOW_HEIGHT / grid.get_height()).min(WINDOW_WIDTH / grid.get_lenght());

    for y in 0..grid.get_height() {
        for x in 0..grid.get_lenght() {
            match grid._get_cell(x, y) {
                true => {
                    for y_off in 0..cell_size {
                        for x_off in 0..cell_size {
                            draw_pixel(
                                buffer,
                                (x * cell_size + x_off) as u32,
                                (y * cell_size + y_off) as u32,
                                _YELLOW,
                            );
                        }
                    }
                }
                false => (),
            }
        }
    }
}

fn draw_pixel(buffer: &mut [u32], x: u32, y: u32, color: u32) {
    let pixel: usize = (x + ((WINDOW_WIDTH as u32) * y)) as usize;
    buffer[pixel] = color;
}

fn clear_buffer(buffer: &mut [u32]) {
    for i in buffer.iter_mut() {
        *i = _BLACK;
    }
}
