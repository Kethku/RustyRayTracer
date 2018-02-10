extern crate minifb;
extern crate num_complex;

use minifb::{Window, Key, Scale, WindowOptions, MouseButton, MouseMode};
use num_complex::Complex64;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let mut view_width: f64 = 4.0;
    let mut view_center: Complex64 = Complex64::new(0.0, 0.0);

    let buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

    let mut window = Window::new("Noise", WIDTH, HEIGHT,
                                WindowOptions {
                                    scale: Scale::X1,
                                    ..WindowOptions::default()
                                }).unwrap();

    let mut right_down = false;
    let mut left_down = false;

    draw(&buffer, view_width, view_center);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep(time::Duration::from_millis(16));
        let new_left_down = window.get_mouse_down(MouseButton::Left);
        if !new_left_down && left_down {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                view_center = to_view_port(mouse.0 as isize, mouse.1 as isize, view_width, view_center);
                view_width = view_width * 0.6;
            });
            draw(&buffer, view_width, view_center);
        }
        left_down = new_left_down;

        let new_right_down = window.get_mouse_down(MouseButton::Right);
        if !new_right_down && right_down {
            view_center = Complex64::new(0.0, 0.0);
            view_width = 4.0;
            draw(&buffer, view_width, view_center);
        }
        right_down = new_right_down;

        let buf = buffer.lock().unwrap();
        window.update_with_buffer(&buf).unwrap();
    }
}

fn to_view_port(x: isize, y: isize, view_width: f64, view_center: Complex64) -> Complex64 {
    let view_height = view_width * (HEIGHT as f64) / (WIDTH as f64);
    Complex64::new(
        (x - WIDTH as isize / 2) as f64 * view_width / WIDTH as f64,
        (y - HEIGHT as isize / 2) as f64 * view_height / HEIGHT as f64) + view_center
}

fn draw(buffer: &Arc<Mutex<Vec<u32>>>, view_width: f64, view_center: Complex64) {
    let thread_count = 8;
    for i in 0..thread_count {
        let buffer = Arc::clone(&buffer);
        thread::spawn(move || {
            for y in i * HEIGHT / thread_count..(i + 1) * HEIGHT / thread_count {
                for x in 0..WIDTH {
                    let c = to_view_port(x as isize, y as isize, view_width, view_center);

                    let gray = match mandel(c) {
                        Some(iters) => iters % 2 * 255,
                        None => 0
                    };

                    let mut buf = buffer.lock().unwrap();
                    buf[x + y * WIDTH] = (gray << 16) | (gray << 8) | gray;
                }
            }
        });
    }
}

fn mandel(c: Complex64) -> Option<u32> {
    let mut z = Complex64::new(0.0, 0.0);

    for i in 0..500 {
        z = z * z + c;
        if z.norm_sqr() > 16.0 {
            return Some(i);
        }
    }
    return None;
}
