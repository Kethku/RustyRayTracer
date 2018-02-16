use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::f64::*;

use vector::*;
use atmosphere::*;

const THREAD_COUNT: usize = 4;

pub fn sky_renderer(buffer_mutex: Arc<Mutex<Vec<u32>>>, width: usize, height: usize, threads: usize) {
    let barrier = Arc::new(Barrier::new(THREAD_COUNT));
    {
        for t in 0..threads {
            let buffer_mutex = buffer_mutex.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                let mut sun_theta: f64 = 0.0;
                loop {
                    let sun_dir = Vector::new(sun_theta.cos(), sun_theta.sin(), 0.0);
                    for j in 0..height {
                        let y = 2.0 * (j as f64 + 0.5) / (height as f64 - 1.0) - 1.0;
                        for i in 0..width {
                            if i % threads == t {
                                let x = 2.0 * (i as f64 + 0.5) / (width as f64 - 1.0) - 1.0;
                                let z2 = x * x + y * y;
                                if z2 <= 1.0 {
                                    let phi = x.atan2(y);
                                    let theta = (1.0 - z2).acos();
                                    let dir = Vector::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());
                                    let color = calculate_sky_color(dir, sun_dir);
                                    let mut buffer = buffer_mutex.lock().unwrap();
                                    buffer[i + width * j] = color.to_int_color();
                                }
                            }
                        }
                    }

                    barrier.wait();
                    sun_theta = sun_theta + consts::PI / 200.0;
                }
            });
        }
    }
}
