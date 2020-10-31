use std::rc::Rc;
use std::boxed::Box;

use crate::actuator::Actuator;
use crate::neuron::ActuatorNeuron;
use crate::neuron::SensoryNeuron;
use crate::sensor::Sensor;

pub struct SensoryInterface {
    sensor: Box<dyn Sensor>,
    pub sensory_neuron: Rc<SensoryNeuron>,
}

impl SensoryInterface {
    pub fn new(
        sensor: Box<dyn Sensor>,
        sensory_neuron: Rc<SensoryNeuron>,
    ) -> SensoryInterface {
        SensoryInterface {
            sensor,
            sensory_neuron,
        }
    }

    pub fn run_cycle(&self) {
        self.sensory_neuron.set_measure(self.sensor.measure());
    }
}

pub struct ActuatorInterface {
    actuator: Box<dyn Actuator>,
    pub actuator_neuron: Rc<ActuatorNeuron>,
}

impl ActuatorInterface {
    pub fn new(
        actuator: Box<dyn Actuator>,
        actuator_neuron: Rc<ActuatorNeuron>,
    ) -> ActuatorInterface {
        ActuatorInterface {
            actuator,
            actuator_neuron,
        }
    }

    pub fn run_cycle(&self) {
        self.actuator
            .set_control_value(self.actuator_neuron.read_measure());
    }
}
