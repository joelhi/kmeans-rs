mod kmeans;
mod data_points;
mod img_colours;

use data_points::*;
use rand::Rng;

fn main() {
    //let data = generate_random_data::<Point4d>(100);

    //let _assignments = kmeans::kmeans(&data, 5);

    let path = "resources/color.png";

    img_colours::cluster_image_colours(path, 12);
}

fn generate_random_data<T: kmeans::DataPoint>(n: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut data_set: Vec<T> = Vec::with_capacity(n);

    for _ in 0..n {
        let data_len = T::get_length();
        let data: Vec<f64> = (0..data_len).map(|_| rng.gen_range(0.0..100.0)).collect();

        data_set.push(kmeans::DataPoint::create_from_array(&data))
    }

    data_set
}
