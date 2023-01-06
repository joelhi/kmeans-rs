use crate::kmeans;
use crate::data_points;

use data_points::*;
use image::GenericImageView;
use image::{DynamicImage, Rgb};

pub fn cluster_image_colours(path: &str, k: usize) {

    let result = image::open(path);

    if !result.is_err(){
        let image = result.unwrap();
        let rgb = get_rgb_values(&image);

        let _assignments = kmeans::kmeans(&rgb, k);

        println!("Done");
    }
    else{
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