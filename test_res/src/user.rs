use rand::Rng;
use crate::resource_reduction::ResourceReduction;
use crate::power_consumption::PowerConsumption;

#[derive(Debug)]
#[derive(Clone)]
pub(crate) struct User {
    pub id: u32,
    pub(crate) resource_reduction: u32,
    pub(crate) power_consumption: u32,
    pub bid_amount: u32,
    pub power_saved: u32,
}

impl User {
    pub(crate) fn new(id: u32, resource_reduction: u32, power_consumption: u32) -> Self {
        // Initialize a new User instance
        User {
            id,
            resource_reduction,
            power_consumption,
            bid_amount: 0,
            power_saved: 0
        }
    }

    pub fn allocate_resource_reduction(&mut self, system_power_consumption: u32, system_demand: u32) -> (ResourceReduction, PowerConsumption, u32, u32) {
        // Perform resource allocation logic for the user
        // Return the updated resource reduction, power consumption, and bid amount

        // coefficients for the models
        let a = 0.5;
        let b = 0.3;
        let c = 0.2;
        let d = 10.0;
        let e = 0.4;
        let f = 0.6;
        let g = 15.0;
        let h = 0.7;
        let i = 0.3;
        let j = 20.0;

        let resource_reduction_limit = (self.power_consumption * system_demand) / system_power_consumption;

        // Generate random values for updated_resource_reduction and updated_power_consumption
        let updated_resource_reduction = ((a * (self.resource_reduction as f64) + b * (self.power_consumption as f64) + c) * rand::thread_rng().gen_range(0..=resource_reduction_limit as f64)) as u32;

        let power_change_factor = if rand::random::<bool>() {
            // Increase power consumption by 1 to 10 units
            ((d * (self.power_consumption as f64) + e * (self.resource_reduction as f64) + f) * rand::thread_rng().gen_range(1..=10) as f64) as i32
        } else {
            // Decrease power consumption by 1 to 10 units
            -((d * (self.power_consumption as f64) + e * (self.resource_reduction as f64) + f) * rand::thread_rng().gen_range(1..=10) as f64) as i32
        };

        let updated_power_consumption = PowerConsumption::new((self.power_consumption as i32 + power_change_factor).max(0) as u32);

        let bid_amount = ((g * (self.power_consumption as f64) + h * (self.resource_reduction as f64) + i) * self.generate_bid_amount()) as u32; // Generate the bid amount

        let power_saved = if self.power_consumption > updated_power_consumption.consumption_value {
            self.power_consumption - updated_power_consumption.consumption_value
        } else {
            // Handle the overflow condition, e.g., return a default value
            0
        };

        self.bid_amount = bid_amount; // Assign the bid amount to the user
        self.power_saved = power_saved; // Assign the power saved to the user

        (ResourceReduction::new(updated_resource_reduction), updated_power_consumption, bid_amount, power_saved)
    }

    pub fn generate_bid_amount(&self) -> u32 {
        // Generate and return the bid amount
        // Replace with actual implementation
        self.resource_reduction * self.power_consumption
    }

    pub fn update_resource_reduction(&mut self, allocation: u32) {
        // Update the resource reduction based on the allocation
        self.resource_reduction = allocation;
    }
}
