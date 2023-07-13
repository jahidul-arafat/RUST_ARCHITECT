pub(crate) struct PowerConsumption {
    pub consumption_value: u32,
}

impl PowerConsumption {
    pub fn new(consumption_value: u32) -> Self {
        // Initialize a new PowerConsumption instance
        PowerConsumption {
            consumption_value,
        }
    }
}