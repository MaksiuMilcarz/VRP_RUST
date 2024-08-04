

#[derive(Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl Point{
    pub fn distance(&self, other: &Point) -> f64{
        let x_diff = (self.x as f64 - other.x as f64).powi(2);
        let y_diff = (self.y as f64 - other.y as f64).powi(2);
        (x_diff + y_diff).sqrt()
    }
}