
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self{
        Self {
            x,
            y,
        }
    }
}

pub struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self{
        Self {
            a,
            b,
        }
    }

    pub fn segments_to_polylines(segments: Vec<Segment>) -> Vec<Vec<Point>> {
        // TODO
        Vec::new()
    }
}