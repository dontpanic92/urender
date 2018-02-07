mod world;
mod utility;
mod geometry;
mod tracer;
mod camera;
mod light;
mod material;
mod sampler;
extern crate rand;
extern crate bmp;
use world::World;
use camera::*;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: {} scene.xml output_bitmap_path", args[0]);
        return;
    }
    let now = Instant::now();
    /*let mut w = World::new();
    w.build();*/
    let w = World::load_scene(args[1].as_str());
    let img = w.camera().render(&w, &tracer::RayCast::new());

    println!("Finished rendering in {} seconds", now.elapsed().as_secs());
    img.save(&args[2]).unwrap();
}
