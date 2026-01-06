use crate::Params;
use crate::Boid;
use crate::Vec2;

pub struct World {
    boids: Vec<Boid>,
    params: Params,
    width: f32,
    height: f32,
    
}

impl World {
    pub fn new(n: usize, width: f32, height: f32) -> Self {
        let mut boids = Vec::with_capacity(n);

        for _ in 0..n {
            boids.push(Boid::new(
                Vec2::new(
                    rand::random::<f32>() * width,
                    rand::random::<f32>() * height
                ),
                Vec2::new(
                    0.2,
                    0.2
                )
            ));
        }

        Self {
            boids,
            params: Params::default(),
            width,
            height
        }
    }
}