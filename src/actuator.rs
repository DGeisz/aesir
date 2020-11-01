pub trait Actuator {
    fn set_control_value(&self, value: f32);
    fn get_name(&self) -> String;
}

pub mod custom_actuator {
    use crate::actuator::Actuator;
    use std::cell::RefCell;

    /// Literally just takes and stores a value
    pub struct BasicActuator {
        name: String,
        measure: RefCell<f32>,
    }

    impl BasicActuator {
        pub fn new(name: String) -> BasicActuator {
            BasicActuator {
                name,
                measure: RefCell::new(0.),
            }
        }
    }

    impl Actuator for BasicActuator {
        fn set_control_value(&self, value: f32) {
            *self.measure.borrow_mut() = value;
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }
}
