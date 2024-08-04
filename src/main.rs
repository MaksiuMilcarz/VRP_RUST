use std::collections::HashSet;

use VRP_Rust::methods::{evaluate_solution::evaluate, insertion::greedy_insertion, local_search::simulated_annealing, setup::create_problem, visualization::visualize_solution};

fn main() {
    let num_customers: usize = 50;
    let num_vehicles: usize = 7;
    let capacity: u32 = 50;

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
    let mut cost = evaluate(&vehicles);
    println!("Initial cost: {}", cost);

    // improve the solution using SA-based 2-opt local search
    vehicles = simulated_annealing(vehicles);
    cost = evaluate(&vehicles);
    println!("Final cost: {}", cost);

    // Visualize the solution
    visualize_solution(&vehicles);
}

