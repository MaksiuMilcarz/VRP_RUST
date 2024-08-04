use std::collections::HashSet;
use VRP_Rust::customer::Customer;
use VRP_Rust::insertion::greedy_insertion;
use VRP_Rust::setup::create_problem;
use VRP_Rust::vehicle::Vehicle;
use VRP_Rust::visualization::visualize_solution;

fn main() {
    let num_customers: usize = 10;
    let num_vehicles: usize = 3;
    let capacity: u32 = 25;

    let (customers, mut vehicles) = create_problem(num_customers, num_vehicles, capacity);
    // create a set of unassigned customers
    let mut unassigned_customers = HashSet::new();
    for c in &customers{
        if c.id != num_customers{
            unassigned_customers.insert(c.id);
        }
    }
    // insert hub into each vehicle route
    for v in &mut vehicles{
        v.route.push(customers[num_customers].clone());
    }

    // insert each customer one by one into routes using greedy insertion
    vehicles = greedy_insertion(&customers, vehicles, &unassigned_customers);

    // improve the solution using: 

    // Visualize the solution
    visualize_solution(&customers, &vehicles);
}

