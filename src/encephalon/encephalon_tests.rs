use crate::actuator::custom_actuator::ConstantActuator;
use crate::actuator::Actuator;
use crate::ecp_geometry::{EcpBox, EcpGeometry};
use crate::encephalon::Encephalon;
use crate::neuron::SynapseType;
use crate::neuron::TxNeuronic;
use crate::reflex::Reflex;
use crate::sensor::custom_sensors::ConstantSensor;
use crate::sensor::Sensor;

fn weight_modifier(target_measure: f32, weight_measure: f32) -> f32 {
    let x = (target_measure - weight_measure).abs();
    let denominator = 1. - (-1.5_f32).exp();

    let numerator = (-15. * x).exp() - (-1.5_f32).exp();

    return numerator / denominator;
}

#[test]
fn basic_encephalon_test() {
    let sensor_names = ["1", "2", "3", "4"];

    let mut sensors: Vec<Box<dyn Sensor>> = Vec::new();

    for name in &sensor_names {
        sensors.push(Box::new(ConstantSensor::new((*name).into(), 0.5)));
    }

    let actuator_names = ["act1", "act2", "act3"];

    let mut actuators: Vec<Box<dyn Actuator>> = Vec::new();

    for name in &actuator_names {
        actuators.push(Box::new(ConstantActuator::new((*name).into())));
    }

    let reflexes = vec![
        Reflex::new("1".into(), "act1".into(), SynapseType::Excitatory, 20.),
        Reflex::new("2".into(), "act1".into(), SynapseType::Inhibitory, 20.),
        Reflex::new("1".into(), "act2".into(), SynapseType::Excitatory, 20.),
        Reflex::new("3".into(), "act3".into(), SynapseType::Inhibitory, 20.),
    ];

    let ecp_g = Box::new(EcpBox::new(6_u32.pow(3) as u32, 3, 4, 26));

    let mut ecp = Encephalon::new(
        ecp_g,
        sensors,
        actuators,
        reflexes,
        10,
        weight_modifier,
        2.,
        10.,
        (2., 5.),
    );

    // Check neurons are all making the proper number of connections
    for plastic_neuron in ecp.plastic_neurons.values() {
        assert_eq!(26, plastic_neuron.get_plastic_synapses().len());
    }

    for sensory_neuron in ecp.sensory_neurons.values() {
        assert_eq!(27, sensory_neuron.get_plastic_synapses().len());
    }

    // Check static synapses are made
    assert_eq!(
        ecp.sensory_interfaces
            .get("1".into())
            .unwrap()
            .sensory_neuron
            .get_static_synapses()
            .len(),
        2
    );
    assert_eq!(
        ecp.sensory_interfaces
            .get("2".into())
            .unwrap()
            .sensory_neuron
            .get_static_synapses()
            .len(),
        1
    );
    assert_eq!(
        ecp.sensory_interfaces
            .get("3".into())
            .unwrap()
            .sensory_neuron
            .get_static_synapses()
            .len(),
        1
    );

    for _ in 0..100 {
        ecp.run_cycle();
    }
}
