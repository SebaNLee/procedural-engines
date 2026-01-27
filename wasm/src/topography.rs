use wasm_bindgen::prelude::*;
use topography_engine::Topography;

#[wasm_bindgen]
pub struct TopographyAPI {
    engine: Topography,
}

#[wasm_bindgen]
impl TopographyAPI {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, levels: usize, roughness: f32, hurst: f32, blur_radious: usize, blur_iterations: usize) -> Self {
        TopographyAPI {
            engine: Topography::new(size, levels, roughness, hurst, blur_radious, blur_iterations),
        }
    }

    pub fn compute(&mut self) {
        self.engine.compute();
    }

    pub fn get_map(&self) -> Vec<f32>  {
        self.engine.get_map().clone()
    }
    
    pub fn get_level_borders(&self, level: usize) -> Vec<f32> {
        if level >= self.engine.levels() {
            return Vec::new();
        }

        let borders = self.engine.get_level_borders(level);
        
        // [level][x0,y0,x1,y1,-1.0,-1.0,...] (-1,-1 = polyline separator)
        let mut buffer = Vec::new();

        for polyline in borders {
            for point in polyline {
                buffer.push(point.x);
                buffer.push(point.y);
            }

            buffer.push(-1.0);
            buffer.push(-1.0);
        }

        buffer
    }
}