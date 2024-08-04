use crate::resources::customer::Customer;

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
}