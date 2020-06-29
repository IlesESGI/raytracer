#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate rayon;
extern crate indicatif;

pub mod scene;
mod rendering;
mod vector;
mod matrix;
mod point;

use indicatif::ProgressBar;
use rayon::prelude::*;
use scene::{Scene, Color, Intersection};
use image::{DynamicImage, GenericImage, Rgba, Pixel};
use rendering::{Ray, Intersectable};

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let direction_to_light = -scene.light.direction.normalize();
    let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                      scene.light.intensity;
    let light_reflected = intersection.element.albedo() / std::f32::consts::PI;

    let color = intersection.element.color().clone() * scene.light.color.clone() * light_power *
                light_reflected;
    color.clamp()
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);

    let pb = ProgressBar::new((scene.width * scene.height).into());

    let rows: Vec<Vec<image::Rgba<u8>>> = (0..scene.width)
    .into_par_iter()
    .map(|j| {
        (0..scene.height)
            .into_par_iter()
            .map(|i| {
                let ray = Ray::create_prime(j, i, scene);
                let intersection = scene.trace(&ray);
                let color = intersection.map(|i| to_rgba(&get_color(scene, &ray, &i)))
                    .unwrap_or(black);
                pb.inc(1);
                color
            })
            .collect()
    })
    .collect();

    // Je n'arrivais pas à faire du parallelisme directement avec la structure image donc je suis passé par un vecteur de vecteur de Pixel

    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, rows[x as usize][y as usize]);
        }
    }

    pb.finish_with_message("done");

    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels((color.red * 255.0) as u8,
                        (color.green * 255.0) as u8,
                        (color.blue * 255.0) as u8,
                        0)
}
