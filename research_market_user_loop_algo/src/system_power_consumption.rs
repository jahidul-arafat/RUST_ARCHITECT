/*
As a data center implementation specialist, I understand the importance of considering various parameters to accurately calculate system power consumption. Here's an enhanced version of the `calculate_system_power_consumption` function with an extensive set of parameters:
In this implementation, the function takes into account the following parameters:

- `data_center_capacity`: The total power capacity of the data center.
- `servers_count`: The number of servers in the data center.
- `server_power_consumption`: The power consumption of each server.
- `storage_count`: The number of storage units in the data center.
- `storage_power_consumption`: The power consumption of each storage unit.
- `networking_devices_count`: The number of networking devices (routers, switches, etc.) in the data center.
- `networking_device_power_consumption`: The power consumption of each networking device.
- `cooling_systems_count`: The number of cooling systems (air conditioners, cooling towers, etc.) in the data center.
- `cooling_system_power_consumption`: The power consumption of each cooling system.
- `lighting_power_consumption`: The power consumption for lighting in the data center.
- `other_equipment_power_consumption`: The power consumption of other equipment not covered by the above parameters.

The function calculates the power consumption of different equipment categories and sums them up to obtain the total
 system power consumption. If the calculated power consumption exceeds the data center capacity,
 the function returns the maximum allowed power consumption specified by the data center capacity.
 Otherwise, it returns the calculated power consumption.

Feel free to adjust the parameter names and values to match your specific implementation and requirements.


let data_center_capacity = 5000; // Total power capacity of the data center
    let servers_count = 100; // Number of servers in the data center
    let server_power_consumption = 150; // Power consumption of each server in watts
    let storage_count = 20; // Number of storage units in the data center
    let storage_power_consumption = 200; // Power consumption of each storage unit in watts
    let networking_devices_count = 50; // Number of networking devices in the data center
    let networking_device_power_consumption = 80; // Power consumption of each networking device in watts
    let cooling_systems_count = 5; // Number of cooling systems in the data center
    let cooling_system_power_consumption = 1000; // Power consumption of each cooling system in watts
    let lighting_power_consumption = 200; // Power consumption for lighting in the data center in watts
    let other_equipment_power_consumption = 500; // Power consumption of other equipment in the data center in watts

 */

/*
System Power Consumption:
System power consumption refers to the total amount of power consumed by all the components and devices within a system.
It represents the actual power usage of the system at a given time. It is typically measured in watts (W) or kilowatts (kW).
System power consumption takes into account the power consumed by servers, storage devices, networking equipment, cooling systems,
lighting, and other components in the system.
 */
pub(crate) fn calculate_system_power_consumption(
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

    // if system_power_consumption > data_center_capacity {
    //     return data_center_capacity; // Return the maximum power consumption allowed by the data center capacity
    // }
    system_power_consumption
}