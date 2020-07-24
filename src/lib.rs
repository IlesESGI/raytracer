#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate indicatif;
extern crate rayon;

mod matrix;
mod point;
mod rendering;
pub mod scene;
mod vector;

use image::{DynamicImage, GenericImage, Pixel, Rgba};
use indicatif::ProgressBar;
use rayon::prelude::*;
use rendering::{Intersectable, Ray};
use scene::{Color, Intersection, Scene};

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let direction_to_light = -scene.light.direction.normalize();
    let light_power =
        (surface_normal.dot(&direction_to_light) as f32).max(0.0) * scene.light.intensity;
    let light_reflected = intersection.element.albedo() / std::f32::consts::PI;

    let color = intersection.element.color().clone()
        * scene.light.color.clone()
        * light_power
        * light_reflected;
    color.clamp()
}

pub fn multithreading_render(scene: &Scene) -> DynamicImage {
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
                    let color = intersection
                        .map(|i| to_rgba(&get_color(scene, &ray, &i)))
                        .unwrap_or(black);
                    pb.inc(1);
                    color
                })
                .collect()
        })
        .collect();

    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, rows[x as usize][y as usize]);
        }
    }

    pb.finish_with_message("done");

    image
}

pub fn render(scene: &Scene) -> DynamicImage {
    let pb = ProgressBar::new((scene.width * scene.height).into());
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = intersection
                .map(|i| to_rgba(&get_color(scene, &ray, &i)))
                .unwrap_or(black);
            image.put_pixel(x, y, color);
            pb.inc(1);
        }
    }

    pb.finish_with_message("done");
    image
}

// Same version of the function without the println so we can see the result in the console without being submerged by the logs
pub fn bench_multithreading_render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);

    let rows: Vec<Vec<image::Rgba<u8>>> = (0..scene.width)
        .into_par_iter()
        .map(|j| {
            (0..scene.height)
                .into_par_iter()
                .map(|i| {
                    let ray = Ray::create_prime(j, i, scene);
                    let intersection = scene.trace(&ray);
                    let color = intersection
                        .map(|i| to_rgba(&get_color(scene, &ray, &i)))
                        .unwrap_or(black);
                    color
                })
                .collect()
        })
        .collect();

    for x in 0..scene.width {
        for y in 0..scene.height {
            image.put_pixel(x, y, rows[x as usize][y as usize]);
        }
    }

    image
}

// Same version of the function without the println so we can see the result in the console without being submerged by the logs
pub fn bench_render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let intersection = scene.trace(&ray);
            let color = intersection
                .map(|i| to_rgba(&get_color(scene, &ray, &i)))
                .unwrap_or(black);
            image.put_pixel(x, y, color);
        }
    }

    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels(
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
        0,
    )
}
