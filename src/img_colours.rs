use crate::data_points;
use crate::kmeans;
use crate::kmeans::DataPoint;

use data_points::*;
use image::GenericImageView;
use image::ImageBuffer;
use image::RgbImage;
use image::{DynamicImage, Rgb};

pub fn cluster_image_colours(path: &str, k: usize) {
    let result = image::open(path);

    if !result.is_err() {
        let image = result.unwrap();

        let rgb = get_rgb_values(&image);

        let (assignments, centroids) = kmeans::kmeans(&rgb, k);

        let clustered: Vec<Point3d> = assignments.iter().map(|i| centroids[*i].clone()).collect();

        let clustered_image = create_image(image.width(), image.height(), &clustered);

        clustered_image
            .save(format!("{}{}{}", "resources/clustered_k", k, ".png"))
            .expect("failed to save file.");
    } else {
        println!("{}", result.unwrap_err());
    }
}

pub fn get_rgb_values(image: &DynamicImage) -> Vec<Point3d> {
    let mut data_points = vec![];
    for pixel in image.pixels() {
        let rgb = pixel.2;
        let (r, g, b) = (rgb.0[0] as f64, rgb.0[1] as f64, rgb.0[2] as f64);
        let data_point = Point3d::new(r, g, b);
        data_points.push(data_point);
    }
    data_points
}

fn create_image(width: u32, height: u32, pixels: &[Point3d]) -> DynamicImage {
    let mut image: RgbImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let i: usize = (y * width + x).try_into().unwrap();
        *pixel = Rgb {
            0: [pixels[i].x as u8, pixels[i].y as u8, pixels[i].z as u8],
        };
    }
    DynamicImage::ImageRgb8(image)
}
