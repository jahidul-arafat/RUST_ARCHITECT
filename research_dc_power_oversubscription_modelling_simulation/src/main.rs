mod system_power_consumption;
mod system_demand;
mod user;
mod resource_reduction;
mod csv_creator;
mod power_oversubscription_threading;
mod utilities;
mod power_consumption;

use prettytable::{format, Table};
use prettytable::row;
use csv::Writer;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Main Program
    power_oversubscription_threading::handle_power_oversubscription();
    // Perform other tasks and computations
}
