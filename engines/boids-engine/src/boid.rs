use crate::Vec2;

pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel }
    }
}