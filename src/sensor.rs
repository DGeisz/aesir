pub trait Sensor {
    /// Must return a value between 0 and 1
    fn measure(&self) -> f32;
    fn get_name(&self) -> String;
}

mod custom_sensors {
    use crate::sensor::Sensor;

    pub struct ConstantSensor {
        name: String,
        measure: f32
    }

    impl ConstantSensor {
        pub fn new(name: String, measure: f32) -> ConstantSensor {
            ConstantSensor {
                name,
                measure
            }
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
}
