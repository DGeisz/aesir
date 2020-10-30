use std::rc::Rc;

use crate::actuator::Actuator;
use crate::neuron::ActuatorNeuron;
use crate::neuron::SensoryNeuron;
use crate::sensor::Sensor;

pub struct SensoryInterface<'a> {
    sensor: &'a mut dyn Sensor,
    sensory_neuron: Rc<SensoryNeuron>
}

impl<'a> SensoryInterface<'a> {
    pub fn new(sensor: &'a mut dyn Sensor, sensory_neuron: Rc<SensoryNeuron>) -> SensoryInterface<'a> {
        SensoryInterface {
            sensor,
            sensory_neuron,
        }
    }

    pub fn run_cycle(&mut self) {
        self.sensory_neuron.set_measure(self.sensor.measure());
    }
}

pub struct ActuatorInterface<'a> {
    actuator: &'a dyn Actuator,
    actuator_neuron: Rc<ActuatorNeuron>,
}

impl<'a> ActuatorInterface<'a> {
    pub fn new(actuator: &'a dyn Actuator, actuator_neuron: Rc<ActuatorNeuron>,
) -> ActuatorInterface<'a> {
        ActuatorInterface {
            actuator,
            actuator_neuron
        }
    }

    pub fn run_cycle(&self) {
        self.actuator.set_control_value(self.actuator_neuron.read_measure());
    }
}
