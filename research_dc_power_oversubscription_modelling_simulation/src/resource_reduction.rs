pub(crate) struct ResourceReduction {
    pub reduction_value: u32,
}

impl ResourceReduction {
    pub(crate) fn new(reduction_value: u32) -> Self {
        // Initialize a new ResourceReduction instance
        ResourceReduction {
            reduction_value,
        }
    }
}