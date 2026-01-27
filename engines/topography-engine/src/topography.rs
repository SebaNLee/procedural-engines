use crate::{Point, Segment};
use crate::random::random_f32;

/*
 * references:
 * 
 * diamond-square:
 * https://grokipedia.com/page/Diamond-square_algorithm
 * https://en.wikipedia.org/wiki/Diamond-square_algorithm
 * https://janert.me/blog/2022/the-diamond-square-algorithm-for-terrain-generation/
 * https://www.youtube.com/watch?v=4GuAtrPnurU
 * 
 * polygonal chain:
 * https://en.wikipedia.org/wiki/Polygonal_chain
 * 
 * marching squares:
 * https://en.wikipedia.org/wiki/Marching_squares
 * https://ragingnexus.com/creative-code-lab/experiments/algorithms-marching-squares/
 * https://www.youtube.com/watch?v=0ZONMNUKTfU
 */

/*
 * note:
 * using x for X-axis
 * using y for Y-axis
 * it results in matrix[y][x]
*/
pub struct Topography {
    size: usize,
    map: Vec<f32>, // flat buffer for WASM efficiency
    borders: Vec<Vec<Vec<Point>>>, // polyline (ordered sequenced points) per level

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

    pub fn get_map(&self) -> &Vec<f32> {
        &self.map
    }

    pub fn normalize(&mut self) {
        let map = &mut self.map;
        
        let min = map.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = map.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        for value in map {
            *value = (*value - min) / (max - min);
        }
    }

    pub fn compute(&mut self) {
        self.diamond_square();
        self.compute_borders();
    }

    pub fn get_borders(&self) -> &Vec<Vec<Vec<Point>>> {
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

        let mut chunk = size - 1;

        while chunk > 1 {

            self.square_step(chunk, roughness);
            self.diamond_step(chunk, roughness);

            chunk /= 2;
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
        let map = &mut self.map;
        let half = chunk / 2;

        for y in (0..size).step_by(half) {

            let shift = if y % chunk == 0 { half } else { 0 };

            for x in (shift..size).step_by(chunk) {

                let mut sum: f32 = 0.0;
                let mut count: usize = 0;

                // north
                if y >= half {
                    sum += map[x + (y - half) * size];
                    count += 1;
                }

                // west
                if x >= half {
                    sum += map[(x - half) + y * size];
                    count += 1;
                }

                // east
                if x + half < size {
                    sum += map[(x + half) + y * size];
                    count += 1;
                }

                // south
                if y + half < size {
                    sum += map [x + (y + half) * size];
                    count += 1;
                }

                let avg = sum / count as f32;
                let random = (random_f32() - 0.5) * roughness;

                map[x + y * size] = avg + random;
            }
        }
    }

    fn compute_borders(&mut self) {
        let last = self.size - 1;

        for level in 0..self.levels {
            let threshold = level as f32 / self.levels as f32;
            let mut segments = Vec::new();

            for y in 0..last {
                for x in 0..last {
                    self.marching_squares(x, y, threshold, &mut segments);
                }
            }

            self.borders[level] = Topography::segments_to_polylines(segments);
        }
    }

    fn marching_squares(&self, x: usize, y: usize, threshold: f32, segments: &mut Vec<Segment>) {
        let size = self.size;
        let map = &self.map;

        // clockwise, starting from top left
        let value_tl = map[x + y * size];
        let value_tr = map[(x + 1) + y * size];
        let value_br = map[(x + 1) + (y + 1) * size];
        let value_bl = map[x + (y + 1) * size];
            
        let mut index = 0;
        if value_tl > threshold { index |= 1; }
        if value_tr > threshold { index |= 2; }
        if value_br > threshold { index |= 4; }
        if value_bl > threshold { index |= 8; }

        if index == 0 || index == 15 {
            return;
        }

        let point_tl = Point::new(x as f32, y as f32);
        let point_tr = Point::new((x + 1) as f32, y as f32);
        let point_br = Point::new((x + 1) as f32, (y + 1) as f32);
        let point_bl = Point::new(x as f32, (y + 1) as f32);

        let top    = || Topography::linear_interpolation(point_tl, point_tr, value_tl, value_tr, threshold);
        let right  = || Topography::linear_interpolation(point_tr, point_br, value_tr, value_br, threshold);
        let bottom = || Topography::linear_interpolation(point_br, point_bl, value_br, value_bl, threshold);
        let left   = || Topography::linear_interpolation(point_bl, point_tl, value_bl, value_tl, threshold);

        match index {
            1 | 14 => segments.push(Segment::new(top(), left())),
            2 | 13 => segments.push(Segment::new(top(), right())),
            3 | 12 => segments.push(Segment::new(left(), right())),
            4 | 11 => segments.push(Segment::new(right(), bottom())),
            6 | 9  => segments.push(Segment::new(top(), bottom())),
            7 | 8  => segments.push(Segment::new(left(), bottom())),

            5 => {
                segments.push(Segment::new(top(), right()));
                segments.push(Segment::new(bottom(), left()));
            }
            10 => {
                segments.push(Segment::new(top(), left()));
                segments.push(Segment::new(right(), bottom()));
            }

            _ => {}
        }
    }

    fn linear_interpolation(point1: Point, point2: Point, value1: f32, value2: f32, t: f32) -> Point {
        let k = (t - value1) / (value2 - value1);
        Point::new(
            point1.x + k * (point2.x - point1.x),
            point1.y + k * (point2.y - point1.y),
        )
    }
    
    fn segments_to_polylines(segments: Vec<Segment>) -> Vec<Vec<Point>> {
        // TODO
        Vec::new()
    }
}