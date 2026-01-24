

pub struct Map {
    width: usize,
    height: usize,

    levels: usize,
}

impl Map {
    pub fn new(width: usize, height: usize, levels: usize) -> Self {

        Self {
            width,
            height,
            
            levels,
        }
    }
}