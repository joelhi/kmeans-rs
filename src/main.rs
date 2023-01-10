mod data_points;
mod img_colours;
mod kmeans;

fn main() {
    let path = "resources/test.png";

    img_colours::cluster_image_colours(path, 2);
}
