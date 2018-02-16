use std::sync::{Arc, Mutex};
use std::thread;
use rand::*;

use vector::*;
use distance_field::*;
use atmosphere::*;
use characteristics::*;

const UP: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0
};

pub fn scene_renderer(colors_mutex: Arc<Mutex<Vec<Vector>>>, width: usize, height: usize, threads: usize) {
    let color_counts_mutex = Arc::new(Mutex::new(vec![0; width * height]));
    let acc_colors_mutex = Arc::new(Mutex::new(vec![Vector::zero(); width * height]));
    let forward = (Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0
    }).normalize();

    let ground = Plane::new(Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0), Characteristics::matte(Vector::one()));
    let mirror = Sphere::new(Vector::new(-0.8, 0.0, 2.0), 1.0, Characteristics::mirror(Vector::one()));
    let sphere = Sphere::new(Vector::new(2.0, 0.0, -1.2), 1.0, Characteristics::matte(Vector::one()));
    let distant = Sphere::new(Vector::new(4.0, 0.0, 20.0), 1.0, Characteristics::matte(Vector::one()));

    let scene = Arc::new(ground + sphere + mirror + distant);

    let start_position = Vector {
        x: 0.0,
        y: 0.2,
        z: -3.0
    };
    let target = start_position + forward;
    let right = UP.cross(forward).normalize();
    let up = forward.cross(right).normalize();

    let scene_width = 1.0;
    let scene_height = height as f64 / width as f64;

    let pixel_width = scene_width / width as f64;
    let pixel_height = scene_height / height as f64;

    let half_width = width as f64 / 2.0;
    let half_height = height as f64 / 2.0;

    let iterations = 10;

    let sun_dir = Vector::new(0.0, 1.0, 0.0).normalize();

    for _ in 0..threads {
        let scene = scene.clone();
        let color_counts_mutex = color_counts_mutex.clone();
        let acc_colors_mutex = acc_colors_mutex.clone();
        let colors_mutex = colors_mutex.clone();

        thread::spawn(move || {
            loop {
                let mut acc_color = Vector::zero();
                let mut processed_iterations = 0;
                let mut rng = thread_rng();
                let x = rng.gen_range(0, width);
                let y = rng.gen_range(0, height);

                for i in 0..iterations {
                    processed_iterations = processed_iterations + 1;
                    let scene_x = (x as f64 - half_width) / width as f64 + rng.gen_range(0.0, pixel_width);
                    let scene_y = -(y as f64 - half_height) / width as f64 + rng.gen_range(0.0, pixel_height);

                    let target = target + right * scene_x + up * scene_y;
                    let dir = (target - start_position).normalize();

                    acc_color = acc_color + scene.trace(
                        start_position,
                        dir,
                        sun_dir
                    );
                    match scene.field.ray_cast(start_position, dir) {
                        Some(intersect) => continue,
                        None => break
                    }
                }

                let i = x as usize + y as usize * width;

                let color_count: u64;

                {
                    let mut acc_colors = acc_colors_mutex.lock().unwrap();
                    acc_color = acc_colors[i] + acc_color;
                    acc_colors[i] = acc_color;
                    let mut color_counts = color_counts_mutex.lock().unwrap();
                    color_count = color_counts[i] + processed_iterations;
                    color_counts[i] = color_count;
                }

                acc_color = acc_color / color_count as f64;

                let mut buffer = colors_mutex.lock().unwrap();
                buffer[i] = acc_color;
            }
        });
    }
}
