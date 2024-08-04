use rand;

use crate::{customer::Customer, point::Point, vehicle::Vehicle};

pub fn create_problem(num_customers: usize, num_vehicles: usize, capacity: u32) -> (Vec<Customer>, Vec<Vehicle>){
    let customers = create_customers(num_customers);
    let vehicles = create_vehicles(num_vehicles, capacity);
    (customers, vehicles)
}

//create customers with random points between 0 and 100, with random demand between 1 and 10
fn create_customers(num_customers: usize) -> Vec<Customer>{
    let mut customers = Vec::new();
    for i in 0..num_customers{
        let x = rand::random::<u32>() % 100;
        let y = rand::random::<u32>() % 100;
        let demand: u32 = rand::random::<u32>() % 10 + 1;
        customers.push(Customer{id: i, demand: demand, location: Point{x: x, y: y}});
    }
    // create hub
    let hub: Customer = Customer{
        id: num_customers, 
        demand: 0, 
        location: Point{x: 50, y: 50}
    };
    customers.push(hub);

    customers
}

fn create_vehicles(num_vehicles: usize, capacity: u32) -> Vec<Vehicle>{
    let mut vehicles = Vec::new();
    for i in 0..num_vehicles{
        vehicles.push(Vehicle{id: i, capacity: capacity, route: Vec::new()});
    }
    vehicles
}

