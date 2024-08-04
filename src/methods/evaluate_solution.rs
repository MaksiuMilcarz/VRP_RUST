use crate::resources::vehicle::Vehicle;

pub fn evaluate(vehicles: &Vec<Vehicle>) -> f64{
    let mut total_cost = 0.0;
    for v in vehicles{
        let mut cost = 0.0;
        for i in 0..v.route.len()-1{
            cost += v.route[i].location.distance(&v.route[i+1].location);
        }
        total_cost += cost;
    }
    total_cost
}