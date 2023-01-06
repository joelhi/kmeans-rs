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

    // iterate until stable
    while changed {
        changed = false;

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

pub struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl DataPoint for Point3d {
    fn calculate_centroid(data_points: &[&Self]) -> Self {
        let n = data_points.len() as f64;
        let x = data_points.iter().map(|p| p.x).sum::<f64>() / n;
        let y = data_points.iter().map(|p| p.y).sum::<f64>() / n;
        let z = data_points.iter().map(|p| p.z).sum::<f64>() / n;

        Point3d { x: x, y: y, z: z }
    }

    fn calculate_distance(&self, other: &Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    fn get_length() -> i32 {
        3
    }

    fn create_from_array(data: &[f64]) -> Self {
        if data.len() != 3 {
            panic!("Point3d can only be made from a list of three numbers.");
        }

        Point3d {
            x: data[0],
            y: data[1],
            z: data[2],
        }
    }

    fn clone(&self) -> Self {
        Point3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
