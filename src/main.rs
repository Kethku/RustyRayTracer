#![feature(iterator_step_by)]

extern crate minifb;
extern crate rand;

use std::sync::{Arc, Mutex};
use minifb::{Key, WindowOptions, Window, Scale};
use rand::*;
use std::thread;

mod vector;
mod distance_field;
mod scene;

use vector::*;
use distance_field::*;

const WIDTH: usize = 1500;
const HEIGHT: usize = 1000;

const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

fn main() {
    let color_counts_mutex = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    let acc_colors_mutex = Arc::new(Mutex::new(vec![Vector::zero(); WIDTH * HEIGHT]));
    let buffer_mutex = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

    let forward = (Vector {
        x: 0.0,
        y: -0.5,
        z: 1.0
    }).normalize();

    let scene = Arc::new(Sphere::new(Vector::new(0.0, -55.0, 0.0), 50.0, Characteristics::matte(Vector::one())) +
        Sphere::new(Vector::new(0.0, 0.0, 0.0), 5.0, Characteristics::matte(Vector::one())));

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                WindowOptions {
                                    scale: Scale::FitScreen,
                                    ..WindowOptions::default()
                                }).unwrap();
    let start_position = Vector {
        x: 0.0,
        y: 25.0,
        z: -50.0
    };
    let target = start_position + forward;
    let right = forward.cross(UP).normalize();

    let scene_width = 1.0;
    let scene_height = HEIGHT as f64 / WIDTH as f64;

    let pixel_width = scene_width / WIDTH as f64;
    let pixel_height = scene_height / HEIGHT as f64;

    let half_width = WIDTH as f64 / 2.0;
    let half_height = HEIGHT as f64 / 2.0;

    let iterations = 10;

    let thread_count = 8;
    for thread_i in 0..thread_count {
        let scene = scene.clone();
        let color_counts_mutex = color_counts_mutex.clone();
        let acc_colors_mutex = acc_colors_mutex.clone();
        let buffer_mutex = buffer_mutex.clone();

        thread::spawn(move || {
            loop {
                let mut acc_color = Vector::zero();
                let mut rng = thread_rng();
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        for _ in 0..iterations {
                            let scene_x = (x as f64 - half_width) / WIDTH as f64 + rng.gen_range(0.0, pixel_width);
                            let scene_y = -(y as f64 - half_height) / WIDTH as f64 + rng.gen_range(0.0, pixel_height);

                            let target = target + right * scene_x + UP * scene_y;
                            let dir = (target - start_position).normalize();

                            acc_color = acc_color + scene.trace(
                                start_position,
                                dir,
                                5000.0
                            );
                        }

                        let i = x as usize + y as usize * WIDTH;

                        let mut color_count: u64;

                        {
                            let mut acc_colors = acc_colors_mutex.lock().unwrap();
                            acc_color = acc_colors[i] + acc_color;
                            acc_colors[i] = acc_color;
                            let mut color_counts = color_counts_mutex.lock().unwrap();
                            color_count = color_counts[i] + iterations;
                            color_counts[i] = color_count;
                        }

                        acc_color = acc_color / color_count as f64;

                        let r = match acc_color.x * 255.0 {
                            r if r > 255.0 => 255.0,
                            r => r
                        };

                        let g = match acc_color.y * 255.0 {
                            g if g > 255.0 => 255.0,
                            g => g
                        };

                        let b = match acc_color.z * 255.0 {
                            b if b > 255.0 => 255.0,
                            b => b
                        };

                        let mut buffer = buffer_mutex.lock().unwrap();
                        buffer[i] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
                    }
                }
            }
        });
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep_ms(16);
        window.update_with_buffer(&buffer_mutex.lock().unwrap()).unwrap();
    }
}
