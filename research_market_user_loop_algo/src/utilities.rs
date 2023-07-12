use rand::Rng;
use crate::user::User;

pub(crate) fn sort_users() -> Vec<User> {
    // Sort the users based on resource reduction
    // Replace with actual implementation
    let mut users = Vec::new();

    for i in 1..=rand::thread_rng().gen_range(1000..=5000) {
        let resource_reduction = i % 10;
        let power_consumption = (i % 100) + 10;
        //println!("Resource reduction: {}, power consumption: {}", resource_reduction, power_consumption);
        users.push(User::new(i, resource_reduction, power_consumption));
    }

    users.sort_by_key(|user| user.resource_reduction);
    users
}