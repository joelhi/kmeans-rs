use rand::{thread_rng, Rng};

pub fn kmeans<T: DataPoint>(data_points: &[T], k: usize) -> Vec<usize> {
    let mut rng = thread_rng();

    // assign random centroids
    let mut centroids: Vec<T> = (0..k)
        .map(|_| {
            let i = rng.gen_range(0..data_points.len());
            data_points[i].clone()
        })
        .collect();

    // Initialize assignment vector
    let len = data_points.len();
    let mut assignments: Vec<usize> = vec![0; len];

    let mut changed: bool = true;
    let mut iter = 0;
    // iterate until stable
    while changed {
        changed = false;
        iter = iter + 1;
        let mut count = 0;
        // find closest centroid
        for (i, data_point) in data_points.iter().enumerate() {
            let mut min_dist = f64::MAX;
            let mut min_index = 0;

            for (j, centroid) in centroids.iter().enumerate() {
                let temp_dist = data_point.calculate_distance(centroid);

                if temp_dist < min_dist {
                    min_dist = temp_dist;
                    min_index = j;
                }
            }

            if assignments[i] != min_index {
                changed = true;
                count = count + 1;
                assignments[i] = min_index;
            }
        }

        // update centroids
        for i in 0..k {
            let cluster: Vec<&T> = data_points
                .iter()
                .zip(assignments.iter())
                .filter(|(_, &a)| a == i)
                .map(|(d, _)| d)
                .collect();

            let new_centroid = T::calculate_centroid(cluster.as_slice());
            centroids[i] = new_centroid;
        }

        println!("Iteration: {}, changes: {}", iter, count);
    }

    // return assignments
    assignments
}

// Trait for being a clusterable data point
pub trait DataPoint: Sized {
    fn calculate_centroid(data_points: &[&Self]) -> Self;

    fn calculate_distance(&self, other: &Self) -> f64;

    fn get_length() -> i32;

    fn create_from_array(data: &[f64]) -> Self;

    fn clone(&self) -> Self;
}


