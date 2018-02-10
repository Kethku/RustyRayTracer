extern crate minifb;

use minifb::{Key, WindowOptions, Window};

mod color;
mod vector;
mod distance_field;

use vector::*;
use distance_field::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const FIELD: Sphere = Sphere {
    position: Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0
    },
    radius: 5.0,
    characteristics: Characteristics { }
};

const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

const FORWARD: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
                                     panic!("{}", e);
                                 });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_position = Vector {
            x: 0,
            y: 0,
            z: -10.0
        };
        let target = start_position + FORWARD;
        let right = FORWARD.cross(UP).normalize();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let (new_position, _) = march(
                    &FIELD,
                    start_position,
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0
                    },
                    5000.0
                );

                let mut dist = (new_position - start_position).length();

                if dist > 255.0 {
                    dist = 255.0;
                }

                buffer[x + y * WIDTH] = (dist as u32) << 16 | (dist as u32) << 8 | (dist as u32);
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}

const MINIMUM_THRESHOLD: f64 = 0.001;
fn march<T: Field>(field: &T, position: Vector, direction: Vector, max_distance: f64) -> (Vector, Characteristics) {
    let distance = field.distance_sampler(position);

    if distance < MINIMUM_THRESHOLD || max_distance < 0.0 {
        field.characteristic_sampler(position)
    } else {
        let new_position = position + (direction * distance);
        let jump_distance = (new_position - position).length();
        march(field, new_position, direction, max_distance - jump_distance)
    }
}
