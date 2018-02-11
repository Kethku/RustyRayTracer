extern crate minifb;
extern crate rand;

use minifb::{Key, WindowOptions, Window, Scale};
use rand::*;

mod color;
mod vector;
mod distance_field;

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
    let mut color_counts: Vec<u64> = vec![0; WIDTH * HEIGHT];
    let mut acc_colors: Vec<Vector> = vec![Vector::zero(); WIDTH * HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let forward = (Vector {
        x: 0.0,
        y: -0.5,
        z: 1.0
    }).normalize();

    let scene = Sphere::new(Vector::new(0.0, -55.0, 0.0), 50.0, Characteristics::matte(Vector::one())) *
        Sphere::new(Vector::new(0.0, -2.0, 0.0), 5.0, Characteristics::matte(Vector::one()));

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
            y: 25.0,
            z: -50.0
        };
        let target = start_position + forward;
        let right = forward.cross(UP).normalize();

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

            buffer[i] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
