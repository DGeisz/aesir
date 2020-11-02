use aesir::actuator::custom_actuator::BasicActuator;
use aesir::actuator::Actuator;
use aesir::ecp_geometry::{EcpBox, EcpGeometry};
use aesir::encephalon::Encephalon;
use aesir::neuron::{basic_weight_modifier, SynapseType};
use aesir::reflex::Reflex;
use aesir::sensor::custom_sensors::BasicSensor;
use aesir::sensor::Sensor;
use mnist::{Mnist, MnistBuilder};
use std::rc::Rc;

const MNIST_SIDE_LENGTH: u32 = 28;
const MNIST_AREA: u32 = MNIST_SIDE_LENGTH * MNIST_SIDE_LENGTH;

const SYNAPTIC_TYPE_RATIO: f32 = 2.;
const SYNAPTIC_WEIGHT_RANGE: (f32, f32) = (2., 5.);
const FIRE_THRESHOLD: f32 = 10.;

const NEARBY_COUNT: u32 = 26;
const NUM_PLASTIC: u32 = 216;

const REFLEX_SENSOR_WEIGHT: f32 = 20.;

fn main() {
    // Load mnist
    let trn_size = 50_000;

    let Mnist {
        trn_img, trn_lbl, ..
    } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();

    let img_start_index = |img_index: u32| MNIST_SIDE_LENGTH * MNIST_SIDE_LENGTH * img_index;

    // Create actuators
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

    // Create img sensors
    let mut mnist_sensors: Vec<Rc<BasicSensor>> = Vec::new();
    let mut mnist_sensors_copy: Vec<Rc<dyn Sensor>> = Vec::new();

    let mut sens_x = 0;
    let mut sens_y = 0;

    loop {
        if sens_y >= MNIST_SIDE_LENGTH {
            break;
        }

        let name = format!("{}:{}", sens_x, sens_y);
        let sensor = Rc::new(BasicSensor::new(name));

        mnist_sensors.push(Rc::clone(&sensor));
        mnist_sensors_copy.push(Rc::clone(&sensor) as Rc<dyn Sensor>);

        if sens_x >= MNIST_SIDE_LENGTH - 1 {
            sens_y += 1;
            sens_x = 0;
        } else {
            sens_x += 1;
        }
    }

    // Create reflex sensors
    let mut reflex_sensors: Vec<Rc<BasicSensor>> = Vec::new();

    for sensor_index in 0..10 {
        reflex_sensors.push(Rc::new(BasicSensor::new(format!("{}", sensor_index))));
    }

    // Create ecp geometry
    let ecp_g = Box::new(EcpBox::new(NUM_PLASTIC, 10, MNIST_AREA, NEARBY_COUNT));

    // Create Encephalon
    let mut ecp = Encephalon::new(
        ecp_g,
        mnist_sensors_copy,
        actuator_copy,
        Vec::new(),
        10,
        basic_weight_modifier,
        SYNAPTIC_TYPE_RATIO,
        FIRE_THRESHOLD,
        SYNAPTIC_WEIGHT_RANGE,
    );

    // Add reflex sensors
    for (sensor_index, reflex_sensor) in reflex_sensors.iter().enumerate() {
        let mut reflexes: Vec<Reflex> = Vec::new();

        for i in 0..10 {

            let synapse_type = if i == sensor_index {
                SynapseType::Excitatory
            } else {
                SynapseType::Inhibitory
            };

            reflexes.push(Reflex::new(
                format!("{}", sensor_index),
                format!("{}", i),
                synapse_type,
                REFLEX_SENSOR_WEIGHT,
            ));
        }

        ecp.add_reflex_sensor(
            Rc::clone(reflex_sensor) as Rc<dyn Sensor>,
            sensor_index as i32,
            reflexes,
            basic_weight_modifier,
        );
    }

    let load_mnist_img = |img_index: u32| {
        let img_index = img_start_index(img_index);

        for sensor_index in 0..MNIST_SIDE_LENGTH {
            mnist_sensors
                .get(sensor_index as usize)
                .unwrap()
                .set_measure(
                    *trn_img.get((img_index + sensor_index) as usize).unwrap() as f32 / 255.,
                );
        }

        
    };
}
