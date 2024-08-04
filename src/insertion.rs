use std::collections::HashSet;

use crate::{customer::Customer, vehicle::Vehicle};

// use greedy insertio but dont forget to also check for demand at each insertion
pub fn greedy_insertion(customers: &Vec<Customer>, mut vehicles: Vec<Vehicle>, unassigned_customers: &HashSet<usize>) -> Vec<Vehicle>{
    let mut unassigned_customers = unassigned_customers.clone();
    while !unassigned_customers.is_empty(){
        let mut best_cost = f64::INFINITY;
        let mut best_customer = 0;
        let mut best_vehicle = 0;

        for &c in &unassigned_customers{
            for v in &mut vehicles{
                let cost = v.calculate_insertion_cost(&customers[c]);
                let demand = customers[c].demand;
                let existing_demand = v.route.iter().map(|c| c.demand).sum::<u32>();
                if cost < best_cost && existing_demand + demand <= v.capacity{
                    best_cost = cost;
                    best_customer = c;
                    best_vehicle = v.id;
                }
            }
        }

        vehicles[best_vehicle].route.push(customers[best_customer].clone());
        unassigned_customers.remove(&best_customer);
    }
    vehicles
}