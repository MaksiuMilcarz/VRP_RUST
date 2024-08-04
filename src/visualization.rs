use plotters::prelude::*;

use crate::{customer::Customer, vehicle::Vehicle};

pub fn visualize_solution(customers: &Vec<Customer>, vehicles: &Vec<Vehicle>) {
    let root_area = BitMapBackend::new("vrp_solution.png", (1024, 768))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Vehicle Routing Problem Solution", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..100u32, 0u32..100u32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Define different colors for different vehicles
    let colors = [RED, BLUE, GREEN, CYAN, MAGENTA, YELLOW, BLACK];

    for (i, vehicle) in vehicles.iter().enumerate() {
        let color = colors[i % colors.len()].to_rgba();
        let mut route_points = vec![];

        for customer in &vehicle.route {
            route_points.push((customer.location.x, customer.location.y));
        }

        chart
            .draw_series(LineSeries::new(route_points.clone(), &color))
            .unwrap()
            .label(format!("Vehicle {}", vehicle.id))
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.clone())
            });

        for (x, y) in route_points {
            chart.draw_series(PointSeries::of_element(
                vec![(x, y)],
                5,
                &color,
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(format!("({},{})", c.0, c.1), (10, 0), ("sans-serif", 15).into_font())
                },
            )).unwrap();
        }
    }

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
