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

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} output_bitmap_path", args[0]);
        return;
    }
    
    let mut w = World::new();
    w.build();
    
    let c = camera::PinholeCamera::new();
    let img = c.render(&w, &tracer::RayCast::new());
    img.save(&args[1]).unwrap();
}
