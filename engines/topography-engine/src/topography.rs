use crate::Point;
use crate::random::random_f32;

/*
 * note:
 * using x for X-axis
 * using y for Y-axis
 * it results in matrix[y][x]
*/
pub struct Topography {
    size: usize,
    map: Vec<f32>, // flat buffer for WASM efficiency
    borders: Vec<Vec<Point>>,

    levels: usize,
    roughness: f32, // pseudo random range [0, 1]
    hurst: f32, // decay factor for roughness [0, 1]
}

impl Topography {
    pub fn new(size: usize, levels: usize, roughness: f32, hurst: f32) -> Self {
        assert!((size - 1).is_power_of_two(), "Error: size must be 2^n + 1");

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
        let size = self.size;
        let last = self.size - 1;
        let map = &mut self.map;
        let mut roughness = self.roughness;

        map[0] = random_f32();
        map[last] = random_f32();
        map[last * size] = random_f32();
        map[size * size - 1] = random_f32();

        let mut chunk = size;

        while chunk > 1 {

            self.square_step(chunk, roughness);
            self.diamond_step(chunk, roughness);

            chunk = chunk.isqrt();
            roughness *= 2.0_f32.powf(-self.hurst);
        }
    }

    fn diamond_step(&mut self, chunk: usize, roughness: f32) {
        let size = self.size;
        let last = self.size - 1;
        let map = &mut self.map;
        let half = chunk / 2;
        
        for y in (0..last).step_by(chunk) {
            for x in (0..last).step_by(chunk) {
                let tl = map[x + y * size];
                let tr = map[x + chunk + y * size];
                let bl = map[x + (y + chunk) * size];
                let br = map[x + chunk + (y + chunk) * size];

                let avg = (tl + tr + bl + br) * 0.25;
                let random = (random_f32() - 0.5) * roughness;

                map[(x + half) + (y + half) * size] = avg + random;
            }
        }
    }

    fn square_step(&mut self, chunk: usize, roughness: f32) {
        let size = self.size;
        let last = self.size - 1;
        let map = &mut self.map;


    }

    fn index_borders(&mut self) {

    }
}