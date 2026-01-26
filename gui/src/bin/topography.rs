use topography_engine::Topography;

fn main() {

    let size = 9;

    let mut topography = Topography::new(size, 10, 0.5, 0.5);

    topography.compute();

    let map = topography.get_map();

    for y in 0..size {
        for x in 0..size {
            print!("{:5.2} ", map[x + y * size]);
        }
        println!();
    }
}
