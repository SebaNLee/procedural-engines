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

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    pub fn get_boids(&self) -> &[Boids] {
        &self.boids
    }

    pub fn step(&mut self, dt: f32) {
        let accelerations: Vec<Vec2> = (0..self.boids.len()).map(|i| self.compute_acceleration(i)).collect();

        for (boid, acc) in self.boids.iter_mut().zip(accelerations) {
            boid.vel += acc;
            boid.vel = boid.vel.limit(self.params.max_speed);
            boid.pos += boid.vel * dt;

            self.wrap_around(boid);
        }
    }

    fn compute_acceleration(&self, i: usize) -> Vec2 {
        self.separation_rule(i) * self.params.separation
         + self.alignment_rule(i) * self.params.alignment
         + self.cohesion_rule(i) * self.params.cohesion
         + self.attraction_rule(i) * self.params.attraction
    }

    fn separation_rule(&self, i: usize) -> Vec2 {
        let boid = &self.boids[i];
        let mut force = Vec2::ZERO;
        let mut count = 0;

        for (j, other) in self.boids.iter().enumerate() {
            if i == j {
                continue;
            }

            let diff = other.pos - boid.pos;
            let dist = diff.normalize();

            if dist > 0.0 && dist < self.params.perception_radius {
                force -= diff.normalize() / dist;
                count += 1;
            }
        }

        if count > 0 {
            force.normalize()
        } else {
            Vec2::ZERO
        }
    }

    fn alignment_rule(&self, i: usize) -> Vec2 {

    }

    fn cohesion_rule(&self, i: usize) -> Vec2 {

    }

    fn attraction_rule(&self, i: usize) -> Vec2 {

    }

    fn wrap_around(&self, boid: &mut Boid) {
        if boid.pos.x < 0.0 {
            boid.pos.x += self.width;
        }

        if boid.pos.y < 0.0 {
            boid.pos.y += self.height;
        }

        if boid.pos.x > self.width {
            boid.pos.x -= self.width;
        }

        if boid.pos.y > self.height {
            boid.pos.y -= self.height;
        }
    }
}