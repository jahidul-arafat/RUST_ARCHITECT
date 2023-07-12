use prettytable::{format, row, Row, Table};
use prettytable::cell;
use csv::Writer;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, world!");
    handle_power_oversubscription();
}


fn handle_power_oversubscription() {
    let system_power_consumption = calculate_system_power_consumption(50000, 500, 150, 20, 200, 50, 80, 5, 1000, 200, 500);
    println!("System Power Consumption (Actual Consumption): {} watts", system_power_consumption);

    let system_demand = calculate_system_demand(90, 50, 150, 10, 100, 20, 80, 3, 500, 200, 300);
    println!("System Demand (Expected to Run Efficiently): {} watts", system_demand);

    if system_power_consumption > system_demand {
        let mut sorted_users = sort_users();

        let total_resource_reduction: u32 = sorted_users.iter().map(|user| user.resource_reduction).sum();
        let total_power_consumption: u32 = sorted_users.iter().map(|user| user.power_consumption).sum();

        if total_power_consumption > system_demand {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row![
                "User ID",
                "Initial Power Consumption",
                "Resource Reduction",
                "Updated Power Consumption",
                "Bid Amount",
                "Power Saved"
            ]);

            let arc_table = Arc::new(Mutex::new(table));

            let threads: Vec<_> = sorted_users
                .into_iter()
                .map(|mut user| {
                    let arc_table = Arc::clone(&arc_table);
                    thread::spawn(move || {
                        let (updated_resource_reduction, updated_power_consumption, bid_amount, power_saved) =
                            user.allocate_resource_reduction(system_power_consumption, system_demand);

                        let mut table = arc_table.lock().unwrap();
                        table.add_row(Row::new(vec![
                            cell!(user.id),
                            cell!(user.power_consumption),
                            cell!(updated_resource_reduction.reduction_value),
                            cell!(updated_power_consumption.consumption_value),
                            cell!(bid_amount),
                            cell!(power_saved),
                        ]));
                    })
                })
                .collect();

            for thread in threads {
                thread.join().unwrap();
            }

            let table = arc_table.lock().unwrap();
            table.printstd();

            export_table_to_csv(&table, "output.csv").expect("Failed to export table to CSV");
        } else {
            println!("No need for power oversubscription. System power consumption is within system demand.");
        }
    } else {
        println!("No need for power oversubscription. System power consumption is within system demand.");
    }
}

fn sort_users() -> Vec<User> {
    // Sort the users based on resource reduction
    // Replace with actual implementation
    let mut users = Vec::new();

    for i in 1..=1000 {
        let resource_reduction = i % 10;
        let power_consumption = (i % 100) + 10;
        users.push(User::new(i, resource_reduction, power_consumption));
    }

    users.sort_by_key(|user| user.resource_reduction);
    users
}

struct User {
    id: u32,
    resource_reduction: u32,
    power_consumption: u32,
    bid_amount: u32,
}

impl User {
    fn allocate_resource_reduction(&mut self, system_power_consumption: u32, system_demand: u32) -> (ResourceReduction, PowerConsumption, u32, u32) {
        let resource_reduction_limit = (self.power_consumption * system_demand) / system_power_consumption;
        let updated_resource_reduction = if self.resource_reduction > resource_reduction_limit {
            resource_reduction_limit
        } else {
            self.resource_reduction
        };

        let updated_power_consumption = PowerConsumption::new(self.power_consumption + 5);
        let bid_amount = self.generate_bid_amount();
        let power_saved = self.power_consumption - updated_power_consumption.consumption_value;

        self.bid_amount = bid_amount;
        self.resource_reduction = updated_resource_reduction;

        (ResourceReduction::new(updated_resource_reduction), updated_power_consumption, bid_amount, power_saved)
    }
}

struct ResourceReduction {
    reduction_value: u32,
}

impl ResourceReduction {
    fn new(reduction_value: u32) -> Self {
        // Initialize a new ResourceReduction instance
        ResourceReduction {
            reduction_value,
        }
    }
}

struct PowerConsumption {
    consumption_value: u32,
}

impl PowerConsumption {
    fn new(consumption_value: u32) -> Self {
        // Initialize a new PowerConsumption instance
        PowerConsumption {
            consumption_value,
        }
    }
}

fn export_table_to_csv(table: &Table, filename: &str) -> Result<(), csv::Error> {
    // Write the table to a CSV file
    let mut writer = Writer::from_path(filename)?;

    // Write the column headers to the CSV file
    let headers: Vec<String> = table
        .titles()
        .iter()
        .map(|cell| cell.to_string())
        .collect();
    writer.write_record(headers)?;

    // Write the table rows to the CSV file
    for row in table.row_iter() {
        let csv_row: Vec<String> = row
            .iter()
            .map(|cell| cell.to_string())
            .collect();
        writer.write_record(csv_row)?;
    }

    writer.flush()?;
    Ok(())
}

fn calculate_system_power_consumption(
    data_center_capacity: u32,
    servers_count: u32,
    server_power_consumption: u32,
    storage_count: u32,
    storage_power_consumption: u32,
    networking_devices_count: u32,
    networking_device_power_consumption: u32,
    cooling_systems_count: u32,
    cooling_system_power_consumption: u32,
    lighting_power_consumption: u32,
    other_equipment_power_consumption: u32,
) -> u32 {
    // Calculate and return the system power consumption
    let servers_power = servers_count * server_power_consumption;
    let storage_power = storage_count * storage_power_consumption;
    let networking_devices_power = networking_devices_count * networking_device_power_consumption;
    let cooling_systems_power = cooling_systems_count * cooling_system_power_consumption;
    let total_equipment_power = servers_power + storage_power + networking_devices_power + cooling_systems_power;
    let other_power_consumption = lighting_power_consumption + other_equipment_power_consumption;

    let system_power_consumption = total_equipment_power + other_power_consumption;

    if system_power_consumption > data_center_capacity {
        return data_center_capacity; // Return the maximum power consumption allowed by the data center capacity
    }
    system_power_consumption
}

fn calculate_system_demand(data_center_capacity: u32, servers_count: u32, server_demand: u32, storage_count: u32, storage_demand: u32, networking_devices_count: u32, networking_device_demand: u32, cooling_systems_count: u32, cooling_system_demand: u32, lighting_demand: u32, other_equipment_demand: u32) -> u32 {
    // Calculate and return the system demand based on the provided parameters
    let server_power_demand = servers_count * server_demand;
    let storage_power_demand = storage_count * storage_demand;
    let networking_device_power_demand = networking_devices_count * networking_device_demand;
    let cooling_system_power_demand = cooling_systems_count * cooling_system_demand;
    let total_power_demand = server_power_demand + storage_power_demand + networking_device_power_demand + cooling_system_power_demand + lighting_demand + other_equipment_demand;

    // Adjust the system demand based on the data center capacity
    let adjusted_demand = total_power_demand * data_center_capacity / 100;

    adjusted_demand
}

pub fn handle_power_oversubscription() {
    let system_power_consumption = system_power_consumption::calculate_system_power_consumption(50000, 500, 150, 20, 200, 50, 80, 5, 1000, 200, 500);
    println!("System Power Consumption: {}", system_power_consumption);

    let system_demand = system_demand::calculate_system_demand(90, 50, 150, 10, 100, 20, 80, 3, 500, 200, 300);
    println!("System Demand: {} watts", system_demand);

    if system_power_consumption > system_demand {
        let mut sorted_users = sort_users();

        let total_resource_reduction: u32 = sorted_users.iter().map(|user| user.resource_reduction).sum();
        let total_power_consumption: u32 = sorted_users.iter().map(|user| user.power_consumption).sum();

        if total_power_consumption > system_demand {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row![
                "User ID",
                "Initial Power Consumption",
                "Resource Reduction",
                "Updated Power Consumption",
                "Bid Amount",
                "Power Saved"
            ]);

            let arc_table = Arc::new(Mutex::new(table));
            let arc_congested_users = Arc::new(Mutex::new(Vec::new()));

            let threads: Vec<_> = sorted_users.into_iter().map(|mut user| {
                let arc_table = Arc::clone(&arc_table);
                let arc_congested_users = Arc::clone(&arc_congested_users);
                thread::spawn(move || {
                    let (updated_resource_reduction, updated_power_consumption, bid_amount, power_saved) =
                        user.allocate_resource_reduction(system_power_consumption, system_demand);

                    println!("User {}: Allocating resources...", user.id);
                    println!("User {}: Initial Power Consumption: {} watts", user.id, user.power_consumption);
                    println!("User {}: Resource Reduction: {}%", user.id, updated_resource_reduction.reduction_value);
                    println!("User {}: Updated Power Consumption: {} watts", user.id, updated_power_consumption.consumption_value);
                    println!("User {}: Bid Amount: {}", user.id, bid_amount);
                    println!("User {}: Power Saved: {} watts", user.id, power_saved);
                    println!("User {}: Resource allocation complete.", user.id);

                    let mut table = arc_table.lock().unwrap();
                    table.add_row(row![
                        user.id,
                        user.power_consumption,
                        updated_resource_reduction.reduction_value,
                        updated_power_consumption.consumption_value,
                        bid_amount,
                        power_saved
                    ]);

                    if updated_resource_reduction.reduction_value == 0 {
                        let mut congested_users = arc_congested_users.lock().unwrap();
                        congested_users.push(user.id);
                    }
                })
            }).collect();

            for thread in threads {
                thread.join().unwrap();
            }

            let table = arc_table.lock().unwrap();
            table.printstd();

            let congested_users = arc_congested_users.lock().unwrap();
            if !congested_users.is_empty() {
                println!("Users experiencing resource congestion: {:?}", congested_users);
            } else {
                println!("No users experiencing resource congestion.");
            }

            export_table_to_csv(&table, "output.csv").expect("Failed to export table to CSV");
        } else {
            println!("No need for power oversubscription. System power consumption is within system demand.");
        }
    } else {
        println!("No need for power oversubscription. System power consumption is within system demand.");
    }
}