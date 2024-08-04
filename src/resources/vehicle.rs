use crate::resources::customer::Customer;

use super::point::Point;

#[derive(Clone)]
pub struct Vehicle{
    pub id: usize,
    pub capacity: u32,
    pub route: Vec<Customer>,
}

impl Vehicle{
    pub fn calculate_insertion_cost(&self, customer: &Customer) -> f64{
        let mut cost = 0.0;
        
        // calculate distance from last customer to the new customer
        if self.route.len() > 0{
            let last_customer = &self.route[self.route.len() - 1];
            cost += last_customer.location.distance(&customer.location);
        }

        cost
    }

    pub fn calculate_insertion_cost_at_position(&self, position: usize, customer: &Customer) -> f64 {
        let mut cost = 0.0;
    
        // Calculate distance from the previous customer to the new customer
        if position > 0 {
            let prev_customer = &self.route[position - 1];
            cost += prev_customer.location.distance(&customer.location);
        } else {
            // If position is 0, consider the depot or starting point
            // Assuming the starting point is at (0, 0)
            cost += Point { x: 0, y: 0 }.distance(&customer.location);
        }
    
        // Calculate distance from the new customer to the next customer
        if position < self.route.len() {
            let next_customer = &self.route[position];
            cost += customer.location.distance(&next_customer.location);
        } else {
            // If position is at the end, consider the depot or ending point
            // Assuming the ending point is at (0, 0)
            cost += customer.location.distance(&Point { x: 0, y: 0 });
        }
    
        cost
    }
}