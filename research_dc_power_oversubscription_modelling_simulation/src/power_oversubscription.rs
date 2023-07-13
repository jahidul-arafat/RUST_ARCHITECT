// fn handle_power_oversubscription() {
//     let system_power_consumption = calculate_system_power_consumption();
//     let system_demand = calculate_system_demand();
//
//     // Check if systemPowerConsumption > systemDemand
//     if system_power_consumption > system_demand {
//         let mut sorted_users = sort_users();
//
//         let total_resource_reduction: u32 = sorted_users.iter().map(|user| user.resource_reduction).sum();
//         let total_power_consumption: u32 = sorted_users.iter().map(|user| user.power_consumption).sum();
//
//         if total_power_consumption > system_demand {
//             let mut table = Table::new();
//             table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
//             table.set_titles(row!["User ID", "Initial Power Consumption", "Resource Reduction", "Updated Power Consumption", "Bid Amount"]);
//
//             // Bidding Process
//             for user in &mut sorted_users {
//                 let (updated_resource_reduction, updated_power_consumption, bid_amount) = user.allocate_resource_reduction(system_power_consumption, system_demand);
//                 table.add_row(row![user.id, user.power_consumption, updated_resource_reduction.reduction_value, updated_power_consumption.consumption_value, bid_amount]);
//             }
//
//             // Resource Reallocation
//             let total_available_resource_reduction: u32 = sorted_users.iter().map(|user| user.bid_amount).sum();
//             let ratio = system_power_consumption as f64 / total_power_consumption as f64;
//
//             for user in &mut sorted_users {
//                 let allocation = (user.bid_amount as f64 * ratio).round() as u32;
//                 user.update_resource_reduction(allocation);
//             }
//
//             table.printstd();
//
//             // Export the table to a CSV file
//             export_table_to_csv(&table, "output.csv").expect("Failed to export table to CSV");
//         } else {
//             println!("No need for power oversubscription. System power consumption is within system demand.");
//         }
//     } else {
//         println!("No need for power oversubscription. System power consumption is within system demand.");
//     }
//
//     // Continue with the rest of handlePowerOversubscription logic
// }