use indexmap::IndexMap;
use rand::Rng;

use crate::actuator::Actuator;
use crate::ecp_geometry::EcpGeometry;
use crate::neuron::{
    ActuatorNeuron, ChargeCycle, Neuronic, PlasticNeuron, RxNeuronic, SensoryNeuron, SynapseType,
    TxNeuronic,
};
use crate::neuron_interfaces::{ActuatorInterface, SensoryInterface};
use crate::reflex::Reflex;
use crate::sensor::Sensor;
use std::rc::Rc;

pub struct Encephalon {
    ecp_geometry: Box<dyn EcpGeometry>,
    plastic_neurons: IndexMap<Vec<i32>, Rc<PlasticNeuron>>,
    actuator_neurons: IndexMap<Vec<i32>, Rc<ActuatorNeuron>>,
    sensory_neurons: IndexMap<Vec<i32>, Rc<SensoryNeuron>>,
    actuator_interfaces: IndexMap<String, ActuatorInterface>,
    sensory_interfaces: IndexMap<String, SensoryInterface>,
    reflexes: Vec<Reflex>,
    cycle: ChargeCycle,
}

impl Encephalon {
    pub fn new(
        ecp_geometry: Box<dyn EcpGeometry>,
        mut sensors: Vec<Box<dyn Sensor>>,
        mut actuators: Vec<Box<dyn Actuator>>,
        reflexes: Vec<Reflex>,

        //Neuron parameters
        charge_bins: u8,
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
        synaptic_type_ratio: f32, //Ratio of excitatory to inhibitory synapses
        fire_threshold: f32,
        synapse_weight_ranges: (f32, f32),
    ) -> Encephalon {
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
            plastic_neurons: IndexMap::new(),
            actuator_neurons: IndexMap::new(),
            sensory_neurons: IndexMap::new(),
            actuator_interfaces: IndexMap::new(),
            sensory_interfaces: IndexMap::new(),
            reflexes,
            cycle: ChargeCycle::Odd,
        };

        //Populate plastic neurons
        let mut plastic_loc_option = Some(encephalon.ecp_geometry.first_plastic_loc());
        loop {
            if let Some(loc) = &plastic_loc_option {
                encephalon.plastic_neurons.insert(
                    loc.clone(),
                    Rc::new(PlasticNeuron::new(
                        charge_bins,
                        weight_modifier,
                        fire_threshold,
                    )),
                );
                plastic_loc_option = encephalon.ecp_geometry.next_plastic_loc(loc);
            } else {
                break;
            }
        }

        //Populate sensory neurons
        let mut sensory_loc_option = Some(encephalon.ecp_geometry.first_sensory_loc());
        loop {
            if let Some(loc) = &sensory_loc_option {
                let neuron = Rc::new(SensoryNeuron::new(weight_modifier));

                encephalon
                    .sensory_neurons
                    .insert(loc.clone(), Rc::clone(&neuron));

                let sensor = sensors.pop().unwrap();
                encephalon.sensory_interfaces.insert(
                    sensor.get_name(),
                    SensoryInterface::new(sensor, Rc::clone(&neuron)),
                );

                sensory_loc_option = encephalon.ecp_geometry.next_sensory_loc(loc);
            } else {
                break;
            }
        }

        //Populate actuator neurons
        let mut actuator_loc_option = Some(encephalon.ecp_geometry.first_actuator_loc());
        loop {
            if let Some(loc) = &actuator_loc_option {
                let neuron = Rc::new(ActuatorNeuron::new(charge_bins, fire_threshold));

                encephalon
                    .actuator_neurons
                    .insert(loc.clone(), Rc::clone(&neuron));

                let actuator = actuators.pop().unwrap();
                encephalon.actuator_interfaces.insert(
                    actuator.get_name(),
                    ActuatorInterface::new(actuator, Rc::clone(&neuron)),
                );

                actuator_loc_option = encephalon.ecp_geometry.next_actuator_loc(loc);
            } else {
                break;
            }
        }

        //Closure to generate synaptic weight
        let gen_weight =
            || rand::thread_rng().gen_range(synapse_weight_ranges.0, synapse_weight_ranges.1);

        let gen_synapse_type = || {
            let type_threshold = synaptic_type_ratio / (synaptic_type_ratio + 1.);
            let val = rand::thread_rng().gen_range(0.0, 1.0);

            if val > type_threshold {
                SynapseType::Excitatory
            } else {
                SynapseType::Inhibitory
            }
        };

        //Make synapses for plastic neurons
        for (loc, p_neuron) in encephalon.plastic_neurons.iter() {
            let (plastic_locs, actuator_locs) = encephalon.ecp_geometry.get_nearby_rx_neurons(loc);

            for plastic_loc in plastic_locs.iter() {
                p_neuron.add_plastic_synapse(
                    gen_weight(),
                    gen_synapse_type(),
                    Rc::clone(encephalon.plastic_neurons.get(plastic_loc).unwrap())
                        as Rc<dyn RxNeuronic>,
                )
            }

            for actuator_loc in actuator_locs.iter() {
                p_neuron.add_plastic_synapse(
                    gen_weight(),
                    gen_synapse_type(),
                    Rc::clone(encephalon.actuator_neurons.get(actuator_loc).unwrap())
                        as Rc<dyn RxNeuronic>,
                )
            }
        }

        //Make synapses for sensory neurons
        for (loc, s_neuron) in encephalon.sensory_neurons.iter() {
            let (plastic_locs, actuator_locs) = encephalon.ecp_geometry.get_nearby_rx_neurons(loc);

            for plastic_loc in plastic_locs.iter() {
                s_neuron.add_plastic_synapse(
                    gen_weight(),
                    gen_synapse_type(),
                    Rc::clone(encephalon.plastic_neurons.get(plastic_loc).unwrap())
                        as Rc<dyn RxNeuronic>,
                )
            }

            for actuator_loc in actuator_locs.iter() {
                s_neuron.add_plastic_synapse(
                    gen_weight(),
                    gen_synapse_type(),
                    Rc::clone(encephalon.actuator_neurons.get(actuator_loc).unwrap())
                        as Rc<dyn RxNeuronic>,
                )
            }
        }

        //Make reflexes
        for reflex in encephalon.reflexes.iter() {
            let sensory_interface = encephalon
                .sensory_interfaces
                .get(&reflex.sensor_name)
                .unwrap();
            let actuator_interface = encephalon
                .actuator_interfaces
                .get(&reflex.actuator_name)
                .unwrap();

            sensory_interface.sensory_neuron.add_static_synapse(
                reflex.weight,
                reflex.synapse_type,
                Rc::clone(&actuator_interface.actuator_neuron) as Rc<dyn RxNeuronic>,
            );
        }

        encephalon
    }

    pub fn run_cycle(&mut self) {
        self.cycle = self.cycle.next_cycle();

        for sensory_interface in self.sensory_interfaces.values() {
            sensory_interface.run_cycle();
        }

        for actuator_interface in self.actuator_interfaces.values() {
            actuator_interface.run_cycle();
        }

        for sensory_neuron in self.sensory_neurons.values() {
            sensory_neuron.run_cycle(self.cycle);
        }

        for plastic_neuron in self.plastic_neurons.values() {
            plastic_neuron.run_cycle(self.cycle);
        }

        for actuator_neuron in self.actuator_neurons.values() {
            actuator_neuron.run_cycle(self.cycle);
        }
    }
}

#[cfg(test)]
mod encephalon_tests;