#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub attraction: f32, // aditional 4th rule, external force
    pub noise: f32, // noise factor

    pub max_speed: f32,
    pub perception_radius: f32,

    pub bounce_on_edge: bool, 
}

impl Default for Params {
    fn default() -> Self {
        Self {
            separation: 2.7,
            alignment: 0.5,
            cohesion: 3.0,
            attraction: 15.0,
            noise: 0.5,

            max_speed: 200.0,
            perception_radius: 50.0,
            bounce_on_edge: true,
        }
    }
}