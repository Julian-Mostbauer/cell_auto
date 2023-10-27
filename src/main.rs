use std::{thread, time};

mod grid_object;
use grid_object::GridObject;

fn main() {
    let mut grid1: GridObject = GridObject::new(10, 10);
    grid1.uniform_to_value(false);
    loop {
        grid1.apply_rules();
        grid1.output();
        println!("-------------");
        thread::sleep(time::Duration::from_millis(1000));
    }
}
