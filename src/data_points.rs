use crate::kmeans;

pub struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d{
    pub fn new(x: f64, y: f64, z:f64) -> Self {
        Point3d { x: x, y: y, z: z} 
    }
}

impl kmeans::DataPoint for Point3d {
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

pub struct Point4d {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl kmeans::DataPoint for Point4d {
    fn calculate_centroid(data_points: &[&Self]) -> Self {
        let n = data_points.len() as f64;
        let x = data_points.iter().map(|p| p.x).sum::<f64>() / n;
        let y = data_points.iter().map(|p| p.y).sum::<f64>() / n;
        let z = data_points.iter().map(|p| p.z).sum::<f64>() / n;
        let w = data_points.iter().map(|p| p.w).sum::<f64>() / n;

        Point4d { x: x, y: y, z: z, w: w }
    }

    fn calculate_distance(&self, other: &Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2) + (other.w - self.w).powi(2))
            .sqrt()
    }

    fn get_length() -> i32 {
        4
    }

    fn create_from_array(data: &[f64]) -> Self {
        if data.len() != 4 {
            panic!("Point4d can only be made from a list of four numbers.");
        }

        Point4d {
            x: data[0],
            y: data[1],
            z: data[2],
            w: data[3],
        }
    }

    fn clone(&self) -> Self {
        Point4d {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}