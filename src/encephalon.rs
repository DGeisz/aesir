use std::collections::HashMap;

use crate::ecp_geometry::EcpGeometry;
use crate::neuron::{PlasticNeuron, ActuatorNeuron, SensoryNeuron};
use crate::neuron_interfaces::{ActuatorInterface, SensoryInterface};
use crate::reflex::Reflex;
use crate::sensor::Sensor;
use crate::actuator::Actuator;
use std::rc::Rc;

pub struct Encephalon<'a> {
    ecp_geometry: &'a dyn EcpGeometry,
    plastic_neurons: HashMap<Vec<i32>, Rc<PlasticNeuron>>,
    actuator_neurons: HashMap<Vec<i32>, ActuatorNeuron>,
    sensory_neurons: HashMap<Vec<i32>, Rc<SensoryNeuron>>,
    actuator_interfaces: HashMap<String, ActuatorInterface<'a>>,
    sensory_interfaces: HashMap<String, SensoryInterface<'a>>,
    reflexes: Vec<Reflex>
}

impl<'a> Encephalon<'a> {
    pub fn new(
        ecp_geometry: &'a dyn EcpGeometry,
        mut sensors: Vec<&'a mut dyn Sensor>,
        actuators: Vec<&'a mut dyn Actuator>,
        reflexes: Vec<Reflex>,

        //Neuron parameters
        charge_bins: u8,
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
        fire_threshold: f32,
    ) -> Encephalon<'a> {
        if ecp_geometry.get_num_sensory() != sensors.len() as u32 {
            panic!(
                "The number of sensors passed to the encephalon doesn't \
             match the number of sensor neuron positions within the specified ecp_geometry"
            );
        } else if ecp_geometry.get_num_actuator() != actuators.len() as u32 {
            panic!(
                "The number of actuators passed to the encephalon doesn't \
             match the number of actuator neuron positions within the specified ecp_geometry"
            );
        }

        let mut encephalon = Encephalon {
            ecp_geometry,
            plastic_neurons: HashMap::new(),
            actuator_neurons: HashMap::new(),
            sensory_neurons: HashMap::new(),
            actuator_interfaces: HashMap::new(),
            sensory_interfaces: HashMap::new(),
            reflexes
        };

        //Populate plastic neurons
        let mut plastic_loc_option = Some(encephalon.ecp_geometry.first_plastic_loc());
        loop {
            if let Some(loc) = &plastic_loc_option {
                encephalon.plastic_neurons.insert(loc.clone(), Rc::new(PlasticNeuron::new(
                    charge_bins,
                    weight_modifier,
                    fire_threshold
                )));
                plastic_loc_option = encephalon.ecp_geometry.next_plastic_loc(loc.clone());
            } else {
                break;
            }
        }

        let mut sensory_loc_option = Some(encephalon.ecp_geometry.first_sensory_loc());
        loop {
            if let Some(loc) = &sensory_loc_option {

                let neuron = Rc::new(SensoryNeuron::new(weight_modifier));

                encephalon.sensory_neurons.insert(loc.clone(), Rc::clone(&neuron));

                let sensor = sensors.pop().unwrap();
                encephalon.sensory_interfaces.insert(
                    sensor.get_name(),
                    SensoryInterface::new(
                        sensor,
                        Rc::clone(&neuron)
                    ));

                sensory_loc_option = encephalon.ecp_geometry.next_sensory_loc(loc.clone());
            } else {
                break;
            }
        }










        encephalon
    }
}