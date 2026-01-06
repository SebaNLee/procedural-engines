#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub attraction: f32, // external force

    pub max_speed: f32,
    pub perception_radius: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            separation: 1.0,
            alignment: 1.0,
            cohesion: 1.0,
            attraction: 0.0,

            max_speed: 120.0,
            perception_radius: 50.0, 
        }
    }
}