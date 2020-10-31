pub trait Actuator {
    fn set_control_value(&self, value: f32);
    fn get_name(&self) -> String;
}

pub mod custom_actuator {
    use std::cell::RefCell;
    use crate::actuator::Actuator;

    pub struct ConstantActuator {
        name: String,
        measure: RefCell<f32>
    }

    impl ConstantActuator {
        pub fn new(name: String) -> ConstantActuator {
            ConstantActuator {
                name,
                measure: RefCell::new(0.)
            }
        }
    }

    impl Actuator for ConstantActuator {
        fn set_control_value(&self, value: f32) {
            *self.measure.borrow_mut() = value;
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }
}