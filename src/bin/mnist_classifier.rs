use aesir::actuator::custom_actuator::BasicActuator;
use aesir::actuator::Actuator;
use aesir::sensor::custom_sensors::BasicSensor;
use aesir::sensor::Sensor;
use std::rc::Rc;

const MNIST_SIDE_LENGTH: u8 = 28;

fn main() {
    let actuator_names = (0..10)
        .collect::<Vec<u8>>()
        .iter()
        .map(|num| format!("{}", num))
        .collect::<Vec<String>>();

    let mut actuators: Vec<Rc<dyn Actuator>> = Vec::new();
    let mut actuator_copy: Vec<Rc<dyn Actuator>> = Vec::new();

    for name in &actuator_names {
        let actuator = Rc::new(BasicActuator::new(name.clone()));

        actuators.push(Rc::clone(&actuator) as Rc<dyn Actuator>);
        actuator_copy.push(Rc::clone(&actuator) as Rc<dyn Actuator>);
    }

    let mut mnist_sensors: Vec<Rc<dyn Sensor>> = Vec::new();
    let mut mnist_sensors_copy: Vec<Rc<dyn Sensor>> = Vec::new();

    let mut sens_x = 0;
    let mut sens_y = 0;

    loop {
        if sens_y >= MNIST_SIDE_LENGTH {
            break;
        }

        let name = format!("{}:{}", sens_x, sens_y);
        let sensor = Rc::new(BasicSensor::new(name));

        mnist_sensors.push(Rc::clone(&sensor) as Rc<dyn Sensor>);
        mnist_sensors_copy.push(Rc::clone(&sensor) as Rc<dyn Sensor>);

        if sens_x >= MNIST_SIDE_LENGTH - 1 {
            sens_y += 1;
            sens_x = 0;
        } else {
            sens_x += 1;
        }
    }


}
