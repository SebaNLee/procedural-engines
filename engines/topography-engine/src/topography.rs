

pub struct Topography {
    width: usize,
    height: usize,

    levels: usize,
}

impl Topography {
    pub fn new(width: usize, height: usize, levels: usize) -> Self {

        Self {
            width,
            height,
            
            levels,
        }
    }
}