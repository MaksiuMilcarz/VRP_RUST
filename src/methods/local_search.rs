use crate::resources:: vehicle::Vehicle;
use super::evaluate_solution::evaluate;
use rand::Rng;
use std::f64;

pub fn simulated_annealing(vehicles: Vec<Vehicle>) -> Vec<Vehicle> {
    let mut temperature = 5000.0;
    let cooling_rate = 0.99;
    let min_temperature = 1.0; // Ensure the temperature does not become too small
    let max_iterations = 10000000;

    let mut rng = rand::thread_rng();
    let mut current_solution = vehicles.clone();
    let mut best_solution = current_solution.clone();
    let mut best_cost = evaluate(&best_solution);
    let mut current_cost = best_cost;
    let mut iteration = 0;

    while temperature > min_temperature && iteration < max_iterations {
        let new_solution = perturb(&current_solution);
        let new_cost = evaluate(&new_solution);
        let acceptance_probability = acceptance_probability(current_cost, new_cost, temperature);
        let random_number = rng.gen::<f64>();
        if new_cost < current_cost || random_number < acceptance_probability {
            current_solution = new_solution;
            current_cost = new_cost;
            if current_cost < best_cost {
                best_solution = current_solution.clone();
                best_cost = current_cost;
            }
        }
        temperature *= cooling_rate; // Decay temperature
        iteration += 1;
    }
    best_solution
}

fn perturb(vehicles: &Vec<Vehicle>) -> Vec<Vehicle> {
    let mut rng = rand::thread_rng();
    let mut new_solution = vehicles.clone();

    // Ensure we have at least one vehicle and a route with at least two customers
    if new_solution.is_empty() || new_solution.iter().all(|v| v.route.len() < 2) {
        return new_solution;
    }

    let vehicle_index = rng.gen_range(0..new_solution.len());
    let vehicle = &mut new_solution[vehicle_index];

    if vehicle.route.len() < 2 {
        return new_solution;
    }

    let customer_index = rng.gen_range(1..vehicle.route.len() - 1);
    let customer = vehicle.route.remove(customer_index);

    // Ensure that we do not insert at out-of-bound positions
    let best_position = (0..vehicle.route.len())
        .map(|i| (i, vehicle.calculate_insertion_cost_at_position(i, &customer)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|x| x.0)
        .unwrap_or(0);

    vehicle.route.insert(best_position, customer);
    new_solution
}

fn acceptance_probability(current_cost: f64, new_cost: f64, temperature: f64) -> f64 {
    if new_cost < current_cost {
        1.0
    } else {
        ((current_cost - new_cost) / temperature).exp()
    }
}
