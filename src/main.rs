use minifb::{Key, Window, WindowOptions};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use raytracer::math::Vec3D;
use std::{thread, time::Duration};

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;
const RGB_FACTOR: f32 = 255.999;
fn main() {
    let v1 = Vec3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let v2 = Vec3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    dbg!(v1);
    dbg!(v2);
    dbg!(v1 + v2);
    dbg!(v1 - v2);

    let mut buffer = vec![0u32; IMAGE_WIDTH * IMAGE_HEIGHT];
    let buffer_ptr = buffer.as_mut_ptr() as usize;
    thread::spawn(move || {
        (0..IMAGE_WIDTH * IMAGE_HEIGHT)
            .into_par_iter()
            .for_each(|n| {
                let col = n % IMAGE_WIDTH;
                let row = n / IMAGE_WIDTH;
                const WIDTH: f32 = (IMAGE_WIDTH - 1) as f32;
                const HEIGHT: f32 = (IMAGE_HEIGHT - 1) as f32;
                let r = col as f32 / WIDTH;
                let g = 1.0 - (row as f32 / HEIGHT);
                let b = 0.25;
                let ir = (r * RGB_FACTOR).floor() as u32;
                let ig = (g * RGB_FACTOR).floor() as u32;
                let ib = (b * RGB_FACTOR).floor() as u32;
                let rgba = ((ir & 0xFF) << 16) | ((ig & 0xFF) << 8) | (ib & 0xFF);
                let ptr = buffer_ptr as *mut u32;
                std::thread::sleep(Duration::from_millis(1));
                unsafe { ptr.offset(n as isize).write(rgba) }
            });
    });
    let mut window = Window::new("Raytracer", 800, 600, WindowOptions::default())
        .expect("Failed to open window");
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // let mut buf = buffer.read().unwrap().clone();
        window
            .update_with_buffer(&buffer, IMAGE_WIDTH, IMAGE_HEIGHT)
            .unwrap();
    }
}
