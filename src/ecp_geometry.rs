pub trait EcpGeometry {
    fn new(num_plastic: u32, num_actuator: u32, num_sensory: u32, nearby_count: u32) -> Self
    where
        Self: Sized;

    fn get_num_plastic(&self) -> u32;
    fn get_num_sensory(&self) -> u32;
    fn get_num_actuator(&self) -> u32;

    fn get_nearby_count(&self) -> u32;

    fn first_plastic_loc(&self) -> Vec<i32>;
    fn next_plastic_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;
    fn first_actuator_loc(&self) -> Vec<i32>;
    fn next_actuator_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;
    fn first_sensory_loc(&self) -> Vec<i32>;
    fn next_sensory_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;

    /// First index is plastic neurons, second index is
    /// actuator neurons,
    fn get_nearby_rx_neurons(&self, loc: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>);
}
