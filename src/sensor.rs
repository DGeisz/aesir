pub trait Sensor {
    /// Must return a value between 0 and 1
    fn measure(&self) -> f32;
    fn get_name(&self) -> String;
}
