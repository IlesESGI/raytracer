extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate raytracer;
extern crate image;

use std::time::Instant;
use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use raytracer::scene::*;
use image::ImageFormat;

fn main() {
    let before = Instant::now();
    let app = App::new("raytracer")
        .version("1.0")
        .author("Ilès")
        .arg(Arg::with_name("scene")
            .help("Sets the scene file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("image")
            .help("Sets the output image file")
            .required(true)
            .index(2));
    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();
    let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let image = raytracer::render(&scene);

    image.save(&mut image_file, ImageFormat::PPM).unwrap();
    println!("Image generated in : {:.2?}", before.elapsed());
}
