use crate::Point;

pub struct Topography {
    size: usize,
    map: Vec<f32>,
    borders: Vec<Vec<Point>>,

    levels: usize,
    roughness: f32, // random range
    hurst: f32, // decay factor for roughness
}

impl Topography {
    pub fn new(size: usize, levels: usize, roughness: f32, hurst: f32) -> Self {
        Self {
            size,
            map: vec![0.0; size * size],
            borders: vec![Vec::new(); levels],
            
            levels,
            roughness,
            hurst,
        }
    }

    pub fn compute(&mut self) {
        self.diamond_square();
        self.index_borders();
    }


    pub fn get_borders(&self) -> &Vec<Vec<Point>> {
        &self.borders
    }

    fn diamond_square(&mut self) {

    }

    fn index_borders(&mut self) {

    }

    fn square_step(&mut self) {

    }

    fn diamond_step(&mut self) {

    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }
}