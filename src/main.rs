#![feature(iterator_step_by)]

extern crate minifb;
extern crate rand;

use std::sync::{Arc, Mutex};
use minifb::{Key, WindowOptions, Window, Scale};
use rand::*;
use std::thread;
use std::f64::*;

mod vector;
mod distance_field;
mod scene;
mod atmosphere;
mod characteristics;

use vector::*;
use distance_field::*;
use atmosphere::*;
use characteristics::*;

const WIDTH: usize = 300;
const HEIGHT: usize = 200;
const THREAD_COUNT: usize = 1;

const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

fn main() {
    let color_counts_mutex = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    let acc_colors_mutex = Arc::new(Mutex::new(vec![Vector::zero(); WIDTH * HEIGHT]));
    let buffer_mutex = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

    let finished_mutex = Arc::new(Mutex::new(vec![false; THREAD_COUNT]));

    let mut window = Window::new("Test - ESC to exit",
                                WIDTH,
                                HEIGHT,
                                WindowOptions {
                                    scale: Scale::X2,
                                    ..WindowOptions::default()
                                }).unwrap();

    {
        let mut sun_theta: f64 = 0.0;
        for t in 0..THREAD_COUNT {
            let buffer_mutex = buffer_mutex.clone();
            let finished_mutex = finished_mutex.clone();
            thread::spawn(move || {
                loop {
                    let sun_dir = Vector::new(sun_theta.cos(), sun_theta.sin(), 0.0);
                    for j in t * (HEIGHT / THREAD_COUNT)..(t + 1) * (HEIGHT / THREAD_COUNT) {
                        let y = 2.0 * (j as f64 + 0.5) / (HEIGHT as f64 - 1.0) - 1.0;
                        for i in 0..WIDTH {
                            let x = 2.0 * (i as f64 + 0.5) / (WIDTH as f64 - 1.0) - 1.0;
                            let z2 = x * x + y * y;
                            if z2 <= 1.0 {
                                let phi = x.atan2(y);
                                let theta = (1.0 - z2).acos();
                                let dir = Vector::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());
                                let color = calculate_sky_color(dir, sun_dir);
                                let mut buffer = buffer_mutex.lock().unwrap();
                                buffer[i + WIDTH * j] = color.to_int_color();
                            }
                        }
                    }



                    sun_theta = sun_theta + consts::PI / 200.0;
                }
            });
        }
    }

    // let forward = (Vector {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 1.0
    // }).normalize();

    // let ground = Plane::new(Vector::new(0.0, 1.0, 0.0), -1.0, Characteristics::matte(Vector::one()));
    // let sphere = Sphere::new(Vector::zero(), 1.0, Characteristics::matte(Vector::one()));

    // let scene = Arc::new(ground + sphere);

    // let start_position = Vector {
    //     x: 0.0,
    //     y: 0.0,
    //     z: -5.0
    // };
    // let target = start_position + forward;
    // let right = forward.cross(UP).normalize();

    // let scene_width = 1.0;
    // let scene_height = HEIGHT as f64 / WIDTH as f64;

    // let pixel_width = scene_width / WIDTH as f64;
    // let pixel_height = scene_height / HEIGHT as f64;

    // let half_width = WIDTH as f64 / 2.0;
    // let half_height = HEIGHT as f64 / 2.0;

    // let iterations = 1;

    // for _ in 0..THREAD_COUNT {
    //     let scene = scene.clone();
    //     let color_counts_mutex = color_counts_mutex.clone();
    //     let acc_colors_mutex = acc_colors_mutex.clone();
    //     let buffer_mutex = buffer_mutex.clone();

    //     thread::spawn(move || {
    //         loop {
    //             let mut acc_color = Vector::zero();
    //             let mut rng = thread_rng();
    //             let x = rng.gen_range(0, WIDTH);
    //             let y = rng.gen_range(0, HEIGHT);
    //             for _ in 0..iterations {
    //                 let scene_x = (x as f64 - half_width) / WIDTH as f64 + rng.gen_range(0.0, pixel_width);
    //                 let scene_y = -(y as f64 - half_height) / WIDTH as f64 + rng.gen_range(0.0, pixel_height);

    //                 let target = target + right * scene_x + UP * scene_y;
    //                 let dir = (target - start_position).normalize();

    //                 acc_color = acc_color + scene.trace(
    //                     start_position,
    //                     dir,
    //                     5000.0
    //                 );
    //             }

    //             let i = x as usize + y as usize * WIDTH;

    //             let color_count: u64;

    //             {
    //                 let mut acc_colors = acc_colors_mutex.lock().unwrap();
    //                 acc_color = acc_colors[i] + acc_color;
    //                 acc_colors[i] = acc_color;
    //                 let mut color_counts = color_counts_mutex.lock().unwrap();
    //                 color_count = color_counts[i] + iterations;
    //                 color_counts[i] = color_count;
    //             }

    //             acc_color = acc_color / color_count as f64;

    //             let mut buffer = buffer_mutex.lock().unwrap();
    //             buffer[i] = acc_color.to_int_color();
    //         }
    //     });
    // }

    let frame_length = std::time::Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep(frame_length);
        window.update_with_buffer(&buffer_mutex.lock().unwrap()).unwrap();
    }
}
