#![feature(iterator_step_by)]

extern crate minifb;
extern crate rand;

use std::sync::{Arc, Mutex, Barrier};
use minifb::{Key, WindowOptions, Window, Scale};
use std::thread;
use std::f64::*;

mod vector;
mod distance_field;
mod scene;
mod atmosphere;
mod characteristics;
mod sky_renderer;
mod scene_renderer;

use vector::*;

const WIDTH: usize = 960;
const HEIGHT: usize = 540;

fn main() {
    let buffer_mutex = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    let colors_mutex = Arc::new(Mutex::new(vec![Vector::zero(); WIDTH * HEIGHT]));

    let mut window = Window::new("Test - ESC to exit",
                                WIDTH,
                                HEIGHT,
                                WindowOptions {
                                    scale: Scale::X1,
                                    ..WindowOptions::default()
                                }).unwrap();

    // use sky_renderer::*;
    // sky_renderer(buffer_mutex.clone(), WIDTH, HEIGHT);

    use scene_renderer::*;
    scene_renderer(buffer_mutex.clone(), WIDTH, HEIGHT);

    let frame_length = std::time::Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep(frame_length);
        window.update_with_buffer(&buffer_mutex.lock().unwrap()).unwrap();
    }
}
