pub trait Actuator {
    fn set_control_value(&self, value: f32);
    fn get_name(&self) -> String;
}
