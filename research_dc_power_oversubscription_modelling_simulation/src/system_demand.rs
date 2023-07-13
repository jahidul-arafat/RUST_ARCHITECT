/*
System Demand
System Demand:
System demand, on the other hand, refers to the power required by the system to operate effectively
and meet its operational requirements.
It represents the desired or expected power level needed to support the system's workload and provide sufficient resources.
System demand considers factors such as the processing requirements, data storage needs, network traffic, cooling requirements,
and other operational considerations. It is also measured in watts (W) or kilowatts (kW).
 */
/*
Certainly! Here's a detailed implementation of the `calculate_system_demand` function with additional parameters:

In this implementation, the `calculate_system_demand` function takes the following additional parameters:
- `data_center_capacity` - The capacity of the data center in percentage (e.g., 100 for full capacity, 80 for 80% capacity)
- `servers_count` - The number of servers in the data center
- `server_demand` - The power demand of each server
- `storage_count` - The number of storage devices in the data center
- `storage_demand` - The power demand of each storage device
- `networking_devices_count` - The number of networking devices in the data center
- `networking_device_demand` - The power demand of each networking device
- `cooling_systems_count` - The number of cooling systems in the data center
- `cooling_system_demand` - The power demand of each cooling system
- `lighting_demand` - The power demand for lighting in the data center
- `other_equipment_demand` - The power demand for other equipment in the data center

The function calculates the total power demand based on the individual power demands of servers, storage devices, networking devices, cooling systems, lighting, and other equipment. It then adjusts the demand based on the data center capacity to provide the final system demand.

You can call the function with specific values like this:
```rust
fn main() {
    let system_demand = calculate_system_demand(90, 50, 150, 10, 100, 20, 80, 3, 500, 200, 300);
    println!("System Demand: {} watts", system_demand);
}
```

In this example, the function is called with the provided parameter values, and the system demand is printed as the output.
 */
pub(crate) fn calculate_system_demand(data_center_capacity: u32, servers_count: u32, server_demand: u32, storage_count: u32, storage_demand: u32, networking_devices_count: u32, networking_device_demand: u32, cooling_systems_count: u32, cooling_system_demand: u32, lighting_demand: u32, other_equipment_demand: u32) -> u32 {
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