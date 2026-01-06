use crate::Params;
use crate::Boid;
use crate::Vec2;

pub struct World {
    boids: Vec<Boid>,
    params: Params,
    width: f32,
    height: f32,

    attractor: Option<Vec2>,   
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
            height,
            attractor: None,
        }
    }

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    pub fn get_boids(&self) -> &[Boid] {
        &self.boids
    }

    pub fn set_attractor(&mut self, pos: Option<Vec2>) {
        self.attractor = pos;
    }

    pub fn clear_attractor(&mut self) {
        self.attractor = None
    }

    pub fn step(&mut self, dt: f32) {
        let accelerations: Vec<Vec2> = (0..self.boids.len()).map(|i| self.compute_acceleration(i)).collect();

        for (boid, acc) in self.boids.iter_mut().zip(accelerations) {
            boid.vel += acc;
            boid.vel = boid.vel.limit(self.params.max_speed);
            boid.pos += boid.vel * dt;

            // wrap around
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

    fn compute_acceleration(&self, i: usize) -> Vec2 {
        let acc = self.separation_rule(i) * self.params.separation
         + self.alignment_rule(i) * self.params.alignment
         + self.cohesion_rule(i) * self.params.cohesion
         + self.attraction_rule(i) * self.params.attraction;

        let noise = Vec2::new(
            rand::random::<f32>() - 0.5,
            rand::random::<f32>() - 0.5,
        ).normalize() * 0.8;

        acc + noise
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
            let dist = diff.magnitude();

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
        let boid = &self.boids[i];
        let mut avg_vel = Vec2::ZERO;
        let mut count = 0;

        for (j, other) in self.boids.iter().enumerate() {
            if i == j {
                continue;
            }

            let diff = other.pos - boid.pos;
            let dist = diff.magnitude();

            if dist < self.params.perception_radius {
                avg_vel += other.vel;
                count += 1;
            }
        }

        if count > 0 {
            let target = (avg_vel / count as f32).normalize();
            let current = boid.vel.normalize();
            (target - current).normalize()
        } else {
            Vec2::ZERO
        }
    }

    fn cohesion_rule(&self, i: usize) -> Vec2 {
        let boid = &self.boids[i];
        let mut center = Vec2::ZERO;
        let mut count = 0;


        for (j, other) in self.boids.iter().enumerate() {
            if i == j {
                continue;
            }

            let diff = other.pos - boid.pos;
            let dist = diff.magnitude();

            if dist < self.params.perception_radius {
                center += other.pos;
                count += 1;
            }
        }

        if count > 0 {
            ((center / count as f32) - boid.pos).normalize() 
        } else {
            Vec2::ZERO
        }
    }

    fn attraction_rule(&self, i: usize) -> Vec2 {
        if let Some(target) = self.attractor {
            let dir = target - self.boids[i].pos;
            if dir.magnitude() > 0.0 {
                dir.normalize()
            } else {
                Vec2::ZERO
            }
        } else {
            Vec2::ZERO
        }
    }
}