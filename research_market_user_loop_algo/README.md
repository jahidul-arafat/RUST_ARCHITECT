# Title:

# 1. Introduction
### 1.1 Prelure
In a data center, power is consumed by various components such as servers, cooling systems, 
lighting, and so on. Resource reduction in this context could refer to any measures taken 
to decrease the power consumption of these components. 
This might include more efficient server utilization, improved cooling systems, or switching to energy-saving lighting.

The "Bid Amount" might be related to a system where users (in this case, the components or departments of the data center) 
bid for a certain amount of power allocation based on their needs. 
The bidding system could be designed in such a way that those who save more power (i.e., have a higher resource reduction) 
can bid more. This could be motivated by a variety of reasons - for example, users who save more power might have more budget
available for bidding due to cost savings from reduced power consumption, 
or the system might reward efficient power use by allowing these users to bid more.

The strong positive correlation (i.e. 0.767560) between Resource Reduction and Bid Amount suggests 
that components or departments of the data center that are more efficient in reducing their power consumption 
tend to bid higher amounts for power allocation. 
This could indicate that the power saving measures are effective, 
as they free up resources that can then be used elsewhere in the data center. 
This could ultimately lead to overall cost savings for the data center, and 
possibly improved performance due to the optimized allocation of power resources.

### 1.2 Simulation with two Users (** in this case the components or departments of the data center) 
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


### 1.3 Formulas 
#### 1.3.1 Resource Reduction Calculation
```agsl
updated_resource_reduction = min(user.resource_reduction, (user.power_consumption * system_demand) / system_power_consumption)

```
- [x] In the allocate_resource_reduction function of the User struct:
- [x] Calculate the resource_reduction_limit by multiplying the user's power consumption with the system demand and dividing it by the system power consumption.
- [x] If the user's current resource reduction is greater than the resource_reduction_limit, set the updated_resource_reduction to the resource_reduction_limit. Otherwise, keep the user's current resource reduction.
- [x] Return the updated_resource_reduction as a ResourceReduction instance.

#### 1.3.2 Updated Power Consumption Calculation
```agsl
updated_power_consumption = PowerConsumption::new(user.power_consumption + 5)
```
In the allocate_resource_reduction function of the User struct:
Create a new PowerConsumption instance with the value of the user's current power consumption plus 5.
Return the new PowerConsumption instance as the updated_power_consumption.
Bid Amount Calculation:

#### 1.3.3 Bid Amount Calculation
```agsl
bid_amount = user.resource_reduction * user.power_consumption
```
In the generate_bid_amount function of the User struct:
Multiply the user's current resource reduction by their power consumption.
Return the result as the bid_amount.
Therefore, the calculations for resource reduction, updated power consumption, and bid amount are as follows:

These calculations are based on the provided code and the functions implemented within the User struct.

## 2. Algorithm and Implementation
### 2.1 Psuedo Algorithm

### 2.2 Sequence Diagram of the Operation

### 2.3 Implementation

## 3. Data Introduction, Representation and Analysis

\exec(csv2md output.csv)
### 3.1 Data Introduction and Representation
