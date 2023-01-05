fn kmeans<T: DataPoint>(data_points: &[T], k: usize) -> Vec<usize> {
    return Vec::new();
}

// Trait for being a clusterable data point
trait DataPoint: Sized {
    fn calculate_centroid(data_points: &[Self]) -> Self;

    fn calculate_distance(&self, other: &Self) -> f64;
}

struct Point3d {
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
}
