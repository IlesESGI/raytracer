#![feature(test)]
extern crate clap;
extern crate image;
extern crate raytracer;
extern crate serde;
extern crate serde_json;
extern crate test;

use clap::{App, Arg};
use image::ImageFormat;
use raytracer::scene::*;
use std::fs::{File, OpenOptions};
use std::time::Instant;
use test::Bencher;

fn main() {
    let before = Instant::now();
    let app = App::new("raytracer")
        .version("1.0")
        .author("Group 3 4IABD1")
        .arg(
            Arg::with_name("scene")
                .help("Sets the scene file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("image")
                .help("Sets the output image file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("multithreading")
                .help("Sets the usage of parallelism")
                .required(true)
                .index(3),
        );
    let matches = app.get_matches();
    let multithreading_value = matches.value_of("multithreading").unwrap();

    match multithreading_value == "true" {
        true => println!("Generating image using multithread"),
        _ => println!("Generating image using monothread"),
    }

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();
    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(image_path)
        .unwrap();

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let image;

    match multithreading_value == "true" {
        true => image = raytracer::multithreading_render(&scene),
        _ => image = raytracer::render(&scene),
    };

    image.save(&mut image_file, ImageFormat::PPM).unwrap();
    println!("Image generated in : {:.2?}", before.elapsed());
}

#[bench]
fn bench_multithreading(bencher: &mut Bencher) {
    let scene_path = "scenes/objects_bench.json";
    let scene_file = File::open(scene_path).expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    // Let the bencher compute the average execution time.
    bencher.iter(|| raytracer::bench_multithreading_render(&scene));
}

#[bench]
fn bench_monothreading(bencher: &mut Bencher) {
    let scene_path = "scenes/objects_bench.json";
    let scene_file = File::open(scene_path).expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    // Let the bencher compute the average execution time.
    bencher.iter(|| raytracer::bench_render(&scene));
}
