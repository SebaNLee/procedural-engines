use crate::Params;
use crate::Boid;

pub struct World {
    boids: Vec<Boid>,
    params: Params,
    width: f32,
    height: f32,
    
}