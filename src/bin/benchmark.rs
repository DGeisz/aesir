use std::time::SystemTime;
use std::rc::Rc;

use aesir::sensor::Sensor;
use aesir::sensor::custom_sensors::ConstantSensor;
use aesir::actuator::Actuator;
use aesir::actuator::custom_actuator::ConstantActuator;
use aesir::reflex::Reflex;
use aesir::neuron::{SynapseType, basic_weight_modifier};
use aesir::ecp_geometry::{EcpBox, EcpGeometry};
use aesir::encephalon::Encephalon;

fn main() {

    let sensor_names = ["1", "2", "3", "4"];

    let mut sensors: Vec<Rc<dyn Sensor>> = Vec::new();

    for name in &sensor_names {
        sensors.push(Rc::new(ConstantSensor::new((*name).into(), 0.5)));
    }

    let actuator_names = ["act1", "act2", "act3"];

    let mut actuators: Vec<Rc<dyn Actuator>> = Vec::new();

    for name in &actuator_names {
        actuators.push(Rc::new(ConstantActuator::new((*name).into())));
    }

    let reflexes = vec![
        Reflex::new(
            "1".into(),
            "act1".into(),
            SynapseType::Excitatory,
            20.,
        ),
        Reflex::new(
            "2".into(),
            "act1".into(),
            SynapseType::Inhibitory,
            20.,
        ),
        Reflex::new(
            "1".into(),
            "act2".into(),
            SynapseType::Excitatory,
            20.,
        ),
        Reflex::new(
            "3".into(),
            "act3".into(),
            SynapseType::Inhibitory,
            20.,
        ),
    ];

    let ecp_g = Box::new(EcpBox::new(10_u32.pow(3) as u32, 3, 4, 215));

    let mut ecp = Encephalon::new(
        ecp_g,
        sensors,
        actuators,
        reflexes,
        10,
        basic_weight_modifier,
        2.,
        10.,
        (2., 5.)
    );


    let mut time_now = SystemTime::now();
    for i in 0..3000 {
        ecp.run_cycle();

        if i % 100 == 0 {
            println!(
                "Cycle: {}, Elapsed time: {}",
                i,
                time_now.elapsed().unwrap().as_secs_f32()
            );
            time_now = SystemTime::now();
        }
    }
}