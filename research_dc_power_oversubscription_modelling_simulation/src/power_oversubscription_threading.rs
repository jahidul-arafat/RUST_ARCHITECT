use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use prettytable::{format, row, Table};
use rand::Rng;
use crate::{system_demand, system_power_consumption};

use crate::csv_creator::export_table_to_csv;
use crate::user::User;
use crate::utilities::sort_users;

const MAX_THREADS: usize = 100; // Maximum number of concurrent threads to use

pub fn handle_power_oversubscription() {
    /**
    In summary, system power consumption reflects the actual power usage of the system, while system demand represents the required power level to support the system's operations and workload. The system demand is typically determined based on factors such as performance requirements, resource utilization, and operational considerations, while the system power consumption is measured based on the actual power consumed by the system's components.
     */
    let system_power_consumption = system_power_consumption::calculate_system_power_consumption(rand::thread_rng().gen_range(300000..=500000),
                                                                                                50000,
                                                                                                150,
                                                                                                200,
                                                                                                200,
                                                                                                5000,
                                                                                                80,
                                                                                                5,
                                                                                                1000,
                                                                                                20000,
                                                                                                5000000);
    println!("System Power Consumption (Actual Consumption): {} watts", system_power_consumption);

    let system_demand = system_demand::calculate_system_demand(9000, 500, 150, 10, 100, 20, 80, 3, 500, 200, 300);
    println!("System Demand (Expected to Run Efficiently): {} watts", system_demand);

    // Check if systemPowerConsumption > systemDemand
    if system_power_consumption > system_demand {
        let mut sorted_users = sort_users();

        let total_resource_reduction: u32 = sorted_users.iter().map(|user| user.resource_reduction).sum();
        let total_power_consumption: u32 = sorted_users.iter().map(|user| user.power_consumption).sum();

        println!("Total Resource Reduction: {}\n", total_resource_reduction);
        println!("Total Power Consumption: {}\n", total_power_consumption);

        //if total_power_consumption > system_demand {
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

            // Batch processing
            //let user_batches = sorted_users.chunks(sorted_users.len() / MAX_THREADS).collect::<Vec<_>>();
            // convert the Vec<&[User]> into a Vec<User>
            //let user_chunks: Vec<User> = user_batches.into_iter().flatten().collect();

            let num_users = sorted_users.len();
            let num_threads = MAX_THREADS;
            let batch_size = (num_users + num_threads - 1) / num_threads;

            let user_batches: Vec<Vec<User>> = sorted_users
                .chunks(batch_size)
                .map(|chunk| chunk.to_vec())
                .collect();
            println!("Number of batches: {}", user_batches.len());

            let user_batches: Vec<User> = user_batches.into_iter().flatten().collect();

            // Print the sorted users
            println!("Number of batch Users: {}", user_batches.len());
            // for user in user_batches.iter() {
            //     println!("{:?}", user);
            // }

            let threads: Vec<_> = user_batches
                .into_iter()
                .enumerate()
                .map(|(index,mut user)| {
                    let arc_table = Arc::clone(&arc_table);
                    let arc_congested_users = Arc::clone(&arc_congested_users);
                    thread::spawn(move || {
                        let (updated_resource_reduction,
                            updated_power_consumption,
                            bid_amount,
                            power_saved) =
                            user.allocate_resource_reduction(system_power_consumption, system_demand);

                        println!("User {}: Allocating resources...", user.id);
                        println!("User {}: Initial Power Consumption: {} watts", user.id, user.power_consumption);
                        println!("User {}: Resource Reduction: {}%", user.id, updated_resource_reduction.reduction_value);
                        println!("User {}: Updated Power Consumption: {} watts", user.id, updated_power_consumption.consumption_value);
                        println!("User {}: Bid Amount: {}", user.id, bid_amount);
                        println!("User {}: Power Saved: {} watts", user.id, power_saved);
                        println!("User {}: Resource allocation complete.", user.id);
                        println!();

                        // Simulate some processing time
                        println!("{}",index);
                        thread::sleep(Duration::from_millis(500));
                        //thread::sleep(Duration::from_millis(1000 * 5 as u64));


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
                })
                .collect();

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

            // Export the table to a CSV file
            export_table_to_csv(&table, "output.csv").expect("Failed to export table to CSV");
        // } else {
        //     println!("No need for power oversubscription. System power consumption is within system demand.");
        // }
    } else {
        println!("No need for power oversubscription. System power consumption is within system demand.");
    }

    // Continue with the rest of handlePowerOversubscription logic
}