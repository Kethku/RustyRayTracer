extern crate minifb;
extern crate rand;

use minifb::{Key, WindowOptions, Window, Scale};
use rand::*;

mod color;
mod vector;
mod distance_field;

use vector::*;
use distance_field::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

const FORWARD: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0
};

fn main() {
    let mut color_counts: Vec<u64> = vec![0; WIDTH * HEIGHT];
    let mut acc_colors: Vec<Vector> = vec![Vector::zero(); WIDTH * HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let scene = Sphere::new(Vector::new(0.0, -55.0, 0.0), 50.0, Characteristics::matte()) +
                Sphere::new(Vector::new(0.0, 0.0, 0.0), 5.0, Characteristics::mirror());

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                WindowOptions {
                                    scale: Scale::FitScreen,
                                    ..WindowOptions::default()
                                }).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_position = Vector {
            x: 0.0,
            y: 0.0,
            z: -50.0
        };
        let target = start_position + FORWARD;
        let right = FORWARD.cross(UP).normalize();

        for _ in 0..100000 {
            let half_width = WIDTH as f64 / 2.0;
            let half_height = HEIGHT as f64 / 2.0;
            let x = thread_rng().gen_range(0.0, WIDTH as f64);
            let y = thread_rng().gen_range(0.0, HEIGHT as f64);
            let scene_x = (x - half_width) / WIDTH as f64;
            let scene_y = -(y - half_height) / WIDTH as f64;

            let target = target + right * scene_x + UP * scene_y;
            let dir = (target - start_position).normalize();
            let color = scene.trace(
                Vector::one(),
                start_position,
                dir,
                5000.0
            );

            let i = x as usize + y as usize * WIDTH;

            let mut acc_color = acc_colors[i] + color;
            acc_colors[i] = acc_color;
            let color_count = color_counts[i] + 1;
            color_counts[i] = color_count;

            acc_color = acc_color / color_count as f64;

            buffer[i] = ((acc_color.x * 255.0) as u32) << 16 | ((acc_color.y * 255.0) as u32) << 8 | ((acc_color.z * 255.0) as u32);
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
