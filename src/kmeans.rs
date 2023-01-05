
pub fn kmeans<T: DataPoint>(data_points: &[T], k: usize) -> Vec<usize> {
    return Vec::new();
}

// Trait for being a clusterable data point
pub trait DataPoint: Sized {
    fn calculate_centroid(data_points: &[Self]) -> Self;

    fn calculate_distance(&self, other: &Self) -> f64;

    fn get_length()->i32;

    fn create_from_array(data: &[f64]) -> Self;
}

pub struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl DataPoint for Point3d {
    fn calculate_centroid(data_points: &[Self]) -> Self {
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

    fn get_length()->i32 { 3 }

    fn create_from_array(data: &[f64]) -> Self {
        if data.len() != 3 {
            panic!("Point3d can only be made from a list of three numbers.");
        }

        Point3d { x: data[0], y: data[1], z: data[2] }

    }
}

