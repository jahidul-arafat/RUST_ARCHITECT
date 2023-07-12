# Title:

/* Simulation Explanation 
Certainly! Here are the steps explained in bullet points for the example scenario with user 100 and user 200:

1. Calculate the system power consumption and system demand.
    - Assume the system power consumption is 1000 units.
    - Assume the system demand is 800 units.

2. Check if system power consumption is greater than system demand.
    - Since 1000 units is greater than 800 units, power oversubscription is needed.

3. Sort the users based on their resource reduction values.
    - Assume user 100 has a resource reduction value of 0 and a power consumption of 110 units.
    - Assume user 200 has a resource reduction value of 1 and a power consumption of 210 units.
    - Sort the users in ascending order based on resource reduction.

4. Perform the bidding process for each user.
    - For user 100:
        - Calculate the resource reduction limit based on system power consumption and demand.
        - Since the resource reduction limit is 880 units [(110 * 800) / 1000], user 100 can offer a maximum of 0 units as a bid.
        - The updated power consumption for user 100 is 115 units [(110 + 5)].
        - The bid amount for user 100 is 0 units.
    - For user 200:
        - Calculate the resource reduction limit based on system power consumption and demand.
        - Since the resource reduction limit is 168 units [(210 * 800) / 1000], user 200 can offer a maximum of 1 unit as a bid.
        - The updated power consumption for user 200 is 215 units [(210 + 5)].
        - The bid amount for user 200 is 210 units.

5. Display the bidding results in a table.
    - The table shows the user ID, resource reduction, updated power consumption, and bid amount.
    - For user 100, the table displays: User ID: 100, Resource Reduction: 0, Updated Power Consumption: 115, Bid Amount: 0.
    - For user 200, the table displays: User ID: 200, Resource Reduction: 1, Updated Power Consumption: 215, Bid Amount: 210.

6. Perform resource reallocation based on the bids.
    - Calculate the total available resource reduction from all users' bids (in this case, 210 units).
    - Calculate the ratio between system power consumption and total power consumption (in this case, 0.8).
    - For each user:
        - Calculate the allocation based on the bid amount and the ratio (in this case, 0.8 * 210 = 168 units for user 200).
        - Update the resource reduction for each user based on the allocation.

7. Print the updated table after resource reallocation.
    - The table shows the user ID, resource reduction, updated power consumption, and bid amount.
    - For user 100, the table displays: User ID: 100, Resource Reduction: 0, Updated Power Consumption: 115, Bid Amount: 0.
    - For user 200, the table displays: User ID: 200, Resource Reduction: 168, Updated Power Consumption: 215, Bid Amount: 210.

These steps outline the process of power oversubscription for the example scenario with user 100 and user 200. The code implementation handles these steps for a larger number of users, performing the bidding process, resource reallocation, and updating the user's resource reduction accordingly.
*/

/*
Resource Reduction Calculation:

In the allocate_resource_reduction function of the User struct:
Calculate the resource_reduction_limit by multiplying the user's power consumption with the system demand and dividing it by the system power consumption.
If the user's current resource reduction is greater than the resource_reduction_limit, set the updated_resource_reduction to the resource_reduction_limit. Otherwise, keep the user's current resource reduction.
Return the updated_resource_reduction as a ResourceReduction instance.
Updated Power Consumption Calculation:

In the allocate_resource_reduction function of the User struct:
Create a new PowerConsumption instance with the value of the user's current power consumption plus 5.
Return the new PowerConsumption instance as the updated_power_consumption.
Bid Amount Calculation:

In the generate_bid_amount function of the User struct:
Multiply the user's current resource reduction by their power consumption.
Return the result as the bid_amount.
Therefore, the calculations for resource reduction, updated power consumption, and bid amount are as follows:

Resource Reduction Calculation:
updated_resource_reduction = min(user.resource_reduction, (user.power_consumption * system_demand) / system_power_consumption)

Updated Power Consumption Calculation:
updated_power_consumption = PowerConsumption::new(user.power_consumption + 5)

Bid Amount Calculation:
bid_amount = user.resource_reduction * user.power_consumption

These calculations are based on the provided code and the functions implemented within the User struct.
*/