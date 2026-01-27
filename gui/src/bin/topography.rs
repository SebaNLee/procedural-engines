use topography_engine::Topography;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

const SIZE: usize = 257;
const LEVELS: usize = 16;
const ROUGHNESS: f32 = 0.9;
const HURST: f32 = 0.6;
const BLUR_RADIOUS: usize = 6;
const BLUR_ITERATIONS: usize = 3;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Topography - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut topography = Topography::new(SIZE, LEVELS, ROUGHNESS, HURST, BLUR_RADIOUS, BLUR_ITERATIONS);
    topography.compute();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear(&mut buffer, 0x000000);

        let size_f = (SIZE - 1) as f32;

        for level in 0..topography.levels() {
            let polylines = topography.get_level_borders(level);

            for polyline in polylines {
                    for w in polyline.windows(2) {
                        let p0 = &w[0];
                        let p1 = &w[1];

                        let x0 = (p0.x / size_f * WIDTH as f32) as i32;
                        let y0 = (p0.y / size_f * HEIGHT as f32) as i32;
                        let x1 = (p1.x / size_f * WIDTH as f32) as i32;
                        let y1 = (p1.y / size_f * HEIGHT as f32) as i32;

                        draw_line(
                            &mut buffer,
                            x0,
                            y0,
                            x1,
                            y1,
                            WIDTH as i32,
                            HEIGHT as i32,
                            0xFFFFFF,
                        );
                    }
                }
            }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}


fn clear(buffer: &mut [u32], color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn draw_line(
    buffer: &mut [u32],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    width: i32,
    height: i32,
    color: u32,
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        draw_pixel(buffer, x0, y0, width, height, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_pixel(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: u32,
) {
    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }

    let idx = (y * width + x) as usize;
    buffer[idx] = color;
}

