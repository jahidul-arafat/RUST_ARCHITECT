/**
Certainly, the coefficients that are used in the updated code serve as weights or multipliers that adjust the calculations for resource reduction, power consumption, and bid amount. They are derived from the mathematical models we established earlier.

Here's a breakdown of what each coefficient does:

1. `a`, `b`, and `c` are used in the calculation for `updated_resource_reduction`. These coefficients are obtained from the first mathematical model and they represent the weights of the user's original resource reduction (`a`), the user's original power consumption (`b`), and a constant term (`c`). The expression `a*self.resource_reduction + b*self.power_consumption + c` is a linear combination of the user's original resource reduction and power consumption, adjusted by the coefficients.

2. `d`, `e`, and `f` are used in the calculation for `power_change_factor`. These coefficients are obtained from the second mathematical model and they represent the weights of the user's original power consumption (`d`), the user's original resource reduction (`e`), and a constant term (`f`). The expression `d*self.power_consumption + e*self.resource_reduction + f` is a linear combination of the user's original power consumption and resource reduction, adjusted by the coefficients.

3. `g`, `h`, and `i` are used in the calculation for `bid_amount`. These coefficients are obtained from the third mathematical model and they represent the weights of the user's original power consumption (`g`), the user's original resource reduction (`h`), and a constant term (`i`). The expression `g*self.power_consumption + h*self.resource_reduction + i` is a linear combination of the user's original power consumption and resource reduction, adjusted by the coefficients.

The purpose of these coefficients is to model the behavior of the system according to the research objectives. They allow you to adjust the influence of the different factors (resource reduction, power consumption) on the output variables (updated resource reduction, power change factor, and bid amount). By changing these coefficients, you can simulate different scenarios and study how changes in the input factors affect the system's behavior.
*/

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
        /*
        The optimal set of coefficient values depends on the specific data, the cost function, and the constraints of your problem. Without this information, it's not possible to suggest the best values.

        However, I can provide some general guidance. Typically, coefficient values are chosen to minimize some cost function or error measure. For example, if you're trying to minimize power consumption, you might choose coefficients that reduce the power consumption in the model equations.

        To give you an idea, here are three sets of coefficients that could be used as starting points for further optimization:

        1. Set 1: \(a = 0.1, b = 0.2, c = 0.3, d = 0.4, e = 0.5, f = 0.6, g = 0.7, h = 0.8, i = 0.9, j = 1.0\)
        2. Set 2: \(a = 0.5, b = 0.4, c = 0.3, d = 0.2, e = 0.1, f = 0.2, g = 0.3, h = 0.4, i = 0.5, j = 0.6\)
        3. Set 3: \(a = 0.9, b = 0.8, c = 0.7, d = 0.6, e = 0.5, f = 0.4, g = 0.3, h = 0.2, i = 0.1, j = 0.05\)

        These sets are randomly chosen and may not be optimal for your specific problem. They are intended to be used as starting points for further optimization, such as the gradient descent method I described earlier.

        Keep in mind that optimization is a complex process that often requires trial and error and the use of sophisticated algorithms. You might need to adjust these values, choose a different starting point, or use different optimization methods depending on your specific problem and data.
        */

        // coefficients for the models
        let a = 0.5;
        let b = 0.4;
        let c = 0.3;
        let d = 0.2;
        let e = 0.1;
        let f = 0.1;
        let g = 0.2;
        let h = 0.3;
        let i = 0.5;
        let j = 0.52;

        let resource_reduction_limit = (self.power_consumption * system_demand) / system_power_consumption;

        // Generate random values for updated_resource_reduction and updated_power_consumption
        let updated_resource_reduction = ((a * (self.resource_reduction as f64) + b * (self.power_consumption as f64) + c) * (rand::thread_rng().gen_range(0..=resource_reduction_limit) as f64)) as u32;

        let power_change_factor = if rand::random::<bool>() {
            // Increase power consumption by 1 to 10 units
            ((d * (self.power_consumption as f64) + e * (self.resource_reduction as f64) + f) * (rand::thread_rng().gen_range(1..=10) as f64)) as i32
        } else {
            // Decrease power consumption by 1 to 10 units
            -((d * (self.power_consumption as f64) + e * (self.resource_reduction as f64) + f) * (rand::thread_rng().gen_range(1..=10) as f64)) as i32
        };

        let updated_power_consumption = PowerConsumption::new((self.power_consumption as i32 + power_change_factor).max(0) as u32);

        let bid_amount = ((g * (self.power_consumption as f64) + h * (self.resource_reduction as f64) + i) * (self.generate_bid_amount() as f64)) as u32; // Generate the bid amount

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




// Earlier Implementation


// use rand::Rng;
// use crate::resource_reduction::ResourceReduction;
// use crate::power_consumption::PowerConsumption;
//
// #[derive(Debug)]
// #[derive(Clone)]
// pub(crate) struct User {
//     pub id: u32,
//     pub(crate) resource_reduction: u32,
//     pub(crate) power_consumption: u32,
//     pub bid_amount: u32,
//     pub power_saved: u32,
// }
//
// impl User {
//     pub(crate) fn new(id: u32, resource_reduction: u32, power_consumption: u32) -> Self {
//         // Initialize a new User instance
//         User {
//             id,
//             resource_reduction,
//             power_consumption,
//             bid_amount: 0,
//             power_saved: 0
//         }
//     }
//
//     pub fn allocate_resource_reduction(&mut self, system_power_consumption: u32, system_demand: u32) ->
//     (ResourceReduction, PowerConsumption, u32, u32) {
//         // Perform resource allocation logic for the user
//         // Return the updated resource reduction, power consumption, and bid amount
//
//         let resource_reduction_limit = (self.power_consumption * system_demand) / system_power_consumption;
//
//         // Generate random values for updated_resource_reduction and updated_power_consumption
//         let updated_resource_reduction = rand::thread_rng().gen_range(0..=resource_reduction_limit);
//
//         let power_change_factor = if rand::random::<bool>() {
//             // Increase power consumption by 1 to 10 units
//             rand::thread_rng().gen_range(1..=10) as i32
//         } else {
//             // Decrease power consumption by 1 to 10 units
//             -rand::thread_rng().gen_range(1..=10) as i32
//         };
//
//         let updated_power_consumption = PowerConsumption::new(
//             (self.power_consumption as i32 + power_change_factor).max(0) as u32,
//         );
//
//         let bid_amount = self.generate_bid_amount(); // Generate the bid amount
//         //let power_saved = self.power_consumption - updated_power_consumption.consumption_value;
//         let power_saved = if self.power_consumption > updated_power_consumption.consumption_value {
//             self.power_consumption - updated_power_consumption.consumption_value
//         } else {
//             // Handle the overflow condition, e.g., return a default value
//             0
//         };
//
//         //let power_saved = 0;
//
//         self.bid_amount = bid_amount; // Assign the bid amount to the user
//         self.power_saved = power_saved; // Assign the power saved to the user
//
//
//         (ResourceReduction::new(updated_resource_reduction), updated_power_consumption, bid_amount, power_saved)
//     }
//
//
//     pub fn generate_bid_amount(&self) -> u32 {
//         // Generate and return the bid amount
//         // Replace with actual implementation
//         self.resource_reduction * self.power_consumption
//     }
//
//     pub fn update_resource_reduction(&mut self, allocation: u32) {
//         // Update the resource reduction based on the allocation
//         self.resource_reduction = allocation;
//     }
// }