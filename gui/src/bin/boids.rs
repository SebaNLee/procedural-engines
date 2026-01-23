use minifb::{Key, MouseMode, Window, WindowOptions};
use boids_engine::{World, Vec2};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Boids - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut world = World::new(200, WIDTH as f32, HEIGHT as f32);
    let dt = 1.0 / 60.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // mouse pos as attractor
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
            world.set_attractor(Some(Vec2::new(mx, my)));
        } else {
            world.clear_attractor();
        }

        world.step(dt);

        clear(&mut buffer, 0x000000);

        for boid in world.get_boids() {
            draw_pixel(
                &mut buffer,
                boid.pos.x as i32,
                boid.pos.y as i32,
                WIDTH as i32,
                HEIGHT as i32,
                0xFFFFFF,
            );
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
