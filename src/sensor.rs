pub trait Sensor {
    /// Must return a value between 0 and 1
    fn measure(&self) -> f32;
    fn get_name(&self) -> String;
}

pub mod custom_sensors {
    use crate::sensor::Sensor;
    use std::cell::RefCell;

    /// Sensor with a constant value
    pub struct ConstantSensor {
        name: String,
        measure: f32,
    }

    impl ConstantSensor {
        pub fn new(name: String, measure: f32) -> ConstantSensor {
            ConstantSensor { name, measure }
        }
    }

    impl Sensor for ConstantSensor {
        fn measure(&self) -> f32 {
            self.measure
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    /// Sensor who's measure is explicitly set
    pub struct BasicSensor {
        name: String,
        measure: RefCell<f32>,
    }

    impl BasicSensor {
        pub fn new(name: String) -> BasicSensor {
            BasicSensor {
                name,
                measure: RefCell::new(0.0),
            }
        }

        pub fn set_measure(&self, measure: f32) {
            *self.measure.borrow_mut() = measure
        }
    }

    impl Sensor for BasicSensor {
        fn measure(&self) -> f32 {
            *self.measure.borrow()
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }
}
