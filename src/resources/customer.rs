use crate::resources::point::Point;


#[derive(Clone)]
pub struct Customer{
    pub id: usize,
    pub demand: u32,
    pub location: Point,
}
