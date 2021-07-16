use crate::image::parameters;

use std::env;

mod bvh;

mod image;

mod math;

mod ray_tracing;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parameters::process_args(args) {
        Some((scene, parameters)) => {
            scene.generate_image_threaded(parameters);
        }
        None => {}
    }
}
