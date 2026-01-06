use wasm_bindgen::prelude::*;
use boids_engine::{World, Vec2};

#[wasm_bindgen]
pub struct WorldAPI {
    engine: World,
}

#[wasm_bindgen]
impl WorldAPI {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize, width: f32, height: f32) -> Self {
        WorldAPI {
            engine: World::new(n, width, height),
        }
    }

    pub fn set_params(&mut self, param: &str, value: f32) {
        self.engine.set_params(param, value);
    }

    pub fn set_bounce_on_edge(&mut self, bounce: bool) {
        self.engine.set_bounce_on_edge(bounce);
    }

    pub fn get_boids(&self) -> Vec<f32> {
        let boids = self.engine.get_boids();
        let mut buffer = Vec::with_capacity(boids.len() * 2);

        for boid in boids {
            buffer.push(boid.pos.x);
            buffer.push(boid.pos.y);
        }
        
        buffer
    }

    pub fn set_attractor(&mut self, x: f32, y: f32) {
        self.engine.set_attractor(Some(Vec2::new(x, y)));
    }

    pub fn clear_attractor(&mut self) {
        self.engine.clear_attractor();
    }

    pub fn step(&mut self, dt: f32) {
        self.engine.step(dt);
    }
}